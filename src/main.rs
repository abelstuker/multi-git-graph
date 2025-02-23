use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::Write;
use termcolor::{ColorSpec, StandardStream, WriteColor};

/*
Example response from the GitHub API
[{"id":"46753292831","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224901,"name":"refspace/refspace-website","url":"https://api.github.com/repos/refspace/refspace-website"},"payload":{"repository_id":934224901,"push_id":22752057562,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"90af9d1a9ee02ffb59f15652010b3f57e7cce86d","before":"b9d9e6698ae7e4255d2c6cb91a34cc2ad6d5123f","commits":[{"sha":"90af9d1a9ee02ffb59f15652010b3f57e7cce86d","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"chore: tailwind prettier","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-website/commits/90af9d1a9ee02ffb59f15652010b3f57e7cce86d"}]},"public":false,"created_at":"2025-02-19T22:56:44Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46649369786","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224901,"name":"refspace/refspace-website","url":"https://api.github.com/repos/refspace/refspace-website"},"payload":{"repository_id":934224901,"push_id":22699762953,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"8198e832ebc4cea4385db41bb1e650887d9cd62c","before":"4623de64ed7a57921d3eb09dfd66e4fdcabbba71","commits":[{"sha":"8198e832ebc4cea4385db41bb1e650887d9cd62c","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"feat: basic login functionality","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-website/commits/8198e832ebc4cea4385db41bb1e650887d9cd62c"}]},"public":false,"created_at":"2025-02-17T13:34:19Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46649350519","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224901,"name":"refspace/refspace-website","url":"https://api.github.com/repos/refspace/refspace-website"},"payload":{"ref":"main","ref_type":"branch","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-17T13:33:50Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46649329993","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224901,"name":"refspace/refspace-frontend","url":"https://api.github.com/repos/refspace/refspace-frontend"},"payload":{"ref":null,"ref_type":"repository","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-17T13:33:19Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46649278888","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224223,"name":"AbelStuker/refspace-website","url":"https://api.github.com/repos/AbelStuker/refspace-website"},"payload":{"ref":"main","ref_type":"branch","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-17T13:31:59Z"},{"id":"46649277697","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224223,"name":"AbelStuker/refspace-website","url":"https://api.github.com/repos/AbelStuker/refspace-website"},"payload":{"ref":null,"ref_type":"repository","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-17T13:31:57Z"},{"id":"46649172124","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"repository_id":933686663,"push_id":22699666570,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"ea86f6c756ed02cdc6eab6757204397c75bcb770","before":"87dc419a08521021b6c71b149652304d83e9376a","commits":[{"sha":"ea86f6c756ed02cdc6eab6757204397c75bcb770","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"feat: auth jwt as http-only cookie","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-backend/commits/ea86f6c756ed02cdc6eab6757204397c75bcb770"}]},"public":false,"created_at":"2025-02-17T13:29:19Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46627492153","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"repository_id":933686663,"push_id":22688717487,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"87dc419a08521021b6c71b149652304d83e9376a","before":"520392dc7a0f5f41abcf45fc23f4fd08c588f1e2","commits":[{"sha":"87dc419a08521021b6c71b149652304d83e9376a","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"feat: auth data validation","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-backend/commits/87dc419a08521021b6c71b149652304d83e9376a"}]},"public":false,"created_at":"2025-02-17T00:27:26Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46621611470","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"repository_id":933686663,"push_id":22684768533,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"520392dc7a0f5f41abcf45fc23f4fd08c588f1e2","before":"a6efae33de187448219f51d27e740536c8fb2c9d","commits":[{"sha":"520392dc7a0f5f41abcf45fc23f4fd08c588f1e2","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"feat: passport auth","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-backend/commits/520392dc7a0f5f41abcf45fc23f4fd08c588f1e2"}]},"public":false,"created_at":"2025-02-16T15:36:20Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46619980317","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"ref":"main","ref_type":"branch","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-16T13:19:48Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46619947467","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"ref":null,"ref_type":"repository","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-16T13:17:06Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46611886347","type":"WatchEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":117118012,"name":"danleh/wasabi","url":"https://api.github.com/repos/danleh/wasabi"},"payload":{"action":"started"},"public":true,"created_at":"2025-02-15T22:24:14Z"}]
*/
use std::sync::LazyLock;

