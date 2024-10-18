use crate::bounds::Bound;
use crate::iterator::IntervalIterator;
use crate::multi_intervals::MultiInterval;
use crate::nothing_between::NothingBetween;
use crate::step::Step;
use ::core::cmp::{Ordering, PartialOrd};
use ::core::ops::{Bound as RgBound, RangeBounds};

/// An interval of values.
pub struct Interval<T> {
    pub(crate) lower: Bound<T>,
    pub(crate) upper: Bound<T>,
}

impl<T> Interval<T> {
    /// Construct a left-closed, right-open intervals (`[A,B)`)
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::new_closed_open(1, 10);
    ///    let intv2 = (1..10).into();
    ///    let intv3 = interval!(1, 10, "[)");
    ///    let intv4: Interval<u32> = "[1,10)".into();
    ///    let intv5 = "[1,10)".parse::<Interval<u32>>().unwrap();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// #  assert_eq!(intv1, intv5);
    /// ```
    pub fn new_closed_open(lower: T, upper: T) -> Self {
        Self {
            lower: Bound::LeftOf(lower),
            upper: Bound::LeftOf(upper),
        }
    }

    /// Construct a left-closed, right-closed intervals (`[A,B]`)
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::new_closed_closed(1, 10);
    ///    let intv2 = (1..=10).into();
    ///    let intv3 = interval!(1, 10, "[]");
    ///    let intv4: Interval<u32> = "[1,10]".into();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// ```
    pub fn new_closed_closed(lower: T, upper: T) -> Self {
        Self {
            lower: Bound::LeftOf(lower),
            upper: Bound::RightOf(upper),
        }
    }

    /// Construct a left-open, right-open intervals (`(A,B)`)
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::new_open_open(1, 10);
    ///    let intv2 = interval!(1, 10, "()");
    ///    let intv3: Interval<u32> = "(1,10)".into();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// ```
    pub fn new_open_open(lower: T, upper: T) -> Self {
        Self {
            lower: Bound::RightOf(lower),
            upper: Bound::LeftOf(upper),
        }
    }

    /// Construct a left-open, right-closed intervals (`(A,B]`)
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::new_open_closed(1, 10);
    ///    let intv2 = interval!(1, 10, "(]");
    ///    let intv3: Interval<u32> = "(1,10]".into();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// ```
    pub fn new_open_closed(lower: T, upper: T) -> Self {
        Self {
            lower: Bound::RightOf(lower),
            upper: Bound::RightOf(upper),
        }
    }

    /// Construct a left-unbounded, right-closed intervals (`(,B]`)
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::new_unbounded_closed(10);
    ///    let intv2 = (..=10).into();
    ///    let intv3 = interval!("-inf", 10, "]");
    ///    let intv4: Interval<u32> = "(,10]".into();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// ```
    pub fn new_unbounded_closed(upper: T) -> Self {
        Self {
            lower: Bound::LeftUnbounded,
            upper: Bound::RightOf(upper),
        }
    }

    /// Construct a left-unbounded, right-open intervals (`(,B)`)
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::new_unbounded_open(10);
    ///    let intv2 = (..10).into();
    ///    let intv3 = interval!("-inf", 10, ")");
    ///    let intv4: Interval<u32> = "(,10)".into();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// ```
    pub fn new_unbounded_open(upper: T) -> Self {
        Self {
            lower: Bound::LeftUnbounded,
            upper: Bound::LeftOf(upper),
        }
    }

    /// Construct a left-closed, right-unbounded intervals (`[A,)`)
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::new_closed_unbounded(10);
    ///    let intv2 = (10..).into();
    ///    let intv3 = interval!(10, "[inf");
    ///    let intv4: Interval<u32> = "[10,)".into();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// ```
    pub fn new_closed_unbounded(lower: T) -> Self {
        Self {
            lower: Bound::LeftOf(lower),
            upper: Bound::RightUnbounded,
        }
    }

    /// Construct a left-open, right-unbounded intervals (`(A,)`)
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::new_open_unbounded(10);
    ///    let intv2 = interval!(10, "(inf");
    ///    let intv3: Interval<u32> = "(10,)".into();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// ```
    pub fn new_open_unbounded(lower: T) -> Self {
        Self {
            lower: Bound::RightOf(lower),
            upper: Bound::RightUnbounded,
        }
    }

