use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

mod colors;
mod github_contributions;
mod gitlab_contributions;

use colors::ColorScheme;
use time::{Duration, OffsetDateTime, UtcOffset};

// Core domain types
#[derive(Debug)]
pub struct ContributionCollection {
    provider: String,
    contributions: Vec<(i64, Vec<ContributionDay>)>,
    max_contributions: i64,
}

#[derive(Debug, Clone)]
pub struct ContributionDay {
    contribution_count: i64,
    date: String,
    weeknumber: i64,
    weekday: i64,
}

// Color handling
trait HexToRGB {
    fn to_rgb(&self) -> (u8, u8, u8);
}

impl HexToRGB for String {
    fn to_rgb(&self) -> (u8, u8, u8) {
        let hex = self.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        (r, g, b)
    }
}

// Contribution graph renderer
struct ContributionGraphRenderer {
    stdout: StandardStream,
    color_scheme: Vec<String>,
}

impl ContributionGraphRenderer {
    fn new(color_scheme: Vec<String>) -> Self {
        Self {
            stdout: StandardStream::stdout(ColorChoice::Always),
            color_scheme,
        }
    }

    fn render_months(
        &mut self,
        contributions_per_row: &[Vec<Option<ContributionDay>>],
    ) -> std::io::Result<()> {
        const MONTHS: [&str; 12] = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];

        let mut previous_month = "";
        let mut just_printed_month = false;

        if let Some(row) = contributions_per_row.last() {
            for contribution in row {
                if let Some(day) = contribution {
                    if let Some(contribution_day) = contribution {
                        if contribution_day.date == row.last().unwrap().as_ref().unwrap().date
                            && contribution_day.weekday
                                > OffsetDateTime::now_utc()
                                    .weekday()
                                    .number_days_from_sunday()
                                    as i64
                        {
                            break;
                        }
                    }

                    let date_parts: Vec<&str> = day.date.split('-').collect();
                    if date_parts.len() >= 3 {
                        let month_idx = date_parts[1].parse::<usize>().unwrap_or(1) - 1;
                        let day = date_parts[2].parse::<u64>().unwrap_or(0);
                        let month = MONTHS[month_idx];

                        if month != previous_month && day < 20 {
                            write!(self.stdout, "{}", month)?;
                            previous_month = month;
                            just_printed_month = true;
                            continue;
                        }
                    }
                }
                write!(
                    self.stdout,
                    "{}",
                    if just_printed_month { " " } else { "  " }
                )?;
                just_printed_month = false;
            }
            writeln!(self.stdout)?;
        }
        Ok(())
    }

    fn render_graph(
        &mut self,
        contributions_per_row: &[Vec<Option<ContributionDay>>],
        max_contributions: i64,
    ) -> std::io::Result<()> {
        for row in contributions_per_row {
            for contribution in row {
                // Stop when we are in the last column and the row (corresponding to the weekday) is after the current weekday
                if let Some(contribution) = contribution {
                    if contribution.date == row.last().unwrap().as_ref().unwrap().date
                        && contribution.weekday
                            > OffsetDateTime::now_utc()
                                .weekday()
                                .number_days_from_sunday() as i64
                    {
                        break;
                    }
                }
                let color_index = contribution.as_ref().map_or(0, |c| {
                    ((c.contribution_count as f64 / max_contributions as f64)
                        * (self.color_scheme.len() - 1) as f64)
                        .ceil() as usize
                });

                let rgb = self.color_scheme[color_index].to_rgb();
                self.stdout.set_color(
                    ColorSpec::new().set_fg(Some(termcolor::Color::Rgb(rgb.0, rgb.1, rgb.2))),
                )?;

                if rgb.0 > 200 && rgb.1 > 200 && rgb.2 > 200 {
                    self.stdout.set_color(ColorSpec::new().set_dimmed(true))?;
                }

                write!(self.stdout, "■ ")?;
            }
            writeln!(self.stdout)?;
        }
        Ok(())
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
    // Make sure start-date is always a sunday !! the sunday before the day 365 days ago
    let start_date = end_date
        .checked_sub(Duration::days(365))
        .expect("Failed to subtract 365 days");
    let start_date = start_date
        .checked_sub(Duration::days(
            start_date.weekday().number_days_from_sunday() as i64,
        ))
        .expect("Failed to subtract days to get to sunday");

    // Fetch contributions. Allow any of these to fail, then we'll just skip them
    let contributions: Vec<Option<ContributionCollection>> = vec![
        github_contributions::get_github_contributions(start_date, end_date)
            .await
            .ok(),
        gitlab_contributions::get_gitlab_contributions(start_date, end_date)
            .await
            .ok(),
    ];

    // Process data
    let (contributions_per_row, max_contributions) = process_contributions(contributions).await;

    // Render visualization
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
