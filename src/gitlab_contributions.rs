use std::fmt::Debug;

use reqwest::Client;
use serde::Deserialize;
use time::Duration;

use crate::{ContributionCollection, ContributionDay};

#[derive(Debug, Deserialize)]
struct GitLabData {
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

pub async fn get_gitlab_contributions() -> Result<ContributionCollection, reqwest::Error> {
    let server = dotenv::var("GITLAB_SERVER").expect("GITLAB_SERVER must be set");
    let server = server.trim_end_matches('/');
    let username = dotenv::var("GITLAB_USERNAME").expect("GITLAB_USERNAME must be set");
    let token = dotenv::var("GITLAB_TOKEN").expect("GITLAB_TOKEN must be set");

    let endpoint = format!("{}/api/v4/users/{}/events", server, username);
    let client = Client::new();

    let res = client
        .get(endpoint)
        .header("PRIVATE-TOKEN", token)
        .send()
        .await?;

    let json_value: serde_json::Value =
        serde_json::from_str(&res.text().await.expect("Failed to get text"))
            .expect("Failed to parse JSON");
    let data: Vec<GitLabData> = serde_json::from_value(json_value).expect("Failed to parse JSON");

    let (contributions_per_week, max_contributions) = process_contributions(data);

    Ok(ContributionCollection {
        provider: "GitLab".to_string(),
        contributions: contributions_per_week,
        max_contributions: max_contributions,
    })
}

fn process_contributions(data: Vec<GitLabData>) -> (Vec<(i64, Vec<ContributionDay>)>, i64) {
    let mut contributions_per_week: Vec<(i64, Vec<ContributionDay>)> = vec![];
    let mut max_contributions = 0;

    let end_date = time::OffsetDateTime::now_utc();
    let start_date = end_date
        .checked_sub(Duration::days(365))
        .expect("Failed to subtract 365 days")
        .checked_sub(Duration::days(end_date.weekday() as i64 - 1))
        .expect("Failed to subtract days to start of week");

    let mut current_date = start_date;
    let mut current_weeknumber = 0;
    while current_date < end_date {
        let mut contribution_days = vec![];
        for i in 0..7 {
            contribution_days.push(ContributionDay {
                contribution_count: 0,
                weeknumber: current_weeknumber,
                date: format!(
                    "{}-{:02}-{:02}",
                    current_date.year(),
                    // Month number
                    current_date.month() as u8,
                    current_date.day()
                ),
                weekday: current_date.weekday().number_days_from_sunday() as i64,
            });
            current_date = current_date.checked_add(Duration::days(1)).unwrap();
        }
        contributions_per_week.push((current_weeknumber, contribution_days));
        current_weeknumber += 1;
    }

    for contribution in data {
        let date = contribution.created_at.split('T').collect::<Vec<&str>>()[0];
        let date = date.split('-').collect::<Vec<&str>>();
        let date = (
            date[0].parse::<i32>().unwrap(),
            date[1].parse::<u32>().unwrap(),
            date[2].parse::<u32>().unwrap(),
        );
        let contribution_day = contributions_per_week
            .iter_mut()
            .find(|(_, contribution_days)| {
                contribution_days
                    .iter()
                    .any(|day| day.date == format!("{}-{:02}-{:02}", date.0, date.1, date.2))
            })
            .expect("Failed to find contribution day A");

        let contribution_day = &mut contribution_day.1;
        let contribution_day = contribution_day
            .iter_mut()
            .find(|day| day.date == format!("{}-{:02}-{:02}", date.0, date.1, date.2))
            .expect("Failed to find contribution day B");

        contribution_day.contribution_count += 1;
        if contribution_day.contribution_count > max_contributions {
            max_contributions = contribution_day.contribution_count;
        }
    }

    (contributions_per_week, max_contributions)
}