    /// Construct a doubly unbounded intervals (`(,)`) that contains all
    /// possible values.
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::<u32>::doubly_unbounded();
    ///    let intv2: Interval::<u32> = (..).into();
    ///    let intv3: Interval<u32> = "(,)".into();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// ```
    pub fn doubly_unbounded() -> Self {
        Self {
            lower: Bound::LeftUnbounded,
            upper: Bound::RightUnbounded,
        }
    }

    /// Returns an empty interval.  Note that there are multiple representations
    /// for empty interval, though they are all equivalent.
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::<u32>::empty();
    ///    let intv2: Interval<u32> = interval!("empty");
    ///    let intv3: Interval<u32> = "empty".into();
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// ```
    pub fn empty() -> Self {
        Self {
            lower: Bound::RightUnbounded,
            upper: Bound::LeftUnbounded,
        }
    }

    /// Returns an interval that contains a single value (`[value,value]`)
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::new_single(32);
    ///    let intv2 = interval!(32, 33);
    ///    let intv3 = interval!(32, 32, "[]");
    ///    let intv4 = interval!(31, 33, "()");
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// ```
    pub fn new_single(value: T) -> Self
    where
        T: Clone,
    {
        Interval::new_closed_closed(value.clone(), value)
    }

    /// Build an interval from one of Rust's range types. In most cases, you can
    /// also simply use `into()`
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = Interval::from_range(1..3);
    ///    assert_eq!(intv1, interval!(1, 3, "[)"));
    ///
    ///    let intv1: Interval<_> = (1..3).into();   //  same as above
    ///
    ///    let intv1 = Interval::from_range(1..=3);
    ///    assert_eq!(intv1, interval!(1, 3, "[]"));
    ///
    ///    let intv1 = Interval::from_range(1..);
    ///    assert_eq!(intv1, interval!(1, "[inf"));
    ///
    ///    let intv1 = Interval::from_range(..3);
    ///    assert_eq!(intv1, interval!("-inf", 3, ")"));
    ///
    ///    let intv1 = Interval::from_range(..=3);
    ///    assert_eq!(intv1, interval!("-inf", 3, "]"));
    ///
    ///    let intv1 = Interval::<f32>::from_range(..);
    ///    assert_eq!(intv1, Interval::doubly_unbounded());
    /// ```
    pub fn from_range<R: RangeBounds<T>>(range: R) -> Self
    where
        T: Clone,
    {
        match (range.start_bound(), range.end_bound()) {
            (RgBound::Included(lo), RgBound::Included(up)) => {
                Interval::new_closed_closed(lo.clone(), up.clone())
            }
            (RgBound::Included(lo), RgBound::Excluded(up)) => {
                Interval::new_closed_open(lo.clone(), up.clone())
            }
            (RgBound::Excluded(lo), RgBound::Included(up)) => {
                Interval::new_open_closed(lo.clone(), up.clone())
            }
            (RgBound::Excluded(lo), RgBound::Excluded(up)) => {
                Interval::new_open_open(lo.clone(), up.clone())
            }
            (RgBound::Unbounded, RgBound::Included(up)) => {
                Interval::new_unbounded_closed(up.clone())
            }
            (RgBound::Unbounded, RgBound::Excluded(up)) => {
                Interval::new_unbounded_open(up.clone())
            }
            (RgBound::Unbounded, RgBound::Unbounded) => {
                Interval::doubly_unbounded()
            }
            (RgBound::Included(lo), RgBound::Unbounded) => {
                Interval::new_closed_unbounded(lo.clone())
            }
            (RgBound::Excluded(lo), RgBound::Unbounded) => {
                Interval::new_open_unbounded(lo.clone())
            }
        }
    }

    /// The lower bound.  Returns None for an unbounded interval (i.e. lower
    /// is -infinity).
    /// For an empty interval, it returns whatever what used to create the
    /// interval (None if you used [`Interval::empty()`]), but the value is
    /// irrelevant.
    pub fn lower(&self) -> Option<&T> {
        self.lower.value()
    }

    /// Whether the lower bound is part of the interval.
    /// Return false for an empty interval, or if lower bound is -infinity.
    pub fn lower_inclusive(&self) -> bool {
        matches!(self.lower, Bound::LeftOf(_))
    }

    /// True if the lower bound is infinite  
    pub fn lower_unbounded(&self) -> bool {
        matches!(self.lower, Bound::LeftUnbounded)
    }

    /// The upper bound.  Returns None for an unbounded interval (i.e. upper
    /// is +infinity).
    /// For an empty interval, it returns whatever what used to create the
    /// interval (None if you used [`Interval::empty()`]), but the value is
    /// irrelevant.
    pub fn upper(&self) -> Option<&T> {
        self.upper.value()
    }

