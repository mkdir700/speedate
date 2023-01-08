use speedate::{Duration, ParseError};

#[path = "./utils.rs"]
mod utils;
use utils::param_tests;

#[test]
fn duration_simple() {
    let d = Duration::parse_str("P1Y").unwrap();
    assert_eq!(
        d,
        Duration {
            positive: true,
            day: 365,
            second: 0,
            microsecond: 0
        }
    );
    assert_eq!(d.to_string(), "P1Y");
}

#[test]
fn duration_total_seconds() {
    let d = Duration::parse_str("P1MT1.5S").unwrap();
    assert_eq!(
        d,
        Duration {
            positive: true,
            day: 30,
            second: 1,
            microsecond: 500_000
        }
    );
    assert_eq!(d.to_string(), "P30DT1.5S");
    assert_eq!(d.signed_total_seconds(), 30 * 86_400 + 1);
    assert_eq!(d.signed_microseconds(), 500_000);
}

#[test]
fn duration_total_seconds_neg() {
    let d = Duration::parse_str("-P1DT42.123456S").unwrap();
    assert_eq!(
        d,
        Duration {
            positive: false,
            day: 1,
            second: 42,
            microsecond: 123_456
        }
    );
    assert_eq!(d.to_string(), "-P1DT42.123456S");
    assert_eq!(d.signed_total_seconds(), -86_442);
    assert_eq!(d.signed_microseconds(), -123_456);
}

#[test]
fn duration_fractions() {
    let d = Duration::parse_str("P1.123W").unwrap();
    assert_eq!(
        d,
        Duration {
            positive: true,
            day: 7,
            second: 74390,
            microsecond: 400_000
        }
    );
}

#[test]
fn duration_new_normalise() {
    let d = Duration::new(false, 1, 86500, 1_000_123).unwrap();
    assert_eq!(
        d,
        Duration {
            positive: false,
            day: 2,
            second: 101,
            microsecond: 123,
        }
    );
}

#[test]
fn duration_new_normalise2() {
    let d = Duration::new(true, 0, 0, 1_000_000).unwrap();
    assert_eq!(
        d,
        Duration {
            positive: true,
            day: 0,
            second: 1,
            microsecond: 0,
        }
    );
}

#[test]
fn duration_comparison() {
    let d1 = Duration::new(true, 0, 0, 1_000_000).unwrap();
    let d2 = Duration::new(true, 0, 0, 1_000_001).unwrap();
    assert!(d1 < d2);
    assert!(d1 <= d2);
    assert!(d1 <= d1.clone());
    assert!(d2 > d1);
    assert!(d2 >= d1);
    assert!(d2 >= d2.clone());

    let d3 = Duration::new(true, 3, 0, 0).unwrap();
    let d4 = Duration::new(false, 4, 0, 0).unwrap();
    assert!(d3 > d4);
    assert!(d3 >= d4);
    assert!(d4 < d3);
    assert!(d4 <= d3);
    // from docs: `positive` is included in in comparisons, thus `+P1D` is greater than `-P2D`
    let d5 = Duration::parse_str("+P1D").unwrap();
    let d6 = Duration::parse_str("-P2D").unwrap();
    assert!(d5 > d6);

    let d7 = Duration::new(false, 3, 0, 0).unwrap();
    let d8 = Duration::new(false, 4, 0, 0).unwrap();
    assert!(d7 > d8);
    assert!(d8 < d7);
}

#[test]
fn duration_new_err() {
    let d = Duration::new(true, u32::MAX, 4294967295, 905969663);
    match d {
        Ok(t) => panic!("unexpectedly valid: {:?}", t),
        Err(e) => assert_eq!(e, ParseError::DurationValueTooLarge),
    }
    let d = Duration::new(true, u32::MAX, 0, 0);
    match d {
        Ok(t) => panic!("unexpectedly valid: {:?}", t),
        Err(e) => assert_eq!(e, ParseError::DurationDaysTooLarge),
    }
}

