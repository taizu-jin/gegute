use chrono::{DateTime, Local, TimeZone};

pub struct Clock;

impl Clock {
    pub fn get() -> DateTime<Local> {
        Local::now()
    }
}