    /// Whether the upper bound is part of the interval.
    /// Return false for an empty interval, or if upper bound is +infinity.
    pub fn upper_inclusive(&self) -> bool {
        matches!(self.upper, Bound::RightOf(_))
    }

    /// True if the upper bound is infinite  
    pub fn upper_unbounded(&self) -> bool {
        matches!(self.upper, Bound::RightUnbounded)
    }

    /// Converts from `Interval<T>` to `Interval<&T>`
    pub fn as_ref(&self) -> Interval<&T> {
        Interval {
            lower: self.lower.as_ref(),
            upper: self.upper.as_ref(),
        }
    }

    /// True if the interval contains no element.
    /// This highly depends on how the NothingBetween trait was implemented.
    ///
    /// For instance, for f32, we consider the numbers as representable on
    /// the machine.  So an interval like:
    /// `[1.0, 1.0 + f32::EPSILON)`
    /// is empty, since we cannot represent any number from this interval.
    ///
    /// ```
    ///    use rust_intervals::Interval;
    ///    assert!(Interval::new_open_open(1.0, 1.0 + f32::EPSILON)
    ///        .is_empty());
    /// ```
    ///
    /// But if you implement your own wrapper type as
    /// ```
    ///     use rust_intervals::NothingBetween;
    ///     #[derive(PartialEq, PartialOrd)]
    ///     struct Real(f32);
    ///     impl NothingBetween for Real {
    ///         fn nothing_between(&self, _other: &Self) -> bool {
    ///             false
    ///         }
    ///     }
    /// ```
    /// then the same interval `[Real(1.0), Real(1.0 + f32::EPSILON)]` is
    /// no longer empty, even though we cannot represent any number from this
    /// interval.
    pub fn is_empty(&self) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        match self.upper.partial_cmp(&self.lower) {
            None => true, //  can't compare bounds
            Some(Ordering::Equal | Ordering::Less) => true,
            Some(Ordering::Greater) => false,
        }
    }

    /// Whether value is contained in the interval
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        self.lower.left_of(value) && self.upper.right_of(value)
    }

    /// Whether self contains all values of the second interval (and possibly
    /// more).
    pub fn contains_interval(&self, other: &Self) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        other.is_empty()
            || (self.lower <= other.lower && other.upper <= self.upper)
    }

    /// Whether the two intervals contain the same set of values
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 10);
    ///    let intv2 = interval!(1, 9, "[]");
    ///    assert!(intv1.equivalent(&intv2));
    ///    assert!(intv1 == intv2);   //  same
    /// ```
    pub fn equivalent(&self, other: &Self) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        if self.is_empty() {
            other.is_empty()
        } else if other.is_empty() {
            false
        } else {
            self.lower == other.lower && self.upper == other.upper
        }
    }

    /// Whether every value in self is strictly less than (<) X
    /// (returns True is if self is empty).
    /// ```txt
    ///    [------] .
    ///             X    => strictly left of the interval
    /// ```
    pub fn strictly_left_of(&self, x: &T) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        self.is_empty() || self.upper.left_of(x)
    }

    /// Whether X is strictly less than (<) every value in self.
    /// (returns True is if self is empty).
    /// ```txt
    ///    . [------]
    ///    X           => strictly right of the interval
    /// ```
    pub fn strictly_right_of(&self, x: &T) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        self.is_empty() || self.lower.right_of(x)
    }

    /// Whether every value in self is less than (<=) X.
    /// (returns True is if self is empty).
    /// ```txt
    ///    [------]
    ///           X    => left of the interval (but not strictly left of)
    /// ```
    pub fn left_of(&self, x: &T) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        self.is_empty() || self.upper <= Bound::RightOf(x)
    }

    /// Whether X is less than (<=) every value in self.
    /// (returns True is if self is empty).
    /// ```txt
    ///      [------]
    ///      X           => right of the interval (but not strictly right of)
    /// ```
    pub fn right_of(&self, x: &T) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        self.is_empty() || self.lower >= Bound::LeftOf(x)
    }

    /// Whether every value in self is less than or equal (<=) to every value
    /// in right (returns true if either interval is empty).
    pub fn left_of_interval(&self, right: &Self) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        self.strictly_left_of_interval(right)
            || self.upper.value() == right.lower.value()
    }

    /// Whethever every value in self is greater or equal (>=) to every value
    /// in right (returns true if either inverval is empty)
    pub fn right_of_interval(&self, right: &Self) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        self.strictly_right_of_interval(right)
            || self.lower.value() == right.upper.value()
    }

    /// Whether every value in self is strictly less than (<) every value in
    /// right (returns True if either interval is empty).
    pub fn strictly_left_of_interval(&self, right: &Self) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        self.is_empty() || right.is_empty() || self.upper <= right.lower
    }

    /// Whether every value in self is strictly greater than (>) every value in
    /// right (returns True if either interval is empty).
    pub fn strictly_right_of_interval(&self, right: &Self) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        self.is_empty() || right.is_empty() || right.upper <= self.lower
    }

    /// True if self is of the form `[A, A]`.
    /// This returns false for any other kind of interval, even if they
    /// happen to contain a single value.
    /// ```
    /// # use rust_intervals::Interval;
    ///   assert!(!Interval::new_open_open(0, 2).is_single());
    /// ```
    pub fn is_single(&self) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        match (&self.lower, &self.upper) {
            (Bound::LeftOf(lp), Bound::RightOf(rp)) => *lp == *rp,
            _ => false,
        }
    }

    /// Returns the convex hull of the two intervals, i.e. the smallest
    /// interval that contains the values of both intervals.
    pub fn convex_hull(&self, right: &Self) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if self.is_empty() {
            right.clone()
        } else if right.is_empty() {
            self.clone()
        } else {
            Self {
                lower: self.lower.min(&right.lower),
                upper: self.upper.max(&right.upper),
            }
        }
    }

    /// Returns the result of removing all values in right from self.
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 20);
    ///    let intv2 = interval!(10, 30, "[]");
    ///    let res1 = intv1.difference(&intv2);
    ///    let res2 = intv1 - intv2;
    ///    assert_eq!(res1, res2);
    ///
    ///    let _ = intv1 - &intv2;    //  Cam combine all variants of refs.
    ///    let _ = &intv1 - intv2;
    ///    let _ = &intv1 - &intv2;
    /// ```
    pub fn difference(&self, right: &Self) -> MultiInterval<T>
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if self.is_empty() || right.is_empty() {
            MultiInterval::One(self.clone())
        } else {
            MultiInterval::new_from_two(
                Interval {
                    lower: self.lower.clone(),
                    upper: right.lower.min(&self.upper),
                },
                Interval {
                    lower: right.upper.max(&self.lower),
                    upper: self.upper.clone(),
                },
            )
        }
    }

    /// Returns the values that are in either of the intervals, but not
    /// both.
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 20);
    ///    let intv2 = interval!(10, 30, "[]");
    ///    let res1 = intv1.symmetric_difference(&intv2);
    ///    let res2 = intv1 ^ intv2;
    ///    assert_eq!(res1, res2);
    ///
    ///    let _ = intv1 ^ &intv2;  // all variants of refs
    ///    let _ = &intv1 ^ &intv2;  // all variants of refs
    ///    let _ = &intv1 ^ intv2;  // all variants of refs
    /// ```
    pub fn symmetric_difference(&self, right: &Self) -> MultiInterval<T>
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if self.is_empty() || right.is_empty() {
            MultiInterval::new_from_two(self.clone(), right.clone())
        } else {
            MultiInterval::new_from_two(
                Interval {
                    lower: self.lower.min(&right.lower),
                    upper: self
                        .lower
                        .max(&right.lower)
                        .min(&self.upper.min(&right.upper)),
                },
                Interval {
                    lower: self
                        .upper
                        .min(&right.upper)
                        .max(&self.lower.max(&right.lower)),
                    upper: self.upper.max(&right.upper),
                },
            )
        }
    }

    /// Whether the two intervals overlap, i.e. have at least one point in
    /// common
    pub fn intersects(&self, right: &Self) -> bool
    where
        T: PartialOrd + NothingBetween,
    {
        !self.is_empty()
            && !right.is_empty()
            && self.lower < right.upper
            && right.lower < self.upper
    }

    /// Returns the intersection of the two intervals.  This is the same as the
    /// [`&`] operator.
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 20);
    ///    let intv2 = interval!(10, 30, "[]");
    ///    let res1 = intv1.intersection(&intv2);
    ///    let res2 = intv1 & intv2;
    ///    assert_eq!(res1, res2);
    ///
    ///    let _ = intv1 & &intv2;  // all variants of refs
    ///    let _ = &intv1 & &intv2;  // all variants of refs
    ///    let _ = &intv1 & intv2;  // all variants of refs
    /// ```
    pub fn intersection(&self, right: &Self) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        Interval {
            lower: self.lower.max(&right.lower),
            upper: self.upper.min(&right.upper),
        }
    }

    /// Returns the largest interval contained in the convex hull, that
    /// doesn't intersect with either self or right.
    /// This is empty if either of the two intervals is empty.
    /// If none of the intervals is empty, this consists of all values that
    /// are strictly between the given intervals
    pub fn between(&self, right: &Self) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if self.is_empty() || right.is_empty() {
            Interval::empty()
        } else {
            Interval {
                lower: self.upper.min(&right.upper),
                upper: self.lower.max(&right.lower),
            }
        }
    }

    /// If neither interval is empty, returns true if no value lies between
    /// them.  True if either of the intervals is empty.
    pub fn contiguous(&self, right: &Self) -> bool
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if self.is_empty() || right.is_empty() {
            true
        } else {
            self.lower <= right.upper && right.lower <= self.upper
        }
    }

    /// Returns the union of the two intervals, if they are contiguous.
    /// If not, returns None.
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 20);
    ///    let intv2 = interval!(20, 30);
    ///    let res1 = intv1.union(&intv2);
    ///    let res2 = intv1 | intv2;
    ///    assert_eq!(res1, res2);
    /// ```
    pub fn union(&self, right: &Self) -> Option<Self>
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if self.contiguous(right) {
            Some(self.convex_hull(right))
        } else {
            None
        }
    }

    /// Provides iteration over all values in the interval.
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    for _ in interval!(1, 10).iter() {
    ///    }
    ///    for _ in interval!(1, 10) {
    ///    }
    /// ```
    pub fn iter(&self) -> IntervalIterator<T>
    where
        T: Clone + Step + PartialOrd,
    {
        IntervalIterator::new(&self.lower, &self.upper)
    }
}

