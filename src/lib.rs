use chrono::Utc;
use chrono_tz::Tz;
use worker::*;

mod db;
mod github;
mod svg;
mod types;

use svg::card::{render_card, render_error_card};
use svg::leaderboard::render_leaderboard;
use types::UserRecord;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/card/:username", handle_card)
        .get_async("/leaderboard", handle_leaderboard)
        .run(req, env)
        .await
}

async fn handle_card(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let username = match ctx.param("username") {
        Some(u) => u.to_string(),
        None => return svg_response(render_error_card("Missing username")),
    };

    let tz_str = req
        .url()
        .ok()
        .and_then(|u| {
            u.query_pairs()
                .find(|(k, _)| k == "tz")
                .map(|(_, v)| v.to_string())
        })
        .unwrap_or_else(|| "UTC".to_string());

    let tz: Tz = tz_str.parse().unwrap_or(chrono_tz::UTC);

    let db = match ctx.env.d1("DB") {
        Ok(d) => d,
        Err(_) => return svg_response(render_error_card("GitHub unavailable, try again")),
    };

    let token = match ctx.env.secret("GITHUB_TOKEN") {
        Ok(t) => t.to_string(),
        Err(_) => return svg_response(render_error_card("GitHub unavailable, try again")),
    };

    let now_str = Utc::now().to_rfc3339();

    let existing = db::get_user(&db, &username).await.unwrap_or(None);

    let record = if let Some(ref user) = existing {
        let last = chrono::DateTime::parse_from_rfc3339(&user.last_loaded)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(Utc::now());
        let age_secs = (Utc::now() - last).num_seconds();
        let age_days = (Utc::now() - last).num_days();

        if age_days > 30 {
            let _ = db::delete_user(&db, &username).await;
            fetch_and_store(&db, &username, &token, tz, &now_str).await
        } else if age_secs < 3600 {
            Ok(user.clone())
        } else {
            fetch_and_store(&db, &username, &token, tz, &now_str).await
        }
    } else {
        fetch_and_store(&db, &username, &token, tz, &now_str).await
    };

    match record {
        Err(e) if e == "user_not_found" => svg_response(render_error_card("User not found")),
        Err(_) => svg_response(render_error_card("GitHub unavailable, try again")),
        Ok(user) => {
            let rank = db::get_rank(&db, user.streak).await.unwrap_or(1);
            svg_response(render_card(&username, user.streak, user.longest_streak, rank))
        }
    }
}

async fn handle_leaderboard(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = match ctx.env.d1("DB") {
        Ok(d) => d,
        Err(_) => return svg_response(render_error_card("GitHub unavailable, try again")),
    };

    match db::get_leaderboard(&db).await {
        Ok(entries) => svg_response(render_leaderboard(&entries)),
        Err(_) => svg_response(render_error_card("GitHub unavailable, try again")),
    }
}

async fn fetch_and_store(
    db: &D1Database,
    username: &str,
    token: &str,
    tz: Tz,
    now_str: &str,
) -> std::result::Result<UserRecord, String> {
    let days = github::fetch_contributions(username, token).await?;
    let result = github::compute_streak(&days, tz);

    let record = UserRecord {
        username: username.to_string(),
        streak: result.streak,
        longest_streak: result.longest_streak,
        last_loaded: now_str.to_string(),
    };

    db::upsert_user(db, &record).await?;
    Ok(record)
}

fn svg_response(svg: String) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Content-Type", "image/svg+xml")?;
    headers.set("Cache-Control", "public, max-age=3600")?;
    Ok(Response::ok(svg)?.with_headers(headers))
}
