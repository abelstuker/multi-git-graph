use crate::{
    ContributionCollection, ProviderError,
    processor::{ContributionProcessor, Event},
};
use reqwest::Client;
use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug)]
struct GitHubConfig {
    token: String,
    username: String,
}

impl GitHubConfig {
    fn from_env() -> Result<Self, ProviderError> {
        Ok(Self {
            token: dotenv::var("GITHUB_TOKEN")
                .map_err(|_| ProviderError::ConfigError("GITHUB_TOKEN must be set".into()))?,
            username: dotenv::var("GITHUB_USERNAME")
                .map_err(|_| ProviderError::ConfigError("GITHUB_USERNAME must be set".into()))?,
        })
    }
}

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

#[derive(Deserialize, Debug)]
struct GitHubResponse {
    data: GitHubData,
}

#[derive(Deserialize, Debug)]
struct GitHubData {
    user: GitHubUser,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct GitHubUser {
    name: String,
    #[serde(rename = "contributionsCollection")]
    contributions_collection: GitHubContributionsCollection,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct GitHubContributionsCollection {
    #[serde(rename = "contributionCalendar")]
    contribution_calendar: GitHubContributionCalendar,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct GitHubContributionCalendar {
    colors: Vec<String>,
    #[serde(rename = "totalContributions")]
    total_contributions: i64,
    weeks: Vec<GitHubWeek>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct GitHubWeek {
    #[serde(rename = "contributionDays")]
    contribution_days: Vec<GitHubContributionDay>,
    #[serde(rename = "firstDay")]
    first_day: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
struct GitHubContributionDay {
    color: String,
    #[serde(rename = "contributionCount")]
    contribution_count: i64,
    date: String,
    weekday: i64,
}

impl Event for GitHubContributionDay {
    fn timestamp(&self) -> Result<OffsetDateTime, ProviderError> {
        time::Date::parse(
            &self.date,
            &time::format_description::well_known::Iso8601::DATE,
        )
        .map(|date| date.midnight().assume_utc())
        .map_err(|e| ProviderError::DateError(format!("Failed to parse date: {}", e)))
    }
    fn contributions(&self) -> i64 {
        self.contribution_count
    }
}

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

    async fn fetch_events(
        &self,
        _start_date: OffsetDateTime,
        _end_date: OffsetDateTime,
    ) -> Result<Vec<GitHubContributionDay>, ProviderError> {
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

        let github_response: GitHubResponse = response.json().await?;
        let events = github_response
            .data
            .user
            .contributions_collection
            .contribution_calendar
            .weeks
            .into_iter()
            .flat_map(|week| week.contribution_days)
            .collect();
        Ok(events)
    }
}

pub async fn get_github_contributions(
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
) -> Result<ContributionCollection, ProviderError> {
    let config = GitHubConfig::from_env()?;
    let client = GitHubClient::new(config);
    let events = client.fetch_events(start_date, end_date).await?;

    let processor = ContributionProcessor::new(start_date, end_date);
    let calendar = processor.initialize_contribution_calendar();
    let (contributions, max_contributions) = processor.process_events(events, calendar)?;

    Ok(ContributionCollection {
        provider: "GitHub".to_string(),
        contributions,
        max_contributions,
    })
}
