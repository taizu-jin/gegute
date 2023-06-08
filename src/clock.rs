use chrono::{DateTime, Local, TimeZone};

pub struct Clock;

impl Clock {
    pub fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(not(windows))]
    pub fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use libc::{settimeofday, timezone};
        use libc::{suseconds_t, time_t, timeval};
        use std::mem::zeroed;

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}
