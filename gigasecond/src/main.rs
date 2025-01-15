fn main() {
    use gigasecond::after;

    println!("{}", after(datetime(2046, 10, 2, 23, 46, 40)));
}

fn datetime(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> time::PrimitiveDateTime {
    use time::{Date, PrimitiveDateTime, Time};

    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}

use time::PrimitiveDateTime as DateTime;

const KILO_SEC: u32 = 1_000;
const MEGA_SEC: u32 = 1_000_000;
const GIGA_SEC: u32 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // let result = start.ordinal()
    println!("{}", start);
    datetime(1959, 7, 19, 0, 0, 0)
}
