#[allow(dead_code)]
#[derive(Debug)]
pub struct ContributionCollection {
    pub provider: String,
    pub contributions: Vec<(i64, Vec<ContributionDay>)>,
    pub max_contributions: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ContributionDay {
    pub contribution_count: i64,
    pub date: String,
    pub weeknumber: i64,
    pub weekday: i64,
}
