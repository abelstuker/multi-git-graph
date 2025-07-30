#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Date parsing error: {0}")]
    DateError(String),
}
