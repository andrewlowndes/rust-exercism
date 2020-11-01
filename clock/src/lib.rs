use std::fmt;

const MAX_HOURS: i32 = 24;
const HOUR_MINUTES: i32 = 60;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * HOUR_MINUTES + minutes;
        let hours = total_minutes.div_euclid(HOUR_MINUTES).rem_euclid(MAX_HOURS);
        let minutes = total_minutes.rem_euclid(HOUR_MINUTES);

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
