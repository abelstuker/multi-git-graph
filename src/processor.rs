use crate::{ContributionDay, ProviderError};
use time::{Duration, OffsetDateTime};

pub struct ContributionProcessor {
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
}

impl ContributionProcessor {
    pub fn new(start_date: OffsetDateTime, end_date: OffsetDateTime) -> Self {
        Self {
            start_date,
            end_date,
        }
    }

    pub fn initialize_contribution_calendar(&self) -> Vec<(i64, Vec<ContributionDay>)> {
        let mut contributions = Vec::new();
        let mut current_date = self.start_date;
        let mut week_number = 0;

        while current_date < self.end_date {
            let mut week_days = Vec::new();

            for _ in 0..7 {
                week_days.push(ContributionDay {
                    contribution_count: 0,
                    weeknumber: week_number,
                    date: format!(
                        "{}-{:02}-{:02}",
                        current_date.year(),
                        current_date.month() as u8,
                        current_date.day()
                    ),
                    weekday: current_date.weekday().number_days_from_sunday() as i64,
                });
                current_date = current_date
                    .checked_add(Duration::days(1))
                    .expect("Failed to increment date");
            }

            contributions.push((week_number, week_days));
            week_number += 1;
        }

        contributions
    }

    pub fn process_events<T: Event + std::fmt::Debug>(
        &self,
        events: Vec<T>,
        mut calendar: Vec<(i64, Vec<ContributionDay>)>,
    ) -> Result<(Vec<(i64, Vec<ContributionDay>)>, i64), ProviderError> {
        let mut max_contributions = 0;

        for event in events {
            let event_date = event.timestamp()?;
            if event_date < self.start_date || event_date >= self.end_date {
                continue;
            }

            let days_since_start = (event_date - self.start_date).whole_days();
            let week_number = (days_since_start / 7) as usize;
            let day_number = (days_since_start % 7) as usize;

            if week_number >= calendar.len() || day_number >= calendar[week_number].1.len() {
                continue; // Skip invalid indices
            }

            let contribution = &mut calendar[week_number].1[day_number];
            contribution.contribution_count += event.contributions();
            max_contributions = max_contributions.max(contribution.contribution_count);
        }

        Ok((calendar, max_contributions))
    }
}

pub trait Event {
    fn timestamp(&self) -> Result<OffsetDateTime, ProviderError>;
    fn contributions(&self) -> i64;
}
