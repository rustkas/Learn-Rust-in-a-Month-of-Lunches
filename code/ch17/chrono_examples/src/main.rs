

pub fn ex01() {
    use std::time::Instant;

    let time = Instant::now();
    println!("{:?}", time);
}

pub fn ex02() {
    use std::time::Instant;

    let start_of_main = Instant::now();
    let before_operation = Instant::now();

    let mut new_string = String::new();
    loop {
        new_string.push('წ');
        if new_string.len() > 100_000 {
            break;
        }
    }
    let after_operation = Instant::now();
    println!("{:?}", before_operation - start_of_main);
    println!("{:?}", after_operation - start_of_main);
}

pub fn ex03() {
    use std::time::Instant;

    let start = Instant::now();
    println!("Time elapsed before busy operation: {:?}", start.elapsed());
    let mut new_string = String::new();
    loop {
        new_string.push('წ');
        if new_string.len() > 100_000 {
            break;
        }
    }
    println!("Operation complete. Time elapsed: {:?}", start.elapsed());
}

pub fn ex04() {
    use std::time::Instant;

    fn bad_random_number(digits: usize) {
        if digits > 9 {
            panic!("Random number can only be up to 9 digits");
        }
        let now_as_string = format!("{:?}", Instant::now());
        now_as_string
            .chars()
            .rev()
            .filter_map(|c| c.to_digit(10))
            .take(digits)
            .for_each(|character| print!("{}", character));
        println!();
    }

    bad_random_number(1);
    bad_random_number(1);
    bad_random_number(3);
    bad_random_number(3);
}

pub fn ex05() {
    use std::time::{Instant, SystemTime};

    let instant = Instant::now();
    let system_time = SystemTime::now();
    println!("{instant:?}");
    println!("{system_time:?}");
}

pub fn ex06() {
    use std::time::{SystemTime, UNIX_EPOCH};
    println!(
        "{:?}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    );
}

pub fn ex07() {
    use std::thread::sleep;
    use std::time::Duration;
    let three_seconds = Duration::from_secs(3);
    println!("I must sleep now.");

    sleep(three_seconds);
    println!("Did I miss anything?");
}

/// using chrono
///
pub fn ex08() {
    use chrono::naive::{NaiveDate, NaiveTime};

    println!("{:?}", NaiveDate::from_ymd_opt(2023, 3, 25));
    println!("{:?}", NaiveTime::from_hms_opt(12, 5, 30));
    println!(
        "{:?}",
        NaiveDate::from_ymd_opt(2023, 3, 25)
            .unwrap()
            .and_hms_opt(12, 5, 30)
    );
}

pub fn ex09() {
    use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
    use std::time::SystemTime;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let seconds = now.as_secs();
    println!("Seconds from 1970 to today: {seconds}");
    let naive_dt = NaiveDateTime::from_timestamp_opt(seconds as i64, 0).unwrap();
    println!("As NaiveDateTime: {naive_dt}");

    let utc_dt = DateTime::<Utc>::from_utc(naive_dt, Utc);

    println!("As DateTime<Utc>: {utc_dt}");
    let kyiv_offset = FixedOffset::east_opt(3 * 60 * 60).unwrap();
    let kyiv_dt: DateTime<FixedOffset> = DateTime::from_utc(naive_dt, kyiv_offset);
    println!("In a timezone 3 hours from UTC: {kyiv_dt}");
    let kyiv_naive_dt = kyiv_dt.naive_local();
    println!("With timezone information removed: {kyiv_naive_dt}");
}
#[allow(dead_code)]
pub fn ex10() {
    use chrono::{DateTime, FixedOffset, Utc};
    use std::str::FromStr;
    const SECONDS_IN_HOUR: i32 = 3600;
    const UTC_TO_KST_HOURS: i32 = 9;
    const UTC_TO_KST_SECONDS: i32 = UTC_TO_KST_HOURS * SECONDS_IN_HOUR;
    #[derive(Debug)]
    struct UtcUserEvent {
        timestamp: &'static str,
        data: String,
    }
    #[derive(Debug)]
    struct KoreaJapanUserEvent {
        timestamp: DateTime<FixedOffset>,
        data: String,
    }
    impl From<UtcUserEvent> for KoreaJapanUserEvent {
        fn from(event: UtcUserEvent) -> Self {
            let utc_datetime: DateTime<Utc> = DateTime::from_str(event.timestamp).unwrap();
            let offset = FixedOffset::east_opt(UTC_TO_KST_SECONDS).unwrap();
            let timestamp: DateTime<FixedOffset> =
                DateTime::from_utc(utc_datetime.naive_utc(), offset);
            Self {
                timestamp,
                data: event.data,
            }
        }
    }

    let incoming_event = UtcUserEvent {
        timestamp: "2023-03-27 23:48:50 UTC",
        data: "Something happened in UTC time".to_string(),
    };
    println!("Event as Utc:\n{incoming_event:?}");
    let korea_japan_event = KoreaJapanUserEvent::from(incoming_event);
    println!("Event in Korea/Japan time:\n{korea_japan_event:?}");
}
fn main() {
    ex10();
}
