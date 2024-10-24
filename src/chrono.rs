use crate::nothing_between::NothingBetween;

impl<T: chrono::TimeZone> NothingBetween for chrono::DateTime<T> {
    fn nothing_between(&self, other: &Self) -> bool {
        other.clone().signed_duration_since(self)
            <= chrono::TimeDelta::nanoseconds(1)
    }
}

impl NothingBetween for chrono::NaiveDate {
    fn nothing_between(&self, other: &Self) -> bool {
        other.signed_duration_since(*self) <= chrono::TimeDelta::days(1)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use ::chrono::{Local, NaiveDate, TimeDelta};

    #[test]
    fn test_chrono() {
        let apr_1 = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        let mar_31 = apr_1.pred_opt().unwrap();
        let apr_2 = apr_1.succ_opt().unwrap();
        assert!(Interval::new_closed_open(&apr_1, &apr_1).is_empty());
        assert!(Interval::new_open_open(&mar_31, &apr_1).is_empty());
        assert!(Interval::new_open_open(&apr_2, &apr_1).is_empty());

        let now = Local::now();
        let one_min_ago = now - TimeDelta::minutes(1);
        let one_sec_ago = now - TimeDelta::seconds(1);
        let one_ns_ago = now - TimeDelta::nanoseconds(1);
        assert!(!Interval::new_closed_open(one_min_ago, now).is_empty());
        assert!(!Interval::new_open_open(one_sec_ago, now).is_empty());
        assert!(Interval::new_open_open(one_ns_ago, now).is_empty());
    }
}
