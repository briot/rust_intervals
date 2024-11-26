use crate::nothing_between::NothingBetween;
use crate::step::Step;

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

impl Step for chrono::NaiveDate {
    fn min_value() -> Self {
        chrono::NaiveDate::MIN
    }
    fn max_value() -> Self {
        chrono::NaiveDate::MAX
    }

    #[cfg_attr(test, mutants::skip)]
    fn forward(&self, step: usize) -> Option<Self> {
        self.checked_add_days(chrono::Days::new(step as u64))
    }
    fn backward(&self, step: usize) -> Option<Self> {
        self.checked_sub_days(chrono::Days::new(step as u64))
    }
    fn elements_between(&self, other: &Self) -> Option<usize> {
        Some(other.signed_duration_since(*self).num_days() as usize)
    }
}

#[cfg(test)]
mod test {
    use crate::tests::test::check_empty;
    use crate::*;
    use ::chrono::{Local, NaiveDate, TimeDelta};

    #[test]
    fn test_chrono() {
        check_empty(
            "NaiveDate",
            NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 4, 2).unwrap(),
            NaiveDate::from_ymd_opt(2024, 4, 3).unwrap(),
            NaiveDate::from_ymd_opt(2024, 4, 10).unwrap(),
        );

        let now = Local::now();
        check_empty(
            "DateTime",
            now,
            now + TimeDelta::nanoseconds(1),
            now + TimeDelta::seconds(1),
            now + TimeDelta::minutes(1),
        );
    }

    #[test]
    fn test_chrono_iter() {
        let feb_27 = NaiveDate::from_ymd_opt(2024, 2, 27).unwrap();
        let feb_28 = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
        let feb_29 = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let mar_01 = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        let mar_02 = NaiveDate::from_ymd_opt(2024, 3, 2).unwrap();
        let mar_03 = NaiveDate::from_ymd_opt(2024, 3, 3).unwrap();
        let intv1 = interval!(feb_27, mar_03);
        assert_eq!(
            intv1.iter().collect::<Vec<_>>(),
            [feb_27, feb_28, feb_29, mar_01, mar_02]
        );
        assert_eq!(intv1.iter().size_hint(), (5, Some(5)));

        let intv1 = interval!(feb_27, mar_03);
        assert_eq!(
            intv1.iter().rev().collect::<Vec<_>>(),
            [mar_02, mar_01, feb_29, feb_28, feb_27],
        );

        let intv1 = Interval::<NaiveDate>::doubly_unbounded();
        assert_eq!(intv1.iter().take(1).collect::<Vec<_>>(), [NaiveDate::MIN],);
        assert_eq!(
            intv1.iter().rev().take(1).collect::<Vec<_>>(),
            [NaiveDate::MAX],
        );
    }
}
