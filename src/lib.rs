// persistent event => persistent state

// transient event => transient state

use chrono::{DateTime, Datelike, Duration, NaiveTime, TimeZone, Timelike, Weekday};
use std::collections::BTreeSet;

/// Internal Weekday representation ordered by day in week.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum OrderedWeekday {
    /// Monday.
    Mon = 0,
    /// Tuesday.
    Tue = 1,
    /// Wednesday.
    Wed = 2,
    /// Thursday.
    Thu = 3,
    /// Friday.
    Fri = 4,
    /// Saturday.
    Sat = 5,
    /// Sunday.
    Sun = 6,
}

impl From<Weekday> for OrderedWeekday {
    fn from(w: Weekday) -> Self {
        use OrderedWeekday::*;
        match w {
            Weekday::Mon => Mon,
            Weekday::Tue => Tue,
            Weekday::Wed => Wed,
            Weekday::Thu => Thu,
            Weekday::Fri => Fri,
            Weekday::Sat => Sat,
            Weekday::Sun => Sun,
        }
    }
}

impl DurationTo for OrderedWeekday {
    fn duration_to(&self, next: OrderedWeekday) -> Duration {
        let current = *self as isize;
        let next = next as isize;
        let mut duration = next - current;
        if duration < 1 {
            duration = duration + 7;
        }
        Duration::days(duration as i64)
    }
}

pub trait DurationTo {
    fn duration_to(&self, next: Self) -> Duration;
}

fn apply_time<T: TimeZone>(date_time: &DateTime<T>, time: &NaiveTime) -> DateTime<T> {
    date_time
        .with_hour(time.hour())
        .unwrap()
        .with_minute(time.minute())
        .unwrap()
        .with_second(time.second())
        .unwrap()
}

impl Weekdays for Weekday {
    fn week_days(&self) -> Vec<Weekday> {
        vec![*self]
    }
}
impl Weekdays for (Weekday, Weekday) {
    fn week_days(&self) -> Vec<Weekday> {
        vec![self.0, self.1]
    }
}
impl Weekdays for (Weekday, Weekday, Weekday) {
    fn week_days(&self) -> Vec<Weekday> {
        vec![self.0, self.1, self.2]
    }
}
impl Weekdays for (Weekday, Weekday, Weekday, Weekday) {
    fn week_days(&self) -> Vec<Weekday> {
        vec![self.0, self.1, self.2, self.3]
    }
}
impl Weekdays for (Weekday, Weekday, Weekday, Weekday, Weekday) {
    fn week_days(&self) -> Vec<Weekday> {
        vec![self.0, self.1, self.2, self.3, self.4]
    }
}
impl Weekdays for (Weekday, Weekday, Weekday, Weekday, Weekday, Weekday) {
    fn week_days(&self) -> Vec<Weekday> {
        vec![self.0, self.1, self.2, self.3, self.4, self.5]
    }
}
impl Weekdays
    for (
        Weekday,
        Weekday,
        Weekday,
        Weekday,
        Weekday,
        Weekday,
        Weekday,
    )
{
    fn week_days(&self) -> Vec<Weekday> {
        vec![self.0, self.1, self.2, self.3, self.4, self.5, self.6]
    }
}

pub trait Weekdays {
    fn week_days(&self) -> Vec<Weekday>;
}
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("At least one WeekDay must be provided")]
    NoWeekDay,
}

/// Something Recurrent week to week
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recurrence<T>
where
    T: DurationTo,
{
    days: BTreeSet<T>,
    time: NaiveTime,
}

impl Recurrence<OrderedWeekday> {
    pub fn new<T: Weekdays>(days: T, time: NaiveTime) -> Result<Self, Error> {
        let days = days.week_days();
        if days.len() == 0 {
            Err(Error::NoWeekDay)
        } else {
            Ok(Recurrence {
                time,
                days: days.iter().map(|d| (*d).into()).collect(),
            })
        }
    }

    pub fn next<T: TimeZone>(&self, date: &DateTime<T>) -> DateTime<T> {
        let current_day: OrderedWeekday = date.weekday().into();

        let days_to_add = if self.days.contains(&current_day) && date.time() < self.time {
            // next is current day :)
            Duration::days(0)
        } else {
            // need to grab next "weekday"
            let next_week_day = self
                .days
                .iter()
                .find(|day| *day > &current_day)
                .unwrap_or(self.days.iter().find(|_| true).unwrap()); // loop to the first
            current_day.duration_to(*next_week_day)
        };
        apply_time(&(date.clone() + days_to_add), &self.time)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Recurrence, Weekday, Weekdays};
    use chrono::{DateTime, NaiveTime, Utc};

    #[test]
    fn test() {
        // today
        test_next(
            "2020-08-30T14:15:16Z",
            Weekday::Sun,
            (15, 0, 0),
            "2020-08-30T15:00:00Z",
        );
        // next sunday
        test_next(
            "2020-08-30T14:15:16Z",
            Weekday::Sun,
            (14, 0, 0),
            "2020-09-06T14:00:00Z",
        );
        test_next(
            "2020-08-30T14:15:16Z",
            (Weekday::Sun, Weekday::Mon),
            (14, 0, 0),
            "2020-08-31T14:00:00Z",
        );
        test_next(
            "2020-08-30T14:15:16Z",
            (Weekday::Tue, Weekday::Mon),
            (14, 0, 0),
            "2020-08-31T14:00:00Z",
        );
        test_next(
            "2020-08-30T14:15:16Z",
            (Weekday::Tue, Weekday::Fri),
            (14, 0, 0),
            "2020-09-01T14:00:00Z",
        );
    }

    fn test_next<T: Weekdays>(now: &str, day: T, (h, m, s): (u32, u32, u32), expect: &str) {
        // Sunday
        let now: DateTime<Utc> = now.parse().unwrap();
        let w = Recurrence::new(day, NaiveTime::from_hms(h, m, s))
            .unwrap()
            .next(&now);
        let e: DateTime<Utc> = expect.parse().unwrap();
        assert_eq!(w, e);
    }
}
