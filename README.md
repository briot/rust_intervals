# Rust intervals &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Badge]][docs]

[Build Status]: https://github.com/briot/rust_intervals/actions/workflows/tests.yml/badge.svg
[actions]: https://actions-badge.atrox.dev/briot/rust_intervals/goto
[Latest Version]: https://img.shields.io/crates/v/rust_intervals.svg
[crates.io]: https://crates.io/crates/rust_intervals
[Docs Badge]: https://docs.rs/rust_intervals/badge.svg
[docs]: https://docs.rs/rust_intervals

This package provides various functions to deal with intervals (aka ranges)
of values.

## Features

It provides the following features:

  - **Generic** over the interval type.  Support out of the box for u8, u16,
    u32, u64, i8, i16, i32, i64, char, and optionally for chrono::DateTime
  - Extensive **testing**, with **coverage** of nearly 100% of the code and
    basic fuzzing.
  - Supports any variation on **open-closed**, open-open, closed-open,
    closed-closed, unbounded-open, unbounded-closed, open-unbounded, and
    close-unbounded intervals.  These intervals can be freely combined.
  - Usable with types that do not even provide comparison, though with reduce
    feature set.  The more traits your type has, the more functions the
    interval provides.
  - Standard queries like
     * `contains()`:  whether the interval contains a specific value
     * `contains_interval()`: whether the interval fully includes another
       interval.
     * `is_empty()`: does the interval contain any value ?
     * `equivalent()`: do two intervals contain the same set of values ?
     * `strictly_left_of()`, `left_of()`, `right_of()`, `strictly_right_of()`:
       relative positions of two intervals
     * `convex_hull()`: smallest interval that contains two others
     * `difference()`: all values in one interval but not in another
     * `symmetric_difference()`: all values in either interval, but not in 
       both
     * `intersection()`: all values in both intervals
     * `between()`: all values between two intervals
     * `contiguous()`: whether two intervals are contiguous, with no values
       between them
     * `union()`: union of two contiguous intervals
  - Operator overloads for the queries above (`&`, `|`, `^`, `==`)
  - Support for `serde`
  - Support for `no_std`
  - Support for standard traits like `Default`, `Clone`, `Copy`, `Display`,
    `Debug`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`,
    `From<String>`, `From<Interval> -> String` and `FromStr`
    depending on what your type provides.   See examples below on how to
    convert to and from a string.
  - Convert from Rust's range `a..b`, `a..=b` and so on
  - Support for `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`
    and `IntoIterator`
  - Support for `Borrow` in parameters to make interface more convenient

## Example

```rust
   use rust_intervals::{interval, Interval};
   let intv1 = Interval::new_closed_open(1, 10);
   let value = 5;
   assert!(intv1.contains(&value));

   let intv2 = interval!(5, 8, "()");
   assert!(intv1.contains_interval(&intv2));

   let _ = intv1.convex_hull(&intv2);
```

An interval can be converted to a string using the various standard Rust
approaches:
```rust
   use rust_intervals::{interval, Interval};

   let intv1 = Interval::new_closed_open(1, 10);
   let s = format!("{}", intv1);  // using the Display trait
   let s = intv1.to_string();     // using the ToString trait (via Display)
   let s = String::from(intv1);   // using From<Interval<T>>->String trait
   let s: String = intv1.into();  // using the Into<String> trait

   let s = "[1, 4]";
   let intv = Interval::<u32>::from(s);  // using From<String> trait
   let intv: Interval<u32> = s.into();   // using Into<Interval<T>> trait
   let intv: Interval<u32> = s.parse()?; // using FromStr trait (may fail)

```

## Testing

This library includes extensive testing (`cargo make test-all`),
including support for MC/DC coverage (`cargo make cov`, reaching a total of
95.16%, where the remaining uncovered code all seem to be unreachable code).

We also work fuzzing tests (`cargo make fuzz`) and random mutations on the
code to detect missing tests (`cargo make mutants`).

## Similar packages

- `std::ops::Range` are the Rust built-in ranges.  They only provide
  a `contains()` function, do not provide `Copy` and do not provide all the
  kinds of bounds.

- `Extent` [https://docs.rs/extent/latest/extent/] only provide closed-closed
  intervals.  They also do not provide all the operations like `between()`,
  `convex_hull()` and so forth.  In exchange, they do not require extra flags
  in the struct, which makes them slightly smaller.

## Roadmap

The following features are planned.

- [ ] Sets of disjoint intervals
- [ ] Map from intervals to values (and resolve overlaps to unique values)

## Authors

- Emmanuel Briot  (Rust version)
- Duncan Sands (Ada version it is based on)
