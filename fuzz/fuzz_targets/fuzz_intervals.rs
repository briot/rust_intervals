#![no_main]

use libfuzzer_sys::fuzz_target;
mod fuzz_support;
use fuzz_support::RangeType;

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

fuzz_target!(|data: Data| {
    let intv1 = data.type1.build(data.lower1, data.upper1);
    let intv2 = data.type2.build(data.lower2, data.upper2);
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
