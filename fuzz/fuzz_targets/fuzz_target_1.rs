#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_intervals::*;

#[derive(Debug, arbitrary::Arbitrary)]
enum RangeType {
    OpenClosed,
    OpenOpen,
    ClosedOpen,
    ClosedClosed,
    ClosedUnbounded,
    OpenUnbounded,
    UnboundedClosed,
    UnboundedOpen,
    Empty,
}

type T = f32;

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
    type1: RangeType,
    lower1: T,
    upper1: T,
    type2: RangeType,
    lower2: T,
    upper2: T,
    val: T,
}

fn build(typ: RangeType, lower: T, upper: T) -> Interval<T> {
    match typ {
        RangeType::OpenClosed => Interval::new_open_closed(lower, upper),
        RangeType::OpenOpen => Interval::new_open_open(lower, upper),
        RangeType::ClosedOpen => Interval::new_closed_open(lower, upper),
        RangeType::ClosedClosed => Interval::new_closed_closed(lower, upper),
        RangeType::ClosedUnbounded => Interval::new_closed_unbounded(lower),
        RangeType::OpenUnbounded => Interval::new_open_unbounded(lower),
        RangeType::UnboundedClosed => Interval::new_unbounded_closed(upper),
        RangeType::UnboundedOpen => Interval::new_unbounded_open(upper),
        RangeType::Empty => Interval::empty(),
    }
}

fuzz_target!(|data: Data| {
    let intv1 = build(data.type1, data.lower1, data.upper1);
    let intv2 = build(data.type2, data.lower2, data.upper2);
    _ = intv1.lower();
    _ = intv1.lower_inclusive();
    _ = intv1.lower_unbounded();
    _ = intv1.upper();
    _ = intv1.upper_inclusive();
    _ = intv1.upper_unbounded();
    _ = intv1.as_ref();
    _ = intv1.is_single();
    _ = intv1.contains_interval(&intv2);
    _ = intv1.contains(&data.val);
    _ = intv1.strictly_left_of(&data.val);
    _ = intv1.left_of(&data.val);
    _ = intv1.strictly_left_of_interval(&intv2);
    _ = intv1.strictly_right_of(&data.val);
    _ = intv1.right_of(&data.val);
    _ = intv1.intersects(&intv2);
    _ = intv1.intersection(&intv2);
    _ = intv1.between(&intv2);
    _ = intv1.contiguous(&intv2);
    _ = intv1.equivalent(&intv2);
    _ = intv1.convex_hull(&intv2);
    _ = intv1.union(&intv2);
    _ = intv1.difference(&intv2);
    _ = intv1.symmetric_difference(&intv2);
});
