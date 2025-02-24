use crate::{ContributionCollection, ContributionDay};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use time::OffsetDateTime;

// Configuration
struct GitHubConfig {
    token: String,
    username: String,
}

impl GitHubConfig {
    fn from_env() -> Result<Self, GitHubError> {
        Ok(Self {
            token: dotenv::var("GITHUB_TOKEN")
                .map_err(|_| GitHubError::ConfigError("GITHUB_TOKEN must be set".into()))?,
            username: dotenv::var("GITHUB_USERNAME")
                .map_err(|_| GitHubError::ConfigError("GITHUB_USERNAME must be set".into()))?,
        })
    }
}

// GraphQL query constant
const GITHUB_CONTRIBUTIONS_QUERY: &str = r#"
query($username: String!) {
    user(login: $username) {
        name
        contributionsCollection {
            contributionCalendar {
                colors
                totalContributions
                weeks {
                    contributionDays {
                        color
                        contributionCount
                        date
                        weekday
                    }
                    firstDay
                }
            }
        }
    }
}
"#;

// API response types
#[derive(Deserialize, Debug)]
struct GitHubResponse {
    data: GitHubData,
}

#[derive(Deserialize, Debug)]
struct GitHubData {
    user: GitHubUser,
}

#[derive(Deserialize, Debug)]
struct GitHubUser {
    name: String,
    #[serde(rename = "contributionsCollection")]
    contributions_collection: GitHubContributionsCollection,
}

#[derive(Deserialize, Debug)]
struct GitHubContributionsCollection {
    #[serde(rename = "contributionCalendar")]
    contribution_calendar: GitHubContributionCalendar,
}

#[derive(Deserialize, Debug)]
struct GitHubContributionCalendar {
    colors: Vec<String>,
    #[serde(rename = "totalContributions")]
    total_contributions: i64,
    weeks: Vec<GitHubWeek>,
}

#[derive(Deserialize, Debug)]
struct GitHubWeek {
    #[serde(rename = "contributionDays")]
    contribution_days: Vec<GitHubContributionDay>,
    #[serde(rename = "firstDay")]
    first_day: String,
}

#[derive(Deserialize, Debug, Clone)]
struct GitHubContributionDay {
    color: String,
    #[serde(rename = "contributionCount")]
    contribution_count: i64,
    date: String,
    weekday: i64,
}

// Custom error type
#[derive(Debug, thiserror::Error)]
pub enum GitHubError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("GitHub API error: {0}")]
    ApiError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

// GitHub client
struct GitHubClient {
    client: Client,
    config: GitHubConfig,
}

impl GitHubClient {
    fn new(config: GitHubConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    async fn fetch_contributions(&self) -> Result<GitHubResponse, GitHubError> {
        let variables = serde_json::json!({
            "username": self.config.username,
        });

        let body = serde_json::json!({
            "query": GITHUB_CONTRIBUTIONS_QUERY,
            "variables": variables,
        });

        let response = self
            .client
            .post("https://api.github.com/graphql")
            .header("Authorization", format!("Bearer {}", self.config.token))
            .header("User-Agent", "All Contributions CLI")
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let github_response = response.json().await?;
        Ok(github_response)
    }
}

// Contribution processor
struct ContributionProcessor {
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
}

impl ContributionProcessor {
    fn new(start_date: OffsetDateTime, end_date: OffsetDateTime) -> Self {
        Self {
            start_date,
            end_date,
        }
    }

    fn process_calendar(self, calendar: &GitHubContributionCalendar) -> ContributionCollection {
        let contributions: Vec<(i64, Vec<ContributionDay>)> = calendar
            .weeks
            .iter()
            .enumerate()
            .map(|(week_idx, week)| {
                let weeknumber = week_idx as i64;
                let days = week
                    .contribution_days
                    .iter()
                    .map(|day| ContributionDay {
                        contribution_count: day.contribution_count,
                        date: day.date.clone(),
                        weekday: day.weekday,
                        weeknumber,
                    })
                    .collect();
                (weeknumber, days)
            })
            .collect();

        let max_contributions = calendar
            .weeks
            .iter()
            .flat_map(|week| week.contribution_days.iter())
            .map(|day| day.contribution_count)
            .max()
            .unwrap_or(0);

        ContributionCollection {
            provider: "GitHub".to_string(),
            contributions,
            max_contributions,
        }
    }
}

// Public API
pub async fn get_github_contributions(
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
) -> Result<ContributionCollection, GitHubError> {
    let config = GitHubConfig::from_env()?;
    let client = GitHubClient::new(config);
    let response = client.fetch_contributions().await?;

    let calendar = response
        .data
        .user
        .contributions_collection
        .contribution_calendar;
    let processor = ContributionProcessor::new(start_date, end_date);
    let contributions = processor.process_calendar(&calendar);

    Ok(contributions)
}
