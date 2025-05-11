use chrono::{DateTime, Datelike, TimeZone, Utc};
use crate::conversion;  // this is what rust suggests in the error message

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MilliyearDate {
    pub year: i32,
    pub milliyear: f64,
}

impl MilliyearDate {
    pub fn new(year: i32, milliyear: f64) -> Self {
        Self { year, milliyear }
    }
    
    pub fn now() -> Self {
        conversion::datetime_to_milliyear(Utc::now())
    }
    
    // Add more methods as needed
}