impl<T> IntoIterator for Interval<T>
where
    T: Step + Clone + PartialOrd,
{
    type Item = T;
    type IntoIter = IntervalIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Default for Interval<T> {
    /// Returns an empty interval
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Interval<T>
where
    T: Clone,
{
    //    /// If self is a closed-open interval, returns the equivalent Range
    //    /// `lower..uper`.  Otherwise return None.
    //    pub fn as_range(&self) -> Option<::core::ops::Range<T>> {
    //        match (&self.lower, &self.upper) {
    //            (Bound::LeftOf(lo), Bound::LeftOf(up)) => {
    //                Some(lo.clone()..up.clone())
    //            }
    //            _ => None,
    //        }
    //    }
    //
    //    pub fn as_range_inclusive(&self) -> Option<::core::ops::RangeInclusive<T>> {
    //        match (&self.lower, &self.upper) {
    //            (Bound::LeftOf(lo), Bound::RightOf(up)) => {
    //                Some(lo.clone()..=up.clone())
    //            }
    //            _ => None,
    //        }
    //    }
    //
    //    pub fn as_range_full(&self) -> Option<::core::ops::RangeFull> {
    //        match (&self.lower, &self.upper) {
    //            (Bound::LeftUnbounded, Bound::RightUnbounded) => Some(..),
    //            _ => None,
    //        }
    //    }
    //
    //    pub fn as_range_from(&self) -> Option<::core::ops::RangeFrom<T>> {
    //        match (&self.lower, &self.upper) {
    //            (Bound::LeftOf(lo), Bound::RightUnbounded) => Some(lo.clone()..),
    //            _ => None,
    //        }
    //    }
    //
    //    pub fn as_range_to(&self) -> Option<::core::ops::RangeTo<T>> {
    //        match (&self.lower, &self.upper) {
    //            (Bound::LeftUnbounded, Bound::LeftOf(up)) => Some(..up.clone()),
    //            _ => None,
    //        }
    //    }
    //
    //    pub fn as_range_to_inclusive(
    //        &self,
    //    ) -> Option<::core::ops::RangeToInclusive<T>> {
    //        match (&self.lower, &self.upper) {
    //            (Bound::LeftUnbounded, Bound::RightOf(up)) => Some(..=up.clone()),
    //            _ => None,
    //        }
    //    }

    //    /// Return the equivalent Rust range `a..b`, `a..=b`, `..b`, and so on
    //    pub fn as_range(
    //        &self,
    //    ) -> Option<(::core::ops::Bound<T>, ::core::ops::Bound<T>)> {
    //        match (&self.lower, &self.upper) {
    //            (Bound::LeftOf(lo), Bound::LeftOf(up)) => Some((
    //                ::core::ops::Bound::Included(lo.clone()),
    //                ::core::ops::Bound::Excluded(up.clone()),
    //            )),
    //            (Bound::LeftOf(lo), Bound::RightOf(up)) => Some((
    //                ::core::ops::Bound::Included(lo.clone()),
    //                ::core::ops::Bound::Included(up.clone()),
    //            )),
    //            (Bound::RightOf(lo), Bound::LeftOf(up)) => Some((
    //                ::core::ops::Bound::Excluded(lo.clone()),
    //                ::core::ops::Bound::Excluded(up.clone()),
    //            )),
    //            (Bound::RightOf(lo), Bound::RightOf(up)) => Some((
    //                ::core::ops::Bound::Excluded(lo.clone()),
    //                ::core::ops::Bound::Included(up.clone()),
    //            )),
    //            (Bound::LeftUnbounded, Bound::RightUnbounded) => Some((
    //                ::core::ops::Bound::Unbounded,
    //                ::core::ops::Bound::Unbounded,
    //            )),
    //            (Bound::LeftOf(lo), Bound::RightUnbounded) => Some((
    //                ::core::ops::Bound::Included(lo.clone()),
    //                ::core::ops::Bound::Unbounded,
    //            )),
    //            (Bound::RightOf(lo), Bound::RightUnbounded) => Some((
    //                ::core::ops::Bound::Excluded(lo.clone()),
    //                ::core::ops::Bound::Unbounded,
    //            )),
    //            (Bound::LeftUnbounded, Bound::LeftOf(up)) => Some((
    //                ::core::ops::Bound::Unbounded,
    //                ::core::ops::Bound::Excluded(up.clone()),
    //            )),
    //            (Bound::LeftUnbounded, Bound::RightOf(up)) => Some((
    //                ::core::ops::Bound::Unbounded,
    //                ::core::ops::Bound::Included(up.clone()),
    //            )),
    //            (Bound::RightUnbounded, _) => None,
    //            (_, Bound::LeftUnbounded) => None,
    //        }
    //    }
}

///  &Interval ^ &Interval
impl<T> ::core::ops::BitXor<&Interval<T>> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = MultiInterval<T>;

    fn bitxor(self, rhs: &Interval<T>) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

///  &Interval ^ Interval
impl<T> ::core::ops::BitXor<Interval<T>> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = MultiInterval<T>;

    fn bitxor(self, rhs: Interval<T>) -> Self::Output {
        self.symmetric_difference(&rhs)
    }
}

///  Interval ^ Interval
impl<T> ::core::ops::BitXor<Interval<T>> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = MultiInterval<T>;

    fn bitxor(self, rhs: Interval<T>) -> Self::Output {
        self.symmetric_difference(&rhs)
    }
}

