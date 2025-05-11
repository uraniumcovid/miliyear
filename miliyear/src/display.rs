use crate::types::MilliyearDate;

impl std::fmt::Display for MilliyearDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:.3}", self.year, self.milliyear)
    }
}

pub fn format_milliyear(my_date: &MilliyearDate, precision: usize) -> String {
    match precision {
        0 => format!("{}-{:.0}", my_date.year, my_date.milliyear),
        _ => format!("{}-{:.*}", my_date.year, precision, my_date.milliyear),
    }
}