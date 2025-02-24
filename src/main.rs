use std::collections::HashMap;

use colors::{GITHUB_COLORS, HALLOWEEN_COLORS, LIME_COLORS, MOON_COLORS, PSYCHEDELIC_COLORS};
use std::io::Write;
use termcolor::{ColorSpec, StandardStream, WriteColor};

mod colors;
mod github_contributions;
mod gitlab_contributions;

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

async fn convert_weeks_to_rows(
    contribution_collections: Vec<ContributionCollection>,
) -> (Vec<Vec<Option<ContributionDay>>>, i64) {
    // All first days of the week go in the first row, second days in the second row, etc.
    let mut contributions_per_row: Vec<Vec<Option<ContributionDay>>> = vec![vec![]; 7];
    let mut max_contributions = 0;
    for contribution_collection in contribution_collections {
        for (weeknumber, contributions) in contribution_collection.contributions {
            println!("Weeknumber: {}", weeknumber);
            for (day, contribution) in contributions.iter().enumerate() {
                while contributions_per_row[day].len() <= weeknumber as usize {
                    contributions_per_row[day].push(None);
                }
                // If contributions_per_row[day][weeknumber] is None, set it to contribution
                if let Some(contribution_day) = &contributions_per_row[day][weeknumber as usize] {
                    // Update contribution count
                    contributions_per_row[day][weeknumber as usize] = Some(ContributionDay {
                        contribution_count: contribution_day.contribution_count
                            + contribution.contribution_count,
                        date: contribution_day.date.clone(),
                        weeknumber: contribution_day.weeknumber,
                        weekday: contribution_day.weekday,
                    });
                } else {
                    contributions_per_row[day][weeknumber as usize] = Some(contribution.clone());
                }

                if contribution.contribution_count > max_contributions {
                    max_contributions = contribution.contribution_count;
                }
            }
        }
    }
    (contributions_per_row, max_contributions)
}

async fn print_months(contributions_per_row: &Vec<Vec<Option<ContributionDay>>>) {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut previous_month = "";
    let mut just_printed_month = false;
    for start_of_week_date in &contributions_per_row[contributions_per_row.len() - 1] {
        let date = &start_of_week_date.clone().unwrap().date;
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

async fn print_contribution_graph(
    contributions_per_row: &Vec<Vec<Option<ContributionDay>>>,
    max_contributions: i64,
) {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    for row in contributions_per_row {
        for contribution in row {
            // Color index between 0 and 5 (based on max_contributions)
            let color_index = contribution.as_ref().map_or(0, |c| {
                (c.contribution_count as f64 / max_contributions as f64
                    * (GITHUB_COLORS.len() - 1) as f64)
                    .ceil() as usize
            });
            let rgb = GITHUB_COLORS[color_index].to_string().get_rgb();
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
    let github_contributions = github_contributions::get_github_contributions()
        .await
        .expect("Failed to get GitHub contributions");
    let gitlab_contributions = gitlab_contributions::get_gitlab_contributions()
        .await
        .expect("Failed to get GitLab contributions");

    let contribution_collections = vec![github_contributions, gitlab_contributions];
    let (contributions_per_row, max_contributions) =
        convert_weeks_to_rows(contribution_collections).await;

    print_months(&contributions_per_row).await;
    print_contribution_graph(&contributions_per_row, max_contributions).await;
}
