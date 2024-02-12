#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub date: u8,
}

impl Date {
    pub fn from(date_str: &str) -> Self {
        let date = date_str[8..].parse().unwrap();
        let month = date_str[5..7].parse().unwrap();
        let year = date_str[..4].parse().unwrap();

        return Date { date, month, year };
    }

    pub fn new() -> Self {
        return Date {
            date: 0,
            month: 0,
            year: 0,
        };
    }
}

pub type Graph = Vec<Vec<usize>>;

pub const MAX_NODES: usize = 34546;
