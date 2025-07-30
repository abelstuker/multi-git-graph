use crate::{
    ContributionCollection, GenericConfig, ProviderConfig,
    errors::ProviderError,
    processor::{ContributionProcessor, Event},
};
use reqwest::Client;
use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug)]
struct CodebergConfig(GenericConfig);

impl ProviderConfig for CodebergConfig {
    fn from_env() -> Result<Self, ProviderError> {
        GenericConfig::new("CODEBERG_SERVER", "CODEBERG_USERNAME", "CODEBERG_TOKEN")
            .map(CodebergConfig)
    }
    fn server(&self) -> &str {
        self.0.server()
    }
    fn username(&self) -> &str {
        self.0.username()
    }
    fn token(&self) -> &str {
        self.0.token()
    }
}

#[derive(Debug, Deserialize)]
struct CodebergEvent {
    timestamp: i64,
    contributions: i64,
}

impl Event for CodebergEvent {
    fn timestamp(&self) -> Result<OffsetDateTime, ProviderError> {
        OffsetDateTime::from_unix_timestamp(self.timestamp)
            .map_err(|e| ProviderError::DateError(format!("Failed to parse date: {}", e)))
    }
    fn contributions(&self) -> i64 {
        self.contributions
    }
}

struct CodebergClient {
    client: Client,
    config: CodebergConfig,
}

impl CodebergClient {
    fn new(config: CodebergConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    async fn fetch_events(
        &self,
        _start_date: OffsetDateTime,
        _end_date: OffsetDateTime,
    ) -> Result<Vec<CodebergEvent>, ProviderError> {
        let endpoint = format!(
            "{}/api/v1/users/{}/heatmap",
            self.config.server(),
            self.config.username()
        );

        let response = self
            .client
            .get(endpoint)
            .header("Authorization", format!("token {}", self.config.token()))
            .header("accept", "application/json")
            .send()
            .await?
            .error_for_status()?;

        let events: Vec<CodebergEvent> = response.json().await?;
        Ok(events)
    }
}

pub async fn get_codeberg_contributions(
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
) -> Result<ContributionCollection, ProviderError> {
    let config = CodebergConfig::from_env()?;
    let client = CodebergClient::new(config);
    let events = client.fetch_events(start_date, end_date).await?;

    let processor = ContributionProcessor::new(start_date, end_date);
    let calendar = processor.initialize_contribution_calendar();
    let (contributions, max_contributions) = processor.process_events(events, calendar)?;

    Ok(ContributionCollection {
        provider: "Codeberg".to_string(),
        contributions,
        max_contributions,
    })
}
