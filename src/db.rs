use worker::D1Database;
use crate::types::{LeaderboardEntry, UserRecord};

pub async fn get_user(db: &D1Database, username: &str) -> Result<Option<UserRecord>, String> {
    let stmt = db
        .prepare("SELECT username, streak, longest_streak, last_loaded FROM users WHERE username = ?1")
        .bind(&[username.into()])
        .map_err(|e| e.to_string())?;

    let result = stmt.first::<UserRecord>(None).await.map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn upsert_user(db: &D1Database, record: &UserRecord) -> Result<(), String> {
    db.prepare(
        "INSERT INTO users (username, streak, longest_streak, last_loaded)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(username) DO UPDATE SET
           streak = excluded.streak,
           longest_streak = excluded.longest_streak,
           last_loaded = excluded.last_loaded",
    )
    .bind(&[
        record.username.as_str().into(),
        record.streak.into(),
        record.longest_streak.into(),
        record.last_loaded.as_str().into(),
    ])
    .map_err(|e| e.to_string())?
    .run()
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn delete_user(db: &D1Database, username: &str) -> Result<(), String> {
    db.prepare("DELETE FROM users WHERE username = ?1")
        .bind(&[username.into()])
        .map_err(|e| e.to_string())?
        .run()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn get_rank(db: &D1Database, streak: i64) -> Result<i64, String> {
    let stmt = db
        .prepare("SELECT COUNT(*) as cnt FROM users WHERE streak > ?1")
        .bind(&[streak.into()])
        .map_err(|e| e.to_string())?;

    #[derive(serde::Deserialize)]
    struct CountRow {
        cnt: i64,
    }

    let result = stmt.first::<CountRow>(None).await.map_err(|e| e.to_string())?;
    Ok(result.map(|r| r.cnt + 1).unwrap_or(1))
}

pub async fn get_leaderboard(db: &D1Database) -> Result<Vec<LeaderboardEntry>, String> {
    let results = db
        .prepare("SELECT username, streak FROM users ORDER BY streak DESC LIMIT 10")
        .all()
        .await
        .map_err(|e| e.to_string())?;

    #[derive(serde::Deserialize)]
    struct Row {
        username: String,
        streak: i64,
    }

    let rows = results.results::<Row>().map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, r)| LeaderboardEntry {
            username: r.username,
            streak: r.streak,
            rank: i + 1,
        })
        .collect())
}
