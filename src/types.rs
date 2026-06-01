use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub username: String,
    pub streak: i64,
    pub longest_streak: i64,
    pub last_loaded: String,
}

#[derive(Debug, Deserialize)]
pub struct ContributionDay {
    #[serde(rename = "contributionCount")]
    pub contribution_count: i64,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct ContributionWeek {
    #[serde(rename = "contributionDays")]
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Deserialize)]
pub struct ContributionCalendar {
    pub weeks: Vec<ContributionWeek>,
}

#[derive(Debug, Deserialize)]
pub struct ContributionsCollection {
    #[serde(rename = "contributionCalendar")]
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    #[serde(rename = "contributionsCollection")]
    pub contributions_collection: ContributionsCollection,
}

#[derive(Debug, Deserialize)]
pub struct GitHubData {
    pub user: Option<GitHubUser>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubResponse {
    pub data: Option<GitHubData>,
}

#[derive(Debug, Clone)]
pub struct StreakResult {
    pub streak: i64,
    pub longest_streak: i64,
}

#[derive(Debug)]
pub struct LeaderboardEntry {
    pub username: String,
    pub streak: i64,
    pub rank: usize,
}
