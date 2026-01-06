fn is_leap_year(year: u32) -> bool {
    if year == 0 {
        panic!("Invalid year");
    }
    if year % 4 == 0 {
        if year % 100 == 0 && year % 400 != 0 {
            return false;
        }
        true
    } else{
        false
    }
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    match (month, is_leap_year(year), month % 2) {
        (2, true, _) => 29,
        (2, false, _) => 28,
        (1..=7, _, 1) => 31,
        (1..=7, _, 0) => 30,
        (8..=12, _, 0) => 31,
        (8..=12, _, 1) => 30,
        _ => panic!("Invalid year or month !"),
    }
}

fn print_friday_13(year: u32, month: u32) -> () {
    print!("Friday, ");
    match month {
        1 => print!("January"),
        2 => print!("February"),
        3 => print!("March"),
        4 => print!("April"),
        5 => print!("May"),
        6 => print!("Jun"),
        7 => print!("July"),
        8 => print!("August"),
        9 => print!("September"),
        10 => print!("October"),
        11 => print!("November"),
        12 => print!("December"),
        _ => panic!("Invalid month"),
    }
    println!(" 13, {year}");
}

fn main() {
    let mut year: u32 = 1;
    let mut month: u32 = 1;
    let mut day_of_week: u32 = 1;

    while year < 2025 {
        while month <= 12{
            for n in 1..(num_days_in_month(year, month) + 1) {
                if n == 13 && day_of_week == 5 {
                    print_friday_13(year, month);
                }
                if day_of_week >= 7 {
                    day_of_week = 1;
                } else {
                    day_of_week += 1;
                }
            }
            month += 1;
        }
        month = 1;
        year += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leap_years() {
        assert_eq!(true, is_leap_year(1600));
        assert_eq!(false, is_leap_year(1500));
        assert_eq!(true, is_leap_year(2004));
        assert_eq!(false, is_leap_year(2003));
    }

    #[test]
    fn test_day_in_month() {
        assert_eq!(num_days_in_month(1600, 2), 29);
        assert_eq!(num_days_in_month(1500, 2), 28);
        assert_eq!(num_days_in_month(1500, 1), 31);
        assert_eq!(num_days_in_month(1500, 3), 31);
        assert_eq!(num_days_in_month(1500, 4), 30);
        assert_eq!(num_days_in_month(1500, 5), 31);
        assert_eq!(num_days_in_month(1500, 6), 30);
        assert_eq!(num_days_in_month(1500, 7), 31);
        assert_eq!(num_days_in_month(1500, 8), 31);
        assert_eq!(num_days_in_month(1500, 9), 30);
        assert_eq!(num_days_in_month(1500, 10), 31);
        assert_eq!(num_days_in_month(1500, 11), 30);
        assert_eq!(num_days_in_month(1500, 12), 31);
    }

    #[test]
    #[should_panic]
    fn test_invalid_month() {
        num_days_in_month(2000, 13);
    }

    #[test]
    #[should_panic]
    fn test_zero_leap_year() {
        is_leap_year(0);
    }
}
