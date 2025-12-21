use chrono::{NaiveDate, NaiveTime};

/// Takes a date and returns the date in unix_nanos.
pub fn to_unix(date: &NaiveDate) -> anyhow::Result<u64> {
    let dt_unix = date
        .and_time(NaiveTime::from_hms_nano_opt(0, 0, 0, 0).unwrap())
        .and_utc()
        .timestamp() as u64;
    Ok(dt_unix * 1_000_000_000)
}
