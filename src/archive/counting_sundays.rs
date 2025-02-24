fn count_sunday_first() -> u32 {
    let end = NaiveDate::from_ymd_opt(2000, 12, 31).unwrap();
    let mut date = NaiveDate::from_ymd_opt(1901, 1, 6).unwrap();
    let mut count = 0;
    while date <= end {
        date = date.checked_add_days(Days::new(7)).unwrap();
        if date.day() == 1 {
            count += 1;
        }
    }

    count
}

use chrono::{Datelike, Days, NaiveDate};
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        println!("Solution 19: {}", count_sunday_first());
    }

    #[test]
    fn test_start_date() {
        let date = NaiveDate::from_ymd_opt(1900, 1, 7).unwrap().weekday();
        println!("{}", date);
    }

    #[test]
    fn test_leap_years() {
        let date = NaiveDate::from_ymd_opt(2000, 2, 29).unwrap();
        println!("{}", date);
    }
}
