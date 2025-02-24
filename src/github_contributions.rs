use std::sync::LazyLock;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{ContributionCollection, ContributionDay};

static GITHUB_TOKEN: LazyLock<String> =
    LazyLock::new(|| dotenv::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found"));
static USERNAME: LazyLock<String> =
    LazyLock::new(|| dotenv::var("GITHUB_USERNAME").expect("GITHUB_USERNAME not found"));

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

pub async fn get_github_contributions() -> Result<ContributionCollection, reqwest::Error> {
    let endpoint = "https://api.github.com/graphql";
    let client = Client::new();

    let body = serde_json::json!({
        "query": format!(r#"query {{
            user(login: "{}") {{
              name
              contributionsCollection {{
                contributionCalendar {{
                  colors
                  totalContributions
                  weeks {{
                    contributionDays {{
                      color
                      contributionCount
                      date
                      weekday
                    }}
                    firstDay
                  }}
                }}
              }}
            }}
          }}"#, *USERNAME)
    });

    let res = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", *GITHUB_TOKEN))
        .header("User-Agent", "All Contributions CLI")
        .body(body.to_string())
        .send()
        .await?;

    let json_value: serde_json::Value =
        serde_json::from_str(&res.text().await.expect("Failed to get text"))
            .expect("Failed to parse JSON");
    let data: GitHubData =
        serde_json::from_value(json_value["data"].clone()).expect("Failed to parse JSON");

    Ok(ContributionCollection {
        provider: "GitHub".to_string(),
        contributions: data
            .user
            .contributions_collection
            .contribution_calendar
            .weeks
            .iter()
            .map(|week| {
                (
                    week.first_day.clone(),
                    week.contribution_days
                        .iter()
                        .map(|day| ContributionDay {
                            contribution_count: day.contribution_count,
                            date: day.date.clone(),
                            weekday: day.weekday,
                        })
                        .collect(),
                )
            })
            .collect(),
        max_contributions: data
            .user
            .contributions_collection
            .contribution_calendar
            .weeks
            .iter()
            .map(|week| {
                week.contribution_days
                    .iter()
                    .map(|day| day.contribution_count)
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap(),
    })
}
