use crate::{
    ContributionCollection, ProviderError,
    processor::{ContributionProcessor, Event},
};
use reqwest::Client;
use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug)]
struct GiteaConfig {
    server: String,
    username: String,
    token: String,
}

impl GiteaConfig {
    fn from_env() -> Result<Self, ProviderError> {
        Ok(Self {
            server: dotenv::var("GITEA_SERVER")
                .map_err(|_| ProviderError::ConfigError("GITEA_SERVER must be set".into()))?
                .trim_end_matches('/')
                .to_string(),
            username: dotenv::var("GITEA_USERNAME")
                .map_err(|_| ProviderError::ConfigError("GITEA_USERNAME must be set".into()))?,
            token: dotenv::var("GITEA_TOKEN")
                .map_err(|_| ProviderError::ConfigError("GITEA_TOKEN must be set".into()))?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct GiteaEvent {
    timestamp: i64,
    contributions: i64,
}

impl Event for GiteaEvent {
    fn timestamp(&self) -> Result<OffsetDateTime, ProviderError> {
        OffsetDateTime::from_unix_timestamp(self.timestamp)
            .map_err(|e| ProviderError::DateError(format!("Failed to parse date: {}", e)))
    }
    fn contributions(&self) -> i64 {
        self.contributions
    }
}

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
        _start_date: OffsetDateTime,
        _end_date: OffsetDateTime,
    ) -> Result<Vec<GiteaEvent>, ProviderError> {
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

pub async fn get_gitea_contributions(
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
) -> Result<ContributionCollection, ProviderError> {
    let config = GiteaConfig::from_env()?;
    let client = GiteaClient::new(config);
    let events = client.fetch_events(start_date, end_date).await?;

    let processor = ContributionProcessor::new(start_date, end_date);
    let calendar = processor.initialize_contribution_calendar();
    let (contributions, max_contributions) = processor.process_events(events, calendar)?;

    Ok(ContributionCollection {
        provider: "Gitea".to_string(),
        contributions,
        max_contributions,
    })
}