static GITHUB_TOKEN: LazyLock<String> =
    LazyLock::new(|| dotenv::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found"));
static USERNAME: LazyLock<String> =
    LazyLock::new(|| dotenv::var("GITHUB_USERNAME").expect("GITHUB_USERNAME not found"));

#[derive(Deserialize, Debug)]
struct Data {
    user: User,
}

#[derive(Deserialize, Debug)]
struct User {
    name: String,
    #[serde(rename = "contributionsCollection")]
    contributions_collection: ContributionsCollection,
}

#[derive(Deserialize, Debug)]
struct ContributionsCollection {
    #[serde(rename = "contributionCalendar")]
    contribution_calendar: ContributionCalendar,
}

#[derive(Deserialize, Debug)]
struct ContributionCalendar {
    colors: Vec<String>,
    #[serde(rename = "totalContributions")]
    total_contributions: i64,
    weeks: Vec<Week>,
}

#[derive(Deserialize, Debug)]
struct Week {
    #[serde(rename = "contributionDays")]
    contribution_days: Vec<ContributionDay>,
    #[serde(rename = "firstDay")]
    first_day: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ContributionDay {
    color: String,
    #[serde(rename = "contributionCount")]
    contribution_count: i64,
    date: String,
    weekday: i64,
}

trait HexToRGB {
    fn get_rgb(&self) -> (u8, u8, u8);
}

impl HexToRGB for String {
    fn get_rgb(&self) -> (u8, u8, u8) {
        let hex = self.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b)
    }
}

async fn get_contributions() -> Result<Data, reqwest::Error> {
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
    let data: Data =
        serde_json::from_value(json_value["data"].clone()).expect("Failed to parse JSON");

    Ok(data)
}

async fn split_contributions_in_weeks(contributions: Data) -> Vec<(String, Vec<ContributionDay>)> {
    let mut contributions_per_week: HashMap<String, Vec<ContributionDay>> = HashMap::new();
    contributions
        .user
        .contributions_collection
        .contribution_calendar
        .weeks
        .iter()
        .for_each(|week| {
            // Put contributions in an hashmap with the first day of the week as the key
            week.contribution_days.iter().for_each(|day| {
                contributions_per_week
                    .entry(week.first_day.clone())
                    .or_insert_with(Vec::new)
                    .push(day.clone());
            });
        });

    let mut contributions_per_week: Vec<_> = contributions_per_week.into_iter().collect();
    contributions_per_week.sort_by(|(date1, _), (date2, _)| date1.cmp(date2));

    contributions_per_week
}

async fn convert_weeks_to_rows(
    contributions_per_week: Vec<(String, Vec<ContributionDay>)>,
) -> Vec<Vec<ContributionDay>> {
    // All first days of the week go in the first row, second days in the second row, etc.
    let mut contributions_per_row: Vec<Vec<ContributionDay>> = vec![vec![]; 7];
    for (_, contributions) in contributions_per_week {
        for (i, contribution) in contributions.iter().enumerate() {
            contributions_per_row[i].push(contribution.clone());
        }
    }

    contributions_per_row
}

// Take a reference to the contributions_per_row to avoid moving it
async fn print_months(contributions_per_row: &Vec<Vec<ContributionDay>>) {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut previous_month = "";
    let mut just_printed_month = false;
    for start_of_week_date in &contributions_per_row[contributions_per_row.len() - 1] {
        let date = &start_of_week_date.date;
        let day: u64 = date.split('-').collect::<Vec<&str>>()[2].parse().unwrap();
        let month = date.split('-').collect::<Vec<&str>>()[1];
        let month = months[month.parse::<usize>().unwrap() - 1];
        if month != previous_month && day < 20 {
            write!(stdout, "{}", month).expect("Failed to write to stdout");
            previous_month = month;
            just_printed_month = true;
            continue;
        }
        if just_printed_month {
            write!(stdout, " ").expect("Failed to write to stdout");
            just_printed_month = false;
        } else {
            write!(stdout, "  ").expect("Failed to write to stdout");
        }
    }
    write!(stdout, "\n").expect("Failed to write to stdout");
}

async fn print_contribution_graph(contributions_per_row: &Vec<Vec<ContributionDay>>) {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    for row in contributions_per_row {
        for contribution in row {
            let rgb = contribution.color.get_rgb();
            stdout
                .set_color(
                    ColorSpec::new().set_fg(Some(termcolor::Color::Rgb(rgb.0, rgb.1, rgb.2))),
                )
                .expect("Failed to set color");

            if rgb.0 > 200 && rgb.1 > 200 && rgb.2 > 200 {
                stdout
                    .set_color(ColorSpec::new().set_dimmed(true))
                    .expect("Failed to set dimmed");
            }
            write!(&mut stdout, "■ ").expect("Failed to write to stdout");
        }
        write!(stdout, "\n").expect("Failed to write to stdout");
    }
}

#[tokio::main]
async fn main() {
    let github_contributions = get_contributions()
        .await
        .expect("Failed to get contributions");

    let github_contributions_per_week = split_contributions_in_weeks(github_contributions).await;
    let github_contributions_per_row = convert_weeks_to_rows(github_contributions_per_week).await;

    print_months(&github_contributions_per_row).await;
    print_contribution_graph(&github_contributions_per_row).await;
}
