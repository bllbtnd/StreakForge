use chrono::NaiveDate;
use chrono_tz::Tz;
use worker::{Fetch, Method, Request, RequestInit, Headers};
use crate::types::{ContributionDay, GitHubResponse, StreakResult};

const GITHUB_API_URL: &str = "https://api.github.com/graphql";

const GRAPHQL_QUERY: &str = r#"
query($username: String!) {
  user(login: $username) {
    contributionsCollection {
      contributionCalendar {
        weeks {
          contributionDays {
            contributionCount
            date
          }
        }
      }
    }
  }
}
"#;

pub async fn fetch_contributions(
    username: &str,
    token: &str,
) -> Result<Vec<ContributionDay>, String> {
    let body = serde_json::json!({
        "query": GRAPHQL_QUERY,
        "variables": { "username": username }
    });

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", token)).map_err(|e| e.to_string())?;
    headers.set("Content-Type", "application/json").map_err(|e| e.to_string())?;
    headers.set("User-Agent", "StreakForge/1.0").map_err(|e| e.to_string())?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(wasm_bindgen::JsValue::from_str(&body.to_string())));

    let request = Request::new_with_init(GITHUB_API_URL, &init).map_err(|e| e.to_string())?;
    let mut response = Fetch::Request(request).send().await.map_err(|e| e.to_string())?;

    let status = response.status_code();
    if !(200..300).contains(&status) {
        return Err(format!("GitHub API returned {}", status));
    }

    let parsed: GitHubResponse = response.json().await.map_err(|e| e.to_string())?;

    let days = parsed
        .data
        .and_then(|d| d.user)
        .map(|u| {
            u.contributions_collection
                .contribution_calendar
                .weeks
                .into_iter()
                .flat_map(|w| w.contribution_days)
                .collect::<Vec<_>>()
        });

    match days {
        Some(d) => Ok(d),
        None => Err("user_not_found".to_string()),
    }
}

pub fn compute_streak(days: &[ContributionDay], tz: Tz) -> StreakResult {
    let today = chrono::Utc::now().with_timezone(&tz).date_naive();

    let mut all_days: Vec<(NaiveDate, i64)> = days
        .iter()
        .filter_map(|d| {
            NaiveDate::parse_from_str(&d.date, "%Y-%m-%d")
                .ok()
                .map(|date| (date, d.contribution_count))
        })
        .collect();

    all_days.sort_by_key(|(date, _)| *date);

    let longest_streak = compute_longest(&all_days);
    let streak = compute_current(&all_days, today);

    StreakResult { streak, longest_streak }
}

fn compute_current(days: &[(NaiveDate, i64)], today: NaiveDate) -> i64 {
    let mut streak = 0i64;
    let mut expected = today;

    for (date, count) in days.iter().rev() {
        if *date > today {
            continue;
        }
        if *date == expected && *count > 0 {
            streak += 1;
            expected = expected.pred_opt().unwrap_or(expected);
        } else if *date == expected && *count == 0 {
            if *date == today {
                expected = expected.pred_opt().unwrap_or(expected);
            } else {
                break;
            }
        } else if *date < expected {
            break;
        }
    }

    streak
}

fn compute_longest(days: &[(NaiveDate, i64)]) -> i64 {
    let mut longest = 0i64;
    let mut current = 0i64;
    let mut prev: Option<NaiveDate> = None;

    for (date, count) in days {
        if *count > 0 {
            let consecutive = prev.map_or(false, |p| {
                date.signed_duration_since(p).num_days() == 1
            });
            if consecutive {
                current += 1;
            } else {
                current = 1;
            }
            if current > longest {
                longest = current;
            }
            prev = Some(*date);
        } else {
            prev = None;
        }
    }

    longest
}
