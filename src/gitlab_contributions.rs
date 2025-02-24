use reqwest::Client;
use serde::Deserialize;
use std::fmt::Debug;
use thiserror::Error;
use time::{Date, Duration, OffsetDateTime};

use crate::{ContributionCollection, ContributionDay};

// Configuration
#[derive(Debug)]
struct GitLabConfig {
    server: String,
    username: String,
    token: String,
}

impl GitLabConfig {
    fn from_env() -> Result<Self, GitLabError> {
        Ok(Self {
            server: dotenv::var("GITLAB_SERVER")
                .map_err(|_| GitLabError::ConfigError("GITLAB_SERVER must be set".into()))?
                .trim_end_matches('/')
                .to_string(),
            username: dotenv::var("GITLAB_USERNAME")
                .map_err(|_| GitLabError::ConfigError("GITLAB_USERNAME must be set".into()))?,
            token: dotenv::var("GITLAB_TOKEN")
                .map_err(|_| GitLabError::ConfigError("GITLAB_TOKEN must be set".into()))?,
        })
    }
}

// API Types
#[derive(Debug, Deserialize)]
struct GitLabEvent {
    id: i64,
    project_id: i64,
    action_name: String,
    target_id: Option<i64>,
    target_iid: Option<i64>,
    target_type: Option<String>,
    author_id: i64,
    target_title: Option<String>,
    created_at: String,
}

// Error handling
#[derive(Debug, Error)]
pub enum GitLabError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Date parsing error: {0}")]
    DateError(String),
}

// GitLab client
struct GitLabClient {
    client: Client,
    config: GitLabConfig,
}

impl GitLabClient {
    fn new(config: GitLabConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    async fn fetch_events(
        &self,
        start_date: OffsetDateTime,
        end_date: OffsetDateTime,
    ) -> Result<Vec<GitLabEvent>, GitLabError> {
        let endpoint = format!(
            "{}/api/v4/users/{}/events?after={}-{}-{}&before={}-{}-{}",
            self.config.server,
            self.config.username,
            start_date.year(),
            start_date.month(),
            start_date.day(),
            end_date.year(),
            end_date.month(),
            end_date.day()
        );

        let response = self
            .client
            .get(endpoint)
            .header("PRIVATE-TOKEN", &self.config.token)
            .send()
            .await?
            .error_for_status()?;

        let events: Vec<GitLabEvent> = response.json().await?;
        Ok(events)
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

    fn initialize_contribution_calendar(&self) -> Vec<(i64, Vec<ContributionDay>)> {
        let mut contributions = Vec::new();
        let mut current_date = self.start_date;
        let mut week_number = 0;

        while current_date < self.end_date {
            let mut week_days = Vec::new();

            for _ in 0..7 {
                week_days.push(ContributionDay {
                    contribution_count: 0,
                    weeknumber: week_number,
                    date: format!(
                        "{}-{:02}-{:02}",
                        current_date.year(),
                        current_date.month() as u8,
                        current_date.day()
                    ),
                    weekday: current_date.weekday().number_days_from_sunday() as i64,
                });
                current_date = current_date
                    .checked_add(Duration::days(1))
                    .expect("Failed to increment date");
            }

            contributions.push((week_number, week_days));
            week_number += 1;
        }

        contributions
    }

    fn process_events(
        &self,
        events: Vec<GitLabEvent>,
        mut calendar: Vec<(i64, Vec<ContributionDay>)>,
    ) -> (Vec<(i64, Vec<ContributionDay>)>, i64) {
        let mut max_contributions = 0;

        for event in events {
            let date_str = event
                .created_at
                .split('T')
                .next()
                .expect("Invalid date format");

            // Find and update the contribution day
            for (_, week) in calendar.iter_mut() {
                for day in week.iter_mut() {
                    if day.date == date_str {
                        day.contribution_count += 1;
                        max_contributions = max_contributions.max(day.contribution_count);
                        break;
                    }
                }
            }
        }

        (calendar, max_contributions)
    }
}

// Public API
pub async fn get_gitlab_contributions(
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
) -> Result<ContributionCollection, GitLabError> {
    let config = GitLabConfig::from_env()?;
    let client = GitLabClient::new(config);
    let events = client.fetch_events(start_date, end_date).await?;

    let processor = ContributionProcessor::new(start_date, end_date);
    let calendar = processor.initialize_contribution_calendar();
    let (contributions, max_contributions) = processor.process_events(events, calendar);

    Ok(ContributionCollection {
        provider: "GitLab".to_string(),
        contributions,
        max_contributions,
    })
}
