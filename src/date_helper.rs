use chrono::{DateTime, Local, TimeZone};

pub fn from_datetime_picker_to_chrono_date(year: i32, month: u32, day: u32) -> DateTime<Local> {
    let datetime: DateTime<Local> = Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
    datetime
}
