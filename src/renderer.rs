use crate::types::ContributionDay;
use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};
use time::OffsetDateTime;

pub trait HexToRGB {
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

pub struct ContributionGraphRenderer {
    pub stdout: StandardStream,
    pub color_scheme: Vec<String>,
}

impl ContributionGraphRenderer {
    pub fn new(color_scheme: Vec<String>) -> Self {
        Self {
            stdout: StandardStream::stdout(ColorChoice::Always),
            color_scheme,
        }
    }

    pub fn render_months(
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

    pub fn render_graph(
        &mut self,
        contributions_per_row: &[Vec<Option<ContributionDay>>],
        max_contributions: i64,
    ) -> std::io::Result<()> {
        for row in contributions_per_row {
            for contribution in row {
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

                // Dim the color if it's light
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