///  Interval ^ &Interval
impl<T> ::core::ops::BitXor<&Interval<T>> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = MultiInterval<T>;

    fn bitxor(self, rhs: &Interval<T>) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

///  &Interval | &Interval
impl<T> ::core::ops::BitOr<&Interval<T>> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = Option<Interval<T>>;

    fn bitor(self, rhs: &Interval<T>) -> Self::Output {
        self.union(rhs)
    }
}

///  &Interval | Interval
impl<T> ::core::ops::BitOr<Interval<T>> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = Option<Interval<T>>;

    fn bitor(self, rhs: Interval<T>) -> Self::Output {
        self.union(&rhs)
    }
}

///  Interval | Interval
impl<T> ::core::ops::BitOr<Interval<T>> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = Option<Interval<T>>;

    fn bitor(self, rhs: Interval<T>) -> Self::Output {
        self.union(&rhs)
    }
}

///  Interval | &Interval
impl<T> ::core::ops::BitOr<&Interval<T>> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = Option<Interval<T>>;

    fn bitor(self, rhs: &Interval<T>) -> Self::Output {
        self.union(rhs)
    }
}

///  &Interval & &Interval
impl<T> ::core::ops::BitAnd<&Interval<T>> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = Interval<T>;

    fn bitand(self, rhs: &Interval<T>) -> Self::Output {
        self.intersection(rhs)
    }
}

