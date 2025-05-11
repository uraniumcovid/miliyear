use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Timelike, Utc};
use crate::types::MilliyearDate;

const SECONDS_PER_YEAR: f64 = 31_557_600.0; // 365.25 days
const SECONDS_PER_MILLIYEAR: f64 = SECONDS_PER_YEAR / 1000.0;

/// Convert a DateTime to milliyear format
pub fn datetime_to_milliyear<Tz: TimeZone>(dt: DateTime<Tz>) -> MilliyearDate {
    // Assuming March 1st as start of year
    let year = if dt.month() < 3 {
        dt.year() - 1
    } else {
        dt.year()
    };
    
    // Calculate days since March 1st
    let year_start = NaiveDate::from_ymd_opt(year, 3, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    
    let duration = dt.naive_utc().signed_duration_since(year_start);
    let seconds_elapsed = duration.num_seconds() as f64;
    
    // Convert to milliyears
    let milliyear = seconds_elapsed / SECONDS_PER_MILLIYEAR;
    
    MilliyearDate::new(year, milliyear)
}

/// Convert milliyear back to DateTime
pub fn milliyear_to_datetime(my_date: MilliyearDate) -> DateTime<Utc> {
    let year_start = NaiveDate::from_ymd_opt(my_date.year, 3, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    
    let seconds_to_add = (my_date.milliyear * SECONDS_PER_MILLIYEAR) as i64;
    let naive_dt = year_start + chrono::Duration::seconds(seconds_to_add);
    
    DateTime::from_naive_utc_and_offset(naive_dt, Utc)
}