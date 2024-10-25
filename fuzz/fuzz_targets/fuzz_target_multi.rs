#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_intervals::*;
mod fuzz_support;
use fuzz_support::RangeType;

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
    type1: RangeType,
    lower1: u32,
    upper1: u32,
}

fuzz_target!(|data: [Data; 50]| {
    let mut m = MultiInterval::<u32>::default();
    m.extend(data.iter().map(|b| b.type1.build(b.lower1, b.upper1)));
    m.check_invariants();
});
