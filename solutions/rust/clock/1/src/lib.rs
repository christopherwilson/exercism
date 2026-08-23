use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (extra_hours, minutes) = Self::mins_to_hrs_mins(minutes);

        let hours = (hours + extra_hours).rem_euclid(24);
        
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (extra_hours, extra_minutes) = Self::mins_to_hrs_mins(minutes);
    
        Clock::new(self.hours + extra_hours, self.minutes + extra_minutes)
    }

    /// Converts minutes to hours and minutes.
    /// 
    /// Returns the tuple `(hours, minutes)`. `minutes` will always be positive.
    fn mins_to_hrs_mins(minutes: i32) -> (i32, i32) {
        if minutes == 0 {
            return (0, 0)
        }

        let remainder = minutes % 60;

        if minutes > 0 || remainder == 0 {
            (minutes / 60, remainder)
        } else {
            // we want to always return positive minutes, so we subtract an extra hour and add it to
            // the minutes
            ((minutes / 60) - 1, 60 + remainder)
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mins_to_hrs_mins_zero() {
        assert_eq!(Clock::mins_to_hrs_mins(0), (0, 0));
    }

    #[test]
    fn mins_to_hrs_mins_fifty_five() {
        assert_eq!(Clock::mins_to_hrs_mins(55), (0, 55));
    }

    #[test]
    fn mins_to_hrs_mins_sixty() {
        assert_eq!(Clock::mins_to_hrs_mins(60), (1, 0));
    }

    #[test]
    fn mins_to_hrs_mins_thousand() {
        assert_eq!(Clock::mins_to_hrs_mins(1000), (16, 40));
    }

    #[test]
    fn mins_to_hrs_mins_neg_five() {
        assert_eq!(Clock::mins_to_hrs_mins(-5), (-1, 55));
    }

    #[test]
    fn mins_to_hrs_mins_neg_sixty() {
        assert_eq!(Clock::mins_to_hrs_mins(-60), (-1, 0));
    }

    #[test]
    fn mins_to_hrs_mins_neg_thousand() {
        assert_eq!(Clock::mins_to_hrs_mins(-1000), (-17, 20));
    }
}
