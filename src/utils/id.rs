use chrono::{TimeZone, Utc};
use sonyflake::{Error, Sonyflake};
use uuid::Uuid;

pub fn get_unique_id_string() -> String {
    Uuid::new_v4().hyphenated().to_string()
}

lazy_static::lazy_static! {
    static ref SF: Sonyflake = Sonyflake::builder().start_time(Utc.ymd(2020, 1, 1).and_hms(0, 0, 0)).finalize().unwrap();
}

pub fn next_id() -> Result<u64, Error> {
    SF.clone().next_id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_id_test() {
        assert!(next_id().is_ok());
    }
}
