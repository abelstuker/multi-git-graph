use crate::processor::{ContributionProcessor, Event};
use crate::{ContributionCollection, ProviderError};
use reqwest::Client;
use serde::Deserialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

#[derive(Debug)]
struct GitLabConfig {
    server: String,
    username: String,
    token: String,
}

impl GitLabConfig {
    fn from_env() -> Result<Self, ProviderError> {
        Ok(Self {
            server: dotenv::var("GITLAB_SERVER")
                .map_err(|_| ProviderError::ConfigError("GITLAB_SERVER must be set".into()))?
                .trim_end_matches('/')
                .to_string(),
            username: dotenv::var("GITLAB_USERNAME")
                .map_err(|_| ProviderError::ConfigError("GITLAB_USERNAME must be set".into()))?,
            token: dotenv::var("GITLAB_TOKEN")
                .map_err(|_| ProviderError::ConfigError("GITLAB_TOKEN must be set".into()))?,
        })
    }
}

#[allow(dead_code)]
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

impl Event for GitLabEvent {
    fn timestamp(&self) -> Result<OffsetDateTime, ProviderError> {
        OffsetDateTime::parse(&self.created_at, &Rfc3339)
            .map_err(|e| ProviderError::DateError(format!("Failed to parse date: {}", e)))
    }

    fn contributions(&self) -> i64 {
        1
    }
}

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
    ) -> Result<Vec<GitLabEvent>, ProviderError> {
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

pub async fn get_gitlab_contributions(
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
) -> Result<ContributionCollection, ProviderError> {
    let config = GitLabConfig::from_env()?;
    let client = GitLabClient::new(config);
    let events = client.fetch_events(start_date, end_date).await?;

    let processor = ContributionProcessor::new(start_date, end_date);
    let calendar = processor.initialize_contribution_calendar();
    let (contributions, max_contributions) = processor.process_events(events, calendar)?;

    Ok(ContributionCollection {
        provider: "GitLab".to_string(),
        contributions,
        max_contributions,
    })
}
