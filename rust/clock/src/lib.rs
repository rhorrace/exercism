#[derive(Debug)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes).rem_euclid(1440);
        Clock {
            hour: (total_minutes / 60).rem_euclid(24),
            minute: total_minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hour, self.minute + minutes)
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hour == other.hour && self.minute == other.minute
    }

    fn ne(&self, other: &Clock) -> bool {
        self.hour != other.hour || self.minute != other.minute
    }
}