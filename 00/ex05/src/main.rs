fn is_leap_year(year: u32) -> bool {
    if year == 0 {
        panic!()
    }
    year % 4 == 0
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    if month > 12 || month < 1 {
        panic!();
    } else if month == 2 && is_leap_year(year) {
        29
    } else if month == 2 && !is_leap_year(year) {
        28
    } else if (month < 8 && month % 2 == 1) || (month > 7 && month % 2 == 0) {
        31
    } else {
        30
    }
}

fn month_to_str(month: u32) -> String {
    match month {
        1 => "January".to_string(),
        2 => "February".to_string(),
        3 => "March".to_string(),
        4 => "April".to_string(),
        5 => "May".to_string(),
        6 => "June".to_string(),
        7 => "July".to_string(),
        8 => "August".to_string(),
        9 => "September".to_string(),
        10 => "October".to_string(),
        11 => "November".to_string(),
        12 => "December".to_string(),
        _ => "Error".to_string(),
    }
}

fn main() {
    let mut weekday: u32 = 1;
    let mut day: u32 = 1;
    let mut month: u32 = 1;
    let mut year: u32 = 1;
    while year < 2026 {
        while month <= 12 {
            while day <= num_days_in_month(year, month) {
                if weekday == 5 && day == 13 {
                    println!("Friday, {} 13, {}", month_to_str(month), year);
                }
                day += 1;
                if weekday == 7 {
                    weekday = 1;
                } else {
                    weekday += 1;
                }
            }
            day = 1;
            month += 1;
        }
        month = 1;
        year += 1;
    }
}

#[test]
fn test_leap_year() -> () {
    assert!(is_leap_year(1600));
    assert!(is_leap_year(1500));
    assert!(is_leap_year(2004));
    assert!(!is_leap_year(2003));
}

#[test]
fn test_nb_days_in_month() -> () {
    assert_eq!(num_days_in_month(1600, 2), 29);
    assert_eq!(num_days_in_month(1601, 2), 28);
    assert_eq!(num_days_in_month(1600, 4), 30);
    assert_eq!(num_days_in_month(1601, 4), 30);
}

#[test]
#[should_panic]
fn test_panic_invalid_month() {
    num_days_in_month(2000, 0);
}
#[test]
#[should_panic]
fn test_panic_leap_year_zero() {
    is_leap_year(0);
}