///  &Interval & Interval
impl<T> ::core::ops::BitAnd<Interval<T>> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = Interval<T>;

    fn bitand(self, rhs: Interval<T>) -> Self::Output {
        self.intersection(&rhs)
    }
}

///  Interval & Interval
impl<T> ::core::ops::BitAnd<Interval<T>> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = Interval<T>;

    fn bitand(self, rhs: Interval<T>) -> Self::Output {
        self.intersection(&rhs)
    }
}

///  Interval & &Interval
impl<T> ::core::ops::BitAnd<&Interval<T>> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = Interval<T>;

    fn bitand(self, rhs: &Interval<T>) -> Self::Output {
        self.intersection(rhs)
    }
}

///   &Interval - &Interval
impl<T> ::core::ops::Sub<&Interval<T>> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = MultiInterval<T>;

    /// Same as [`Interval::difference()`]
    fn sub(self, rhs: &Interval<T>) -> Self::Output {
        self.difference(rhs)
    }
}

///   Interval - &Interval
impl<T> ::core::ops::Sub<&Interval<T>> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = MultiInterval<T>;

    /// Same as [`Interval::difference()`]
    fn sub(self, rhs: &Interval<T>) -> Self::Output {
        self.difference(rhs)
    }
}

///   &Interval - Interval
impl<T> ::core::ops::Sub<Interval<T>> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = MultiInterval<T>;

    /// Same as [`Interval::difference()`]
    fn sub(self, rhs: Interval<T>) -> Self::Output {
        self.difference(&rhs)
    }
}

