use crate::{OrderedWeekday, Weekdays};
use chrono::Weekday;

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

impl Weekdays for &[Weekday] {
    fn week_days(&self) -> Vec<Weekday> {
        self.iter().map(|w| w.clone()).collect()
    }
}
