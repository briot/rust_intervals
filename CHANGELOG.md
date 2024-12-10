# Version History

## 0.6.0
## Fixed
  - fixed incorrect implementation of contains_interval for Separating policy
  - "[1,5) left_of [4,10)" was returning false
## Added
  - support for `Clone` in `IntervalSet`
  - `IntervalSet::clear`
  - `IntervalSet::equivalent`
  - `IntervalSet::remove` and `IntervalSet::remove_interval`
  - `IntervalSet::intersects_interval` and `IntervalSet::intersects_set`
  - `IntervalSet::left_of` and `IntervalSet::right_of`, also variants
    for comparing with an interval or another set.  Add variants for
    strictly_left_of and strictly_right_of.
  - `IntervalSet::intersection_interval` and `IntervalSet::intersection_set`

## 0.5.0
## Fixed
 - two equal intervals did not always have same hash (e.g. "[1,2]" and "(0,3)")
## Added
 - support for iteration in `chrono::NaiveDate` intervals
 - `IntervalSet`
## Changed
 - renamed `MultiInterval` to `Pair`.  The former name will be used for sets of
   any number of intervals, but might not be compatible with no_std.  So we
   kept the simply `Pair` so that subprograms like `difference()` can remain
   compatible with no_std.
 - `Debug` no longer requires that T be PartialOrd (but it also no longer has
   special handling for empty intervals)

## 0.4.0
## Added
 - support `DoubleEndedIterator` and reverse iteration
 - iterators have gained a `as_interval()` function
 - support for `ExactSizeIterator`
### Changed
 - conversion from strings now uses `TryFrom` and `TryInto` instead of `From`.
   Those functions no longer panic, but return an error, and the user can
   decide how to handle that.

## 0.3.0
### Added
 - support for `Copy`
 - interact with Rust ranges `from_range()`
 - provide `BitOr` trait (equivalent to `union()`)
 - support for `Iterator` and `IntoIterator`
### Changed
 - use Borrow in parameters for `contains`

## 0.2.1
### Fixed
 - fix clippy warning about derived Hash and custom PartialEq

## 0.2.0
### Added
 - new functions `left_of_interval`, `right_of_interval`
   and `strictly_right_of_interval`
 - support for more types `u128`, `usize`, `isize`, `time::Duration` and
   `chrono::NaiveDate`
 - support for no_std
 - support for serde
 - support for rust_decimal
 - support for `Eq`, `From<&str>`, `From<Interval>` and `FromStr` traits
 - support for `Hash` trait
 - support for `Ord`, `PartialOrd` traits
