use reqwest::Client;
use serde::Deserialize;
use std::fmt::Debug;
use thiserror::Error;
use time::{Date, Duration, OffsetDateTime};

use crate::{ContributionCollection, ContributionDay};

// Configuration
#[derive(Debug)]
struct GiteaConfig {
    server: String,
    username: String,
    token: String,
}

impl GiteaConfig {
    fn from_env() -> Result<Self, GitLabError> {
        Ok(Self {
            server: dotenv::var("GITEA_SERVER")
                .map_err(|_| GitLabError::ConfigError("GITEA_SERVER must be set".into()))?
                .trim_end_matches('/')
                .to_string(),
            username: dotenv::var("GITEA_USERNAME")
                .map_err(|_| GitLabError::ConfigError("GITEA_USERNAME must be set".into()))?,
            token: dotenv::var("GITEA_TOKEN")
                .map_err(|_| GitLabError::ConfigError("GITEA_TOKEN must be set".into()))?,
        })
    }
}

// API Types
#[derive(Debug, Deserialize)]
struct GiteaEvent {
    timestamp: i64,
    contributions: i64,
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
struct GiteaClient {
    client: Client,
    config: GiteaConfig,
}

impl GiteaClient {
    fn new(config: GiteaConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    async fn fetch_events(
        &self,
        start_date: OffsetDateTime,
        end_date: OffsetDateTime,
    ) -> Result<Vec<GiteaEvent>, GitLabError> {
        /*
              curl -X 'GET' \
        'https://git.infogroep.be/api/v1/users/abstuker/heatmap' \
        -H 'accept: application/json' \
        -H 'Authorization: token <TOKEN> */
        let endpoint = format!(
            "{}/api/v1/users/{}/heatmap",
            self.config.server, self.config.username
        );

        let response = self
            .client
            .get(endpoint)
            .header("Authorization", format!("token {}", self.config.token))
            .header("accept", "application/json")
            .send()
            .await?
            .error_for_status()?;

        let events: Vec<GiteaEvent> = response.json().await?;
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
        events: Vec<GiteaEvent>,
        mut calendar: Vec<(i64, Vec<ContributionDay>)>,
    ) -> (Vec<(i64, Vec<ContributionDay>)>, i64) {
        let mut max_contributions = 0;

        for event in events {
            let event_date = match OffsetDateTime::from_unix_timestamp(event.timestamp) {
                Ok(date) => date,
                Err(e) => panic!("Failed to parse date: {}", e),
            };
            if event_date.lt(&self.start_date) || event_date.ge(&self.end_date) {
                continue;
            }

            let days_since_start = (event_date - self.start_date).whole_days();
            let week_number = (days_since_start / 7) as usize;
            let day_number = (days_since_start % 7) as usize;

            let contribution = &mut calendar[week_number].1[day_number];
            contribution.contribution_count += event.contributions;
            max_contributions = max_contributions.max(contribution.contribution_count);
        }

        (calendar, max_contributions)
    }
}

// Public API
pub async fn get_gitea_contributions(
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
) -> Result<ContributionCollection, GitLabError> {
    let config = GiteaConfig::from_env()?;
    let client = GiteaClient::new(config);
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