///   Interval - Interval
impl<T> ::core::ops::Sub<Interval<T>> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
{
    type Output = MultiInterval<T>;

    /// Same as [`Interval::difference()`]
    fn sub(self, rhs: Interval<T>) -> Self::Output {
        self.difference(&rhs)
    }
}

impl<T> ::core::clone::Clone for Interval<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            lower: self.lower.clone(),
            upper: self.upper.clone(),
        }
    }
}

impl<T> Copy for Interval<T> where T: Copy {}

impl<T> PartialEq for Interval<T>
where
    T: PartialOrd + NothingBetween,
{
    /// True if the two intervals contain the same values (though they might
    /// have different bounds).
    fn eq(&self, other: &Self) -> bool {
        self.equivalent(other)
    }
}

impl<T> Eq for Interval<T> where T: PartialOrd + NothingBetween {}

impl<T> PartialOrd for Interval<T>
where
    T: PartialOrd + NothingBetween,
{
    /// Whether self starts to the left of other.
    /// If they start on the same value, whether self ends before other.
    /// This function might return True even if self has points to the right of
    /// every point in other.
    /// An empty interval is always before another interval.
    /// It has no real geometrical meaning.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.is_empty() {
            Some(Ordering::Greater)
        } else if self.is_empty() {
            Some(Ordering::Less)
        } else {
            match self.lower.partial_cmp(&other.lower) {
                None => None,
                Some(Ordering::Less) => Some(Ordering::Less),
                Some(Ordering::Greater) => Some(Ordering::Greater),
                Some(Ordering::Equal) => self.upper.partial_cmp(&other.upper),
            }
        }
    }
}

impl<T> Ord for Interval<T>
where
    T: PartialOrd + Ord + NothingBetween,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> ::core::hash::Hash for Interval<T>
where
    T: ::core::hash::Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.lower.hash(state);
        self.upper.hash(state);
    }
}

impl<T> ::core::fmt::Debug for Interval<T>
where
    T: ::core::fmt::Debug + NothingBetween + PartialOrd,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        if self.is_empty() {
            write!(f, "empty")?;
        } else {
            write!(f, "({:?},{:?})", self.lower, self.upper)?;
        }
        Ok(())
    }
}

