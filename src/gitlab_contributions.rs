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
    // curl --header "PRIVATE-TOKEN: ..." "https://gitlab.soft.vub.ac.be/api/v4/users/USERNAME/events"
    /*
       Response:
       [{"id":85097,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-12-09T13:05:25.469+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"3bb3c610f2e3f176914d8ab259d4d6f68d11e6ed","commit_to":"2d127c693d3cf084d60a7a72e62d51c969a9eb23","ref":"Drawing-Roads","commit_title":"tests: extend screen width for button accessibility during gui tests","ref_count":null},"author_username":"abelstuker"},{"id":85096,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-12-09T12:50:59.756+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"f368f0234e47f7f3b9f7ccaba2337510bb4b41af","commit_to":"7323b7aeee1e9f4b724e41a4dbcc21ff00494f26","ref":"integration","commit_title":"chore: bump versions","ref_count":null},"author_username":"abelstuker"},{"id":85082,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-12-09T09:16:18.585+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":50,"action":"pushed","ref_type":"branch","commit_from":"d79a49510f7fa6ac84c80b0e656338102d80ca74","commit_to":"3bb3c610f2e3f176914d8ab259d4d6f68d11e6ed","ref":"Drawing-Roads","commit_title":"Merge branch 'integration' into Drawing-Roads","ref_count":null},"author_username":"abelstuker"},{"id":85080,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-12-09T01:06:02.629+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"e9e19cf4481e5db1e447949c96cc29ce28317a75","commit_to":"d79a49510f7fa6ac84c80b0e656338102d80ca74","ref":"Drawing-Roads","commit_title":"Revert \"feat: allow anchors on all non-junction road ends\"","ref_count":null},"author_username":"abelstuker"},{"id":85079,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-12-08T23:30:53.782+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"38ca103ae6a91f2843430d9e9029b6fb35682b46","commit_to":"e9e19cf4481e5db1e447949c96cc29ce28317a75","ref":"Drawing-Roads","commit_title":"feat: allow anchors on all non-junction road ends","ref_count":null},"author_username":"abelstuker"},{"id":84985,"project_id":1877,"action_name":"pushed new","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-12-05T11:04:16.649+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"created","ref_type":"branch","commit_from":null,"commit_to":"11b591ef230c89e4dd99b98935fdf9818dc72536","ref":"FixTests","commit_title":"tests: fix resizable roads and junction move tests","ref_count":null},"author_username":"abelstuker"},{"id":84808,"project_id":1877,"action_name":"pushed new","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-29T09:48:09.109+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":64,"action":"created","ref_type":"branch","commit_from":null,"commit_to":"38ca103ae6a91f2843430d9e9029b6fb35682b46","ref":"integration","commit_title":"feat: new junction creation on junction dragging with intersecting ...","ref_count":null},"author_username":"abelstuker"},{"id":84611,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-24T15:20:30.902+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"f6e3d14b0f8142fd8bbdf6ccf66bc44269860106","commit_to":"38ca103ae6a91f2843430d9e9029b6fb35682b46","ref":"Drawing-Roads","commit_title":"feat: new junction creation on junction dragging with intersecting ...","ref_count":null},"author_username":"abelstuker"},{"id":84329,"project_id":1877,"action_name":"closed","target_id":606,"target_iid":2,"target_type":"Issue","author_id":599,"target_title":"roads arriving at junction not properly connected","created_at":"2024-11-17T22:13:49.053+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","author_username":"abelstuker"},{"id":84328,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-17T22:13:13.173+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"d169a1381293aec0686593a1eea5c65306f748ae","commit_to":"9eaa5efeb8f565ccc23ebf779b2aae93edda6235","ref":"Drawing-Roads","commit_title":"tests: update tests for centered roads in junction","ref_count":null},"author_username":"abelstuker"},{"id":84327,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-17T22:06:56.390+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"0b1297e0b96fb33dd6a7f0253782e0c9e9f4d9a4","commit_to":"d169a1381293aec0686593a1eea5c65306f748ae","ref":"Drawing-Roads","commit_title":"fix: junction road centering issue","ref_count":null},"author_username":"abelstuker"},{"id":84326,"project_id":1877,"action_name":"opened","target_id":606,"target_iid":2,"target_type":"Issue","author_id":599,"target_title":"roads arriving at junction not properly connected","created_at":"2024-11-17T21:58:53.218+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","author_username":"abelstuker"},{"id":84289,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-17T15:20:58.286+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"16e4b0ae05fbc44547df89b90a27dc82cc5f0757","commit_to":"0b1297e0b96fb33dd6a7f0253782e0c9e9f4d9a4","ref":"Drawing-Roads","commit_title":"feat: show selected road type","ref_count":null},"author_username":"abelstuker"},{"id":84280,"project_id":1877,"action_name":"closed","target_id":605,"target_iid":1,"target_type":"Issue","author_id":599,"target_title":"select element button: unexpected behaviour","created_at":"2024-11-17T10:54:19.592+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","author_username":"abelstuker"},{"id":84279,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-17T10:53:59.785+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"86aa982405e8db021f1ac2b7749273485a260c07","commit_to":"16e4b0ae05fbc44547df89b90a27dc82cc5f0757","ref":"Drawing-Roads","commit_title":"fix: select element button unexpected behavior (closes #1)","ref_count":null},"author_username":"abelstuker"},{"id":84275,"project_id":1877,"action_name":"opened","target_id":605,"target_iid":1,"target_type":"Issue","author_id":599,"target_title":"select element button: unexpected behaviour","created_at":"2024-11-16T23:29:42.469+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","author_username":"abelstuker"},{"id":84274,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-16T22:31:08.047+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"54fce36fab0f50048a9c587e062becee34cbdfbb","commit_to":"86aa982405e8db021f1ac2b7749273485a260c07","ref":"Drawing-Roads","commit_title":"docs: method documentation","ref_count":null},"author_username":"abelstuker"},{"id":83926,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-03T21:06:15.868+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"f60d6ec70d3bfeeec0dca7046f83330ae4097d72","commit_to":"0e6ad6322f5fee7d863163b9c84bb50fabca3c97","ref":"Drawing-Roads","commit_title":"refactor: old snapping code","ref_count":null},"author_username":"abelstuker"},{"id":83924,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-03T20:13:02.980+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"8601d81c2fd57d69335f29b30f92b38ba5087f9a","commit_to":"f60d6ec70d3bfeeec0dca7046f83330ae4097d72","ref":"Drawing-Roads","commit_title":"fix: road drawing through multiple other roads bug","ref_count":null},"author_username":"abelstuker"},{"id":83917,"project_id":1877,"action_name":"pushed to","target_id":null,"target_iid":null,"target_type":null,"author_id":599,"target_title":null,"created_at":"2024-11-03T15:22:06.104+01:00","author":{"id":599,"username":"abelstuker","name":"Abel Stuker","state":"active","locked":false,"avatar_url":"https://secure.gravatar.com/avatar/613d5946897f129a1d46810e74c20c645605602298e3aade79b53dd292c3a469?s=80\u0026d=identicon","web_url":"https://gitlab.soft.vub.ac.be/abelstuker"},"imported":false,"imported_from":"none","push_data":{"commit_count":1,"action":"pushed","ref_type":"branch","commit_from":"58c2657321302b3aaa5fba3734dc85cf31107950","commit_to":"8601d81c2fd57d69335f29b30f92b38ba5087f9a","ref":"Drawing-Roads","commit_title":"chore: remove redundant snappable trait","ref_count":null},"author_username":"abelstuker"}]%
    */

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
    // Make a vector with all weeks since 1 year ago.
    // Every week has 7 contribution days, except maybe the last one
    // Input data contains contributions, more than one contribution can belong to the same day.
    // Days with no contributions should remain in the output

    let mut contributions_per_week: Vec<(i64, Vec<ContributionDay>)> = vec![];
    let mut max_contributions = 0;

    let end_date = time::OffsetDateTime::now_utc();
    let start_date = end_date
        .checked_sub(Duration::days(365))
        .expect("Failed to subtract 365 days")
        .checked_sub(Duration::days(end_date.weekday() as i64 - 1))
        .expect("Failed to subtract days to start of week");

    // Starting at start_date, fill a vector with empty contribution days for every week
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

    // Fill the contribution days with the actual contributions
    for contribution in data {
        // contribution.created_at to date or time object
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
