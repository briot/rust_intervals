#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_intervals::*;
mod fuzz_support;
use fuzz_support::RangeType;

/// Check that self is well-formed:
/// - intervals are sorted
/// - they do not overlap
/// - none of the intervals is empty
///
/// This is meant for tests, and should be useless in normal code, as the
/// various functions preserve those invariants.
fn check_invariants<T>(intv: &IntervalSet<T>)
where
    T: PartialOrd + NothingBetween,
{
    let mut it = intv.iter();
    if let Some(first) = it.next() {
        assert!(!first.is_empty());
    }
    for (i1, i2) in intv.iter().zip(it) {
        assert!(!i2.is_empty());
        assert!(i1.strictly_left_of_interval(i2));
    }
}

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
    type1: RangeType,
    lower1: u32,
    upper1: u32,
}

fuzz_target!(|data: [Data; 50]| {
    let mut m = IntervalSet::<u32>::default();
    m.extend(data.iter().map(|b| b.type1.build(b.lower1, b.upper1)));
    check_invariants(&m);
});
