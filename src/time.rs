use crate::constants::{
    DAYS_IN_MONTH, DAYS_PER_100Y, DAYS_PER_400Y, DAYS_PER_4Y, LEAPOCH, MONTHS, SP, WEEKS,
};
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Time {
    min: i64,
    sec: i64,
    hour: i64,
    week: i64,
    year: i64,
    month: i64,
    day: i64,
}

trait Padding {
    fn pad_zero(self) -> String;
}

impl Padding for i64 {
    fn pad_zero(self) -> String {
        let day_string = format!("{}{}", 0, self);
        (&day_string[day_string.len() - 2..]).to_owned()
    }
}

impl Time {
    pub fn now() -> Option<Self> {
        let stamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => {
                if let Ok(x) = n.as_secs().try_into() {
                    x
                } else {
                    return None;
                }
            }
            Err(m) => panic!("Got time error: {}", m),
        };
        Self::time(stamp)
    }
    pub fn format(&self) -> String {
        if let Some(month_day) = self.month_day() {
            if let Some(week_day) = self.week_day() {
                let date = format!(
                    "{}{}{}{}{}",
                    self.day.pad_zero(),
                    SP,
                    month_day,
                    SP,
                    self.year
                );
                let time = format!(
                    "{}:{}:{}",
                    self.hour.pad_zero(),
                    self.min.pad_zero(),
                    self.sec.pad_zero()
                );
                format!("{},{}{}{}{}{}GMT", week_day, SP, date, SP, time, SP)
            } else {
                "".to_owned()
            }
        } else {
            "".to_owned()
        }
    }
    pub fn with_stamp(self, stamp: i64) -> Option<Self> {
        Self::time(stamp)
    }
    fn week_day(&self) -> Option<String> {
        if let Some(week) = WEEKS.iter().enumerate().find(|(i, _)| {
            if let Ok(week_day) = self.week.try_into() {
                *i == week_day
            } else {
                false
            }
        }) {
            Some(week.1.to_string())
        } else {
            None
        }
    }
    fn month_day(&self) -> Option<String> {
        if let Some(month) = MONTHS.iter().enumerate().find(|(i, _)| {
            if let Ok(month_day) = self.month.try_into() {
                *i == month_day
            } else {
                false
            }
        }) {
            Some(month.1.to_string())
        } else {
            None
        }
    }

    fn time(stamp: i64) -> Option<Self> {
        let secs = (stamp) - LEAPOCH;
        let mut days = secs / 86400;
        let mut remsecs = secs % 86400;
        if remsecs < 0 {
            remsecs += 86400;
            days -= 1;
        }
        let mut wday = (days + 3) % 7;
        if wday < 0 {
            wday += 7;
        }
        let mut qc_cycles = days / (DAYS_PER_400Y);
        let mut remdays = days % (DAYS_PER_400Y);
        if remdays < 0 {
            remdays += DAYS_PER_400Y;
            qc_cycles -= 1;
        }
        let mut c_cycles = remdays / DAYS_PER_100Y;
        if c_cycles == 4 {
            c_cycles -= 1;
        }
        remdays -= c_cycles * DAYS_PER_100Y;
        let mut q_cycles = remdays / DAYS_PER_4Y;
        if q_cycles == 25 {
            q_cycles -= 1;
        }
        remdays -= q_cycles * DAYS_PER_4Y;
        let mut remyears = remdays / 365;
        if remyears == 4 {
            remyears -= 1;
        }
        remdays -= remyears * 365;
        let years = remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;
        let mut month: usize = 0;
        while DAYS_IN_MONTH[month] <= remdays {
            remdays -= DAYS_IN_MONTH[month];
            month += 1;
        }
        let mut final_year = years + 100;
        let mut final_mon = (month as i64) + 2;
        if final_mon >= 12 {
            final_mon -= 12;
            final_year += 1;
        }
        Some(Time {
            min: (remsecs / 60) % 60,
            sec: remsecs % 60,
            hour: remsecs / 3600,
            week: wday,
            day: remdays + 1,
            year: 1900 + final_year,
            month: final_mon,
        })
    }
}