/// Also provides an implementation for ToString
impl<T> ::core::fmt::Display for Interval<T>
where
    T: ::core::fmt::Display + NothingBetween + PartialOrd,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        if self.is_empty() {
            write!(f, "empty")?;
        } else {
            match &self.lower {
                Bound::LeftUnbounded => write!(f, "(")?,
                Bound::LeftOf(p) => write!(f, "[{}", p)?,
                Bound::RightOf(p) => write!(f, "({}", p)?,
                Bound::RightUnbounded => panic!("Invalid left bound"),
            }
            match &self.upper {
                Bound::LeftUnbounded => panic!("Invalid right bound"),
                Bound::LeftOf(p) => write!(f, ", {})", p)?,
                Bound::RightOf(p) => write!(f, ", {}]", p)?,
                Bound::RightUnbounded => write!(f, ",)")?,
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ParseError<E> {
    InvalidInput, // An invalid string was provided
    Bound(E),     // An error while parsing bounds
}

impl<T, E> core::str::FromStr for Interval<T>
where
    T: core::str::FromStr<Err = E>,
{
    type Err = ParseError<E>;

    /// This may fail and return an Error.  It is used in general via `parse()`.
    /// It assumes the first occurrence of ',' in the string is the separator
    /// for the two bounds of the interval, and is not part of the display for
    /// one of the bounds.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "empty" {
            return Ok(Interval::empty());
        }

        let mut input = s.char_indices();
        let (_, lo_incl) = input.next().unwrap();
        let mut up_incl: char = ']';
        let mut lo: Option<T> = None;
        let mut up: Option<T> = None;
        let mut start_offset: Option<usize> = None;

        for (c_offset, c) in input {
            if c == ',' {
                match start_offset {
                    None => lo = None,
                    Some(offs) => {
                        lo = match s[offs..c_offset].trim() {
                            "" => None,
                            a => Some(
                                a.parse::<T>()
                                    .map_err(|e| ParseError::Bound(e))?,
                            ),
                        };
                    }
                }
                start_offset = None;
            } else if c == ']' || c == ')' {
                match start_offset {
                    None => up = None,
                    Some(offs) => {
                        up = match s[offs..c_offset].trim() {
                            "" => None,
                            a => Some(
                                a.parse::<T>()
                                    .map_err(|e| ParseError::Bound(e))?,
                            ),
                        };
                    }
                }
                up_incl = c;
            } else if start_offset.is_none() {
                start_offset = Some(c_offset);
            }
        }

        Ok(match (lo_incl, lo, up, up_incl) {
            ('[', Some(lo), Some(up), ']') => {
                Interval::new_closed_closed(lo, up)
            }
            ('[', Some(lo), Some(up), ')') => Interval::new_closed_open(lo, up),
            ('(', Some(lo), Some(up), ')') => Interval::new_open_open(lo, up),
            ('(', Some(lo), Some(up), ']') => Interval::new_open_closed(lo, up),
            ('(', Some(lo), None, ')') => Interval::new_open_unbounded(lo),
            ('[', Some(lo), None, ')') => Interval::new_closed_unbounded(lo),
            ('(', None, Some(up), ')') => Interval::new_unbounded_open(up),
            ('(', None, Some(up), ']') => Interval::new_unbounded_closed(up),
            ('(', None, None, ')') => Interval::doubly_unbounded(),
            _ => Err(ParseError::InvalidInput)?,
        })
    }
}

impl<T: Clone> ::core::convert::From<::core::ops::Range<T>> for Interval<T> {
    fn from(value: ::core::ops::Range<T>) -> Self {
        Interval::new_closed_open(value.start.clone(), value.end.clone())
    }
}
impl<T: Clone> ::core::convert::From<::core::ops::RangeInclusive<T>>
    for Interval<T>
{
    fn from(value: ::core::ops::RangeInclusive<T>) -> Self {
        Interval::new_closed_closed(value.start().clone(), value.end().clone())
    }
}
impl<T: Clone> ::core::convert::From<::core::ops::RangeTo<T>> for Interval<T> {
    fn from(value: ::core::ops::RangeTo<T>) -> Self {
        Interval::new_unbounded_open(value.end.clone())
    }
}
impl<T: Clone> ::core::convert::From<::core::ops::RangeToInclusive<T>>
    for Interval<T>
{
    fn from(value: ::core::ops::RangeToInclusive<T>) -> Self {
        Interval::new_unbounded_closed(value.end.clone())
    }
}
impl<T: Clone> ::core::convert::From<::core::ops::RangeFrom<T>>
    for Interval<T>
{
    fn from(value: ::core::ops::RangeFrom<T>) -> Self {
        Interval::new_closed_unbounded(value.start.clone())
    }
}
impl<T: Clone> ::core::convert::From<::core::ops::RangeFull> for Interval<T> {
    fn from(_: ::core::ops::RangeFull) -> Self {
        Interval::doubly_unbounded()
    }
}

impl<T, E> ::core::convert::From<&str> for Interval<T>
where
    T: ::core::str::FromStr<Err = E>,
    E: ::core::fmt::Debug,
{
    /// Convert from a string to an interval.  This may not fail (see FromStrg
    /// otherwise).
    /// The format of the string is similar to what Display provides.jjj
    fn from(value: &str) -> Self {
        value.parse().expect("Could not parse string")
    }
}

#[cfg(feature = "std")]
impl<T> ::core::convert::From<Interval<T>> for String
where
    T: ::core::fmt::Display + PartialOrd + NothingBetween,
{
    fn from(value: Interval<T>) -> String {
        format!("{}", value)
    }
}
