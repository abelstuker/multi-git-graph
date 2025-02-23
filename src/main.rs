use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};

/*
Example response from the GitHub API
[{"id":"46753292831","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224901,"name":"refspace/refspace-website","url":"https://api.github.com/repos/refspace/refspace-website"},"payload":{"repository_id":934224901,"push_id":22752057562,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"90af9d1a9ee02ffb59f15652010b3f57e7cce86d","before":"b9d9e6698ae7e4255d2c6cb91a34cc2ad6d5123f","commits":[{"sha":"90af9d1a9ee02ffb59f15652010b3f57e7cce86d","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"chore: tailwind prettier","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-website/commits/90af9d1a9ee02ffb59f15652010b3f57e7cce86d"}]},"public":false,"created_at":"2025-02-19T22:56:44Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46649369786","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224901,"name":"refspace/refspace-website","url":"https://api.github.com/repos/refspace/refspace-website"},"payload":{"repository_id":934224901,"push_id":22699762953,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"8198e832ebc4cea4385db41bb1e650887d9cd62c","before":"4623de64ed7a57921d3eb09dfd66e4fdcabbba71","commits":[{"sha":"8198e832ebc4cea4385db41bb1e650887d9cd62c","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"feat: basic login functionality","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-website/commits/8198e832ebc4cea4385db41bb1e650887d9cd62c"}]},"public":false,"created_at":"2025-02-17T13:34:19Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46649350519","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224901,"name":"refspace/refspace-website","url":"https://api.github.com/repos/refspace/refspace-website"},"payload":{"ref":"main","ref_type":"branch","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-17T13:33:50Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46649329993","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224901,"name":"refspace/refspace-frontend","url":"https://api.github.com/repos/refspace/refspace-frontend"},"payload":{"ref":null,"ref_type":"repository","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-17T13:33:19Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46649278888","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224223,"name":"AbelStuker/refspace-website","url":"https://api.github.com/repos/AbelStuker/refspace-website"},"payload":{"ref":"main","ref_type":"branch","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-17T13:31:59Z"},{"id":"46649277697","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":934224223,"name":"AbelStuker/refspace-website","url":"https://api.github.com/repos/AbelStuker/refspace-website"},"payload":{"ref":null,"ref_type":"repository","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-17T13:31:57Z"},{"id":"46649172124","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"repository_id":933686663,"push_id":22699666570,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"ea86f6c756ed02cdc6eab6757204397c75bcb770","before":"87dc419a08521021b6c71b149652304d83e9376a","commits":[{"sha":"ea86f6c756ed02cdc6eab6757204397c75bcb770","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"feat: auth jwt as http-only cookie","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-backend/commits/ea86f6c756ed02cdc6eab6757204397c75bcb770"}]},"public":false,"created_at":"2025-02-17T13:29:19Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46627492153","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"repository_id":933686663,"push_id":22688717487,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"87dc419a08521021b6c71b149652304d83e9376a","before":"520392dc7a0f5f41abcf45fc23f4fd08c588f1e2","commits":[{"sha":"87dc419a08521021b6c71b149652304d83e9376a","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"feat: auth data validation","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-backend/commits/87dc419a08521021b6c71b149652304d83e9376a"}]},"public":false,"created_at":"2025-02-17T00:27:26Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46621611470","type":"PushEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"repository_id":933686663,"push_id":22684768533,"size":1,"distinct_size":1,"ref":"refs/heads/main","head":"520392dc7a0f5f41abcf45fc23f4fd08c588f1e2","before":"a6efae33de187448219f51d27e740536c8fb2c9d","commits":[{"sha":"520392dc7a0f5f41abcf45fc23f4fd08c588f1e2","author":{"email":"stukerabel@gmail.com","name":"Abel Stuker"},"message":"feat: passport auth","distinct":true,"url":"https://api.github.com/repos/refspace/refspace-backend/commits/520392dc7a0f5f41abcf45fc23f4fd08c588f1e2"}]},"public":false,"created_at":"2025-02-16T15:36:20Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46619980317","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"ref":"main","ref_type":"branch","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-16T13:19:48Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46619947467","type":"CreateEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":933686663,"name":"refspace/refspace-backend","url":"https://api.github.com/repos/refspace/refspace-backend"},"payload":{"ref":null,"ref_type":"repository","master_branch":"main","description":null,"pusher_type":"user"},"public":false,"created_at":"2025-02-16T13:17:06Z","org":{"id":199481862,"login":"refspace","gravatar_id":"","url":"https://api.github.com/orgs/refspace","avatar_url":"https://avatars.githubusercontent.com/u/199481862?"}},{"id":"46611886347","type":"WatchEvent","actor":{"id":62062732,"login":"AbelStuker","display_login":"AbelStuker","gravatar_id":"","url":"https://api.github.com/users/AbelStuker","avatar_url":"https://avatars.githubusercontent.com/u/62062732?"},"repo":{"id":117118012,"name":"danleh/wasabi","url":"https://api.github.com/repos/danleh/wasabi"},"payload":{"action":"started"},"public":true,"created_at":"2025-02-15T22:24:14Z"}]
*/

#[derive(Debug, Deserialize, Clone)]
struct GithubEvent {
    id: String,
    r#type: Option<String>,
    actor: Actor,
    repo: Repo,
    payload: Payload,
    public: bool,
    created_at: Option<String>,
    org: Option<Org>,
}

#[derive(Debug, Deserialize, Clone)]
struct Actor {
    id: i64,
    login: Option<String>,
    display_login: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    avatar_url: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
struct Repo {
    id: i64,
    name: Option<String>,
    url: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
struct Payload {
    ref_type: Option<String>,
    ref_: Option<String>,
    master_branch: Option<String>,
    description: Option<String>,
    pusher_type: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
struct Org {
    id: i64,
    login: String,
    gravatar_id: String,
    url: String,
    avatar_url: String,
}

async fn get_github_events() -> Result<Vec<GithubEvent>, reqwest::Error> {
    let client: Client = Client::new();

    let github_token = dotenv::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found");
    let username = dotenv::var("GITHUB_USERNAME").expect("GITHUB_USERNAME not found");

    let res = client
        .get(format!("https://api.github.com/users/{}/events", username))
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", github_token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "All Contributions CLI")
        .send()
        .await?;

    let body = res.text().await?;

    let events: Vec<GithubEvent> = serde_json::from_str(&body).expect("Failed to parse JSON");
    Ok(events)
}

#[tokio::main]
async fn main() {
    let github_events: Vec<GithubEvent> = get_github_events()
        .await
        .expect("Failed to get github events");
    let mut events_by_day: HashMap<String, Vec<GithubEvent>> = HashMap::new();
    // Order the events in buckets per day
    // Fill the buckets
    github_events.iter().for_each(|event| {
        // Get the bucket at the date
        let bucket = events_by_day
            .entry(
                event
                    .created_at
                    .clone()
                    .unwrap()
                    .split("T")
                    .collect::<Vec<&str>>()[0]
                    .to_string(),
            )
            .or_insert(Vec::new());
        // Push the event to the bucket
        bucket.push(event.clone());
    });
    // Print the buckets
    events_by_day.iter().for_each(|(date, events)| {
        println!("Date: {}", date);
        events.iter().for_each(|event| {
            println!("{:?}", event);
        });
    });
}
