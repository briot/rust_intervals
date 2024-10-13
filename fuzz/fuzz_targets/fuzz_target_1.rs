#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_intervals::*;

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
    lower1: f32,
    upper1: f32,
    lower2: f32,
    upper2: f32,
}

fuzz_target!(|data: Data| {
    let intv1 = Interval::new_closed_open(data.lower1, data.upper1);
    let intv2 = Interval::new_closed_open(data.lower2, data.upper2);
    _ = intv1.intersects(&intv2);
    _ = intv1.equivalent(&intv2);
    _ = intv1.convex_hull(&intv2);
    _ = intv1.union(&intv2);
    _ = intv1.difference(&intv2);
    _ = intv1.symmetric_difference(&intv2);
});
