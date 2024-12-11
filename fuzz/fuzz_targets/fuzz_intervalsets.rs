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
struct IntervalData {
    kind: RangeType,
    lower: u32,
    upper: u32,
}

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
    set1: [IntervalData; 25],
    set2: [IntervalData; 5],
    intv: IntervalData,
    val: u32,
}

fuzz_target!(|data: Data| {
    let mut m = IntervalSet::<u32>::default();
    m.extend(data.set1.iter().map(|b| b.kind.build(b.lower, b.upper)));
    check_invariants(&m);

    let intv = data.intv.kind.build(data.intv.lower, data.intv.upper);

    _ = m.lower();
    _ = m.lower_unbounded();
    _ = m.upper();
    _ = m.upper_unbounded();
    _ = m.len();
    _ = m.is_empty();
    _ = m.difference(data.val);
    _ = m.difference_interval(&intv);
    _ = m.contains(&data.val);
    _ = m.contains_interval(&intv);
    _ = m.intersects_interval(&intv);
    _ = m.intersection_interval(&intv);
    _ = m.convex_hull();
    _ = m.left_of(&data.val);
    _ = m.strictly_left_of(&data.val);
    _ = m.left_of_interval(&intv);
    _ = m.strictly_left_of_interval(&intv);
    _ = m.right_of(&data.val);
    _ = m.strictly_right_of(&data.val);
    _ = m.right_of_interval(&intv);
    _ = format!("{}", m);
    _ = format!("{:?}", m);

    m.remove(data.val);
    m.remove_interval(&intv);

    _ = m.iter().collect::<Vec<_>>();

    let mut m2 = IntervalSet::<u32>::default();
    m2.extend(data.set2.iter().map(|b| b.kind.build(b.lower, b.upper)));
    check_invariants(&m2);

    _ = m.equivalent(&m2);
    _ = m.contains_set(&m2);
    _ = m.intersection_set(&m2);
    _ = m.intersects_set(&m2);
    _ = m.left_of_set(&m2);
    _ = m.right_of_set(&m2);

});
