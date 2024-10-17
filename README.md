Rust intervals
==============

This package provides various functions to deal with intervals (aka ranges)
of values.

Features
--------

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
  - Operator overloads for the queries above
  - Support for `serde`
  - Support for `no_std`
  - Support for standard traits like `Default`, `Clone`, `Display`, `Debug`,
    `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`,
    `From<String>`, `From<Interval> -> String` and `FromStr`
    depending on what your type provides.   See examples below on how to
    convert to and from a string.

Example
-------

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

Authors
-------

- Emmanuel Briot  (Rust version)
- Duncan Sands (Ada version it is based on)
