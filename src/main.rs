use crate::colors::ColorScheme;
use crate::errors::ProviderError;
use crate::types::{ContributionCollection, ContributionDay};
use time::{Duration, OffsetDateTime};

mod codeberg_contributions;
mod colors;
mod errors;
mod gitea_contributions;
mod github_contributions;
mod gitlab_contributions;
mod processor;
mod renderer;
mod types;

use renderer::ContributionGraphRenderer;

trait ProviderConfig {
    fn from_env() -> Result<Self, ProviderError>
    where
        Self: Sized;
    fn server(&self) -> &str;
    fn username(&self) -> &str;
    fn token(&self) -> &str;
}

#[derive(Debug)]
struct GenericConfig {
    server: String,
    username: String,
    token: String,
}

impl GenericConfig {
    fn new(server_var: &str, username_var: &str, token_var: &str) -> Result<Self, ProviderError> {
        Ok(Self {
            server: dotenv::var(server_var)
                .map_err(|_| ProviderError::ConfigError(format!("{} must be set", server_var)))?
                .trim_end_matches('/')
                .to_string(),
            username: dotenv::var(username_var)
                .map_err(|_| ProviderError::ConfigError(format!("{} must be set", username_var)))?,
            token: dotenv::var(token_var)
                .map_err(|_| ProviderError::ConfigError(format!("{} must be set", token_var)))?,
        })
    }
}

impl ProviderConfig for GenericConfig {
    fn from_env() -> Result<Self, ProviderError> {
        Self::new("SERVER", "USERNAME", "TOKEN") // Override per provider
    }
    fn server(&self) -> &str {
        &self.server
    }
    fn username(&self) -> &str {
        &self.username
    }
    fn token(&self) -> &str {
        &self.token
    }
}

// Data processing
async fn process_contributions(
    collections: Vec<Option<ContributionCollection>>,
) -> (Vec<Vec<Option<ContributionDay>>>, i64) {
    let mut contributions_per_row: Vec<Vec<Option<ContributionDay>>> = vec![vec![]; 7];
    let mut max_contributions = 0;

    for collection in collections {
        // Only process collections that are not None
        let collection = match collection {
            Some(collection) => collection,
            None => continue,
        };
        for (weeknumber, contributions) in collection.contributions {
            for (day, contribution) in contributions.iter().enumerate() {
                while contributions_per_row[day].len() <= weeknumber as usize {
                    contributions_per_row[day].push(None);
                }

                let current_contribution = &mut contributions_per_row[day][weeknumber as usize];
                match current_contribution {
                    Some(existing) => {
                        existing.contribution_count += contribution.contribution_count;
                        max_contributions = max_contributions.max(existing.contribution_count);
                    }
                    None => {
                        *current_contribution = Some(contribution.clone());
                        max_contributions = max_contributions.max(contribution.contribution_count);
                    }
                }
            }
        }
    }

    (contributions_per_row, max_contributions)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let end_date = OffsetDateTime::now_utc();
    let start_date = end_date
        .checked_sub(Duration::days(365))
        .expect("Failed to subtract 365 days");
    let start_date = start_date
        .checked_sub(Duration::days(
            start_date.weekday().number_days_from_sunday() as i64,
        ))
        .expect("Failed to subtract days to get to Sunday");

    let contributions: Vec<Option<ContributionCollection>> = vec![
        github_contributions::get_github_contributions(start_date, end_date)
            .await
            .ok(),
        gitlab_contributions::get_gitlab_contributions(start_date, end_date)
            .await
            .ok(),
        gitea_contributions::get_gitea_contributions(start_date, end_date)
            .await
            .ok(),
        codeberg_contributions::get_codeberg_contributions(start_date, end_date)
            .await
            .ok(),
    ];

    let (contributions_per_row, max_contributions) = process_contributions(contributions).await;

    let mut renderer = ContributionGraphRenderer::new(
        ColorScheme::find_by_name(
            dotenv::var("COLOR_SCHEME")
                .unwrap_or("github".to_string())
                .as_str(),
        )
        .expect("Color scheme not found")
        .colors
        .iter()
        .map(|&s| s.to_string())
        .collect(),
    );
    renderer.render_months(&contributions_per_row)?;
    renderer.render_graph(&contributions_per_row, max_contributions)?;

    Ok(())
}