param_tests! {
    Duration,
    duration_too_short1: err => "", TooShort;
    duration_too_short2: err => "+", TooShort;
    duration_too_short3: err => "P", TooShort;
    duration_1y: ok => "P1Y", "P1Y";
    duration_123y: ok => "P123Y", "P123Y";
    duration_123_8y: ok => "P123.8Y", "P123Y292D";
    duration_1m: ok => "P1M", "P30D";
    duration_1_5m: ok => "P1.5M", "P45D";
    duration_1w: ok => "P1W", "P7D";
    duration_1_1w: ok => "P1.1W", "P7DT60480S";
    duration_1_123w: ok => "P1.123W", "P7DT74390.4S";
    duration_simple_negative: ok => "-P1Y", "-P1Y";
    duration_simple_positive: ok => "+P1Y", "P1Y";
    duration_fraction1: ok => "PT0.555555S", "PT0.555555S";
    duration_fraction2: ok => "P1Y1DT2H0.5S", "P1Y1DT7200.5S";
    duration_1: ok => "P1DT1S", "P1DT1S";
    duration_all: ok => "P1Y2M3DT4H5M6S", "P1Y63DT14706S";
    duration: err => "PD", DurationInvalidNumber;
    duration: err => "P1DT1MT1S", DurationTRepeated;
    duration: err => "P1DT1.1M1S", DurationInvalidFraction;
    duration: err => "P1DT1X", DurationInvalidTimeUnit;
    duration_invalid_day_unit1: err => "P1X", DurationInvalidDateUnit;
    duration_invalid_day_unit2: err => "P1", DurationInvalidDateUnit;
    duration_time_42s: ok => "00:00:42", "PT42S";
    duration_time_1m: ok => "00:01", "PT60S";
    duration_time_1h_2m_3s: ok => "01:02:03", "PT3723S";
    duration_time_fraction: ok => "00:01:03.123", "PT63.123S";
    duration_time_extra: err => "00:01:03.123x", ExtraCharacters;
    duration_days_1day1: ok => "1 day", "P1D";
    duration_days_1day2: ok => "1day", "P1D";
    duration_days_1day3: ok => "1 day,", "P1D";
    duration_days_1day4: ok => "1 day, ", "P1D";
    duration_days_1day5: ok => "1days", "P1D";
    duration_days_1day6: ok => "1DAYS", "P1D";
    duration_days_1day7: ok => "1d", "P1D";
    duration_days_1day8: ok => "1d ", "P1D";
    duration_days_too_short: err => "x", DurationInvalidNumber;
    duration_days_invalid1: err => "1x", DurationInvalidDays;
    duration_days_invalid2: err => "1dx", TooShort;
    duration_days_invalid3: err => "1da", DurationInvalidDays;
    duration_days_invalid4: err => "1", DurationInvalidDays;
    duration_days_invalid5: err => "1 ", DurationInvalidDays;
    duration_days_invalid6: err => "1 x", DurationInvalidDays;
    duration_days_neg: ok => "-1 day", "-P1D";
    duration_days_pos: ok => "+1 day", "P1D";
    duration_days_123days: ok => "123days", "P123D";
    duration_days_time: ok => "1 day 00:00:42", "P1DT42S";
    duration_days_time_neg: ok => "-1 day 00:00:42", "-P1DT42S";
    duration_exceeds_day: ok => "PT86500S", "P1DT100S";
    duration_days_time_too_shoert: err => "1 day 00:", TooShort;
    duration_days_time_wrong: err => "1 day 00:xx", InvalidCharMinute;
    duration_days_time_extra: err => "1 day 00:00:00.123 ", ExtraCharacters;
    duration_overflow: err => "18446744073709551616 day 12:00", DurationValueTooLarge;
    duration_fuzz1: err => "P18446744073709551611DT8031M1M1M1M", DurationValueTooLarge;
    duration_fuzz2: err => "P18446744073709550PT9970442H6R15D1D", DurationValueTooLarge;
}

#[test]
fn duration_large() {
    let d = Duration::parse_str("999999999 day 00:00").unwrap();
    assert_eq!(d.to_string(), "P2739726Y9D");

    let input = format!("{}1 day 00:00", u64::MAX);
    match Duration::parse_str(&input) {
        Ok(t) => panic!("unexpectedly valid: {:?} -> {:?}", input, t),
        Err(e) => assert_eq!(e, ParseError::DurationValueTooLarge),
    }
}

#[test]
fn duration_limit() {
    let d = Duration::new(true, 999_999_999, 86399, 999_999).unwrap();
    assert_eq!(d.to_string(), "P2739726Y9DT86399.999999S");

    match Duration::new(true, 999_999_999, 86399, 999_999 + 1) {
        Ok(t) => panic!("unexpectedly valid -> {:?}", t),
        Err(e) => assert_eq!(e, ParseError::DurationDaysTooLarge),
    }
    let d = Duration::new(false, 999_999_999, 86399, 999_999).unwrap();
    assert_eq!(d.to_string(), "-P2739726Y9DT86399.999999S");

    match Duration::new(false, 999_999_999, 86399, 999_999 + 1) {
        Ok(t) => panic!("unexpectedly valid -> {:?}", t),
        Err(e) => assert_eq!(e, ParseError::DurationDaysTooLarge),
    }
}
