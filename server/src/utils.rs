use chrono::{NaiveDateTime, FixedOffset, DateTime};

const MOSCOW_OFFSET : i32 = 60 * 60 * 3;

pub fn parse_time(timestamp: i64) -> DateTime<FixedOffset> {
    let naive_dt = NaiveDateTime::from_timestamp(timestamp, 0);
    DateTime::from_utc(naive_dt, FixedOffset::east(MOSCOW_OFFSET))
}
