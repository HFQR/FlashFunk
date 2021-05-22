/// Here we support the time
/// todo: make me more fast

use chrono::{NaiveDateTime, FixedOffset, Utc, NaiveDate, Date, Local};


pub fn get_china_time() -> NaiveDateTime {
    Utc::now().with_timezone(&FixedOffset::east(8 * 3600)).naive_local()
}

pub fn get_china_date() -> NaiveDate {
    Utc::today().with_timezone(&FixedOffset::east(8 * 3600)).naive_local()
}

