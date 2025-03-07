use crate::bounds::Bound;
use crate::iterator::IntervalIterator;
use crate::nothing_between::NothingBetween;
use crate::pairs::Pair;
use crate::step::{Bounded, Step};
use ::core::cmp::{Ordering, PartialOrd};
use ::core::ops::{Bound as RgBound, RangeBounds};

/// An interval of values.
pub struct Interval<T> {
    pub(crate) lower: Bound<T>,
    pub(crate) upper: Bound<T>,
    // For an empty interval, we have lower==RightUnbounded and
    // upper==LeftUnbounded
}

impl<T> Interval<T> {
    /// Construct a left-closed, right-open intervals (`[A,B)`).
    /// ```
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::new_closed_open(1, 10);
    ///    let intv2 = (1..10).into();
    ///    let intv3 = interval!(1, 10, "[)");
    ///    let intv4: Interval<u32> = "[1,10)".try_into()?;
    ///    let intv5 = "[1,10)".parse::<Interval<u32>>()?;
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// #  assert_eq!(intv1, intv5);
    /// #  Ok(())
    /// #  }
    /// ```
    pub fn new_closed_open(lower: T, upper: T) -> Self
    where
        T: PartialOrd,
    {
        match lower.partial_cmp(&upper) {
            None | Some(Ordering::Equal | Ordering::Greater) => {
                Interval::empty()
            }
            Some(Ordering::Less) => Self {
                lower: Bound::LeftOf(lower),
                upper: Bound::LeftOf(upper),
            },
        }
    }

    /// Construct a left-closed, right-closed intervals (`[A,B]`)
    /// ```
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::new_closed_closed(1, 10);
    ///    let intv2 = (1..=10).into();
    ///    let intv3 = interval!(1, 10, "[]");
    ///    let intv4: Interval<u32> = "[1,10]".try_into()?;
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// #  Ok(())
    /// #  }
    /// ```
    pub fn new_closed_closed(lower: T, upper: T) -> Self
    where
        T: PartialOrd,
    {
        match lower.partial_cmp(&upper) {
            None | Some(Ordering::Greater) => Interval::empty(),
            Some(Ordering::Less | Ordering::Equal) => Self {
                lower: Bound::LeftOf(lower),
                upper: Bound::RightOf(upper),
            },
        }
    }

    /// Construct a left-open, right-open intervals (`(A,B)`)
    /// ```
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    ///    use ::core::ops::Bound;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::new_open_open(1, 10);
    ///    let intv2 = interval!(1, 10, "()");
    ///    let intv3: Interval<u32> = "(1,10)".try_into()?;
    ///    let intv4 = Interval::from_range((
    ///        Bound::Excluded(1), Bound::Excluded(10)));
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// #  Ok(())
    /// #  }
    /// ```
    pub fn new_open_open(lower: T, upper: T) -> Self
    where
        T: PartialOrd + NothingBetween,
    {
        match lower.partial_cmp(&upper) {
            None | Some(Ordering::Equal | Ordering::Greater) => {
                Interval::empty()
            }
            Some(Ordering::Less) => {
                if lower.nothing_between(&upper) {
                    Interval::empty()
                } else {
                    Self {
                        lower: Bound::RightOf(lower),
                        upper: Bound::LeftOf(upper),
                    }
                }
            }
        }
    }

    /// Construct a left-open, right-closed intervals (`(A,B]`)
    /// ```
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    ///    use ::core::ops::Bound;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::new_open_closed(1, 10);
    ///    let intv2 = interval!(1, 10, "(]");
    ///    let intv3: Interval<u32> = "(1,10]".try_into()?;
    ///    let intv4 = Interval::from_range((
    ///        Bound::Excluded(1), Bound::Included(10)));
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// #  Ok(())
    /// #  }
    /// ```
    pub fn new_open_closed(lower: T, upper: T) -> Self
    where
        T: PartialOrd,
    {
        match lower.partial_cmp(&upper) {
            None | Some(Ordering::Equal | Ordering::Greater) => {
                Interval::empty()
            }
            Some(Ordering::Less) => Self {
                lower: Bound::RightOf(lower),
                upper: Bound::RightOf(upper),
            },
        }
    }

    /// Construct a left-unbounded, right-closed intervals (`(,B]`)
    /// ```
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::new_unbounded_closed(10);
    ///    let intv2 = (..=10).into();
    ///    let intv3 = interval!("-inf", 10, "]");
    ///    let intv4: Interval<u32> = "(,10]".try_into()?;
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// #  Ok(())
    /// #  }
    /// ```
    pub fn new_unbounded_closed(upper: T) -> Self
    where
        T: PartialOrd + Bounded,
    {
        //  If we can't compare, we likely have a float NAN
        match upper.partial_cmp(&T::min_value()) {
            Some(Ordering::Greater) => Self {
                lower: Bound::LeftUnbounded,
                upper: Bound::RightOf(upper),
            },
            None | Some(Ordering::Less | Ordering::Equal) => Interval::empty(),
        }
    }

    /// Construct a left-unbounded, right-open intervals (`(,B)`)
    /// ```
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::new_unbounded_open(10);
    ///    let intv2 = (..10).into();
    ///    let intv3 = interval!("-inf", 10, ")");
    ///    let intv4: Interval<u32> = "(,10)".try_into()?;
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// #  Ok(())
    /// #  }
    /// ```
    pub fn new_unbounded_open(upper: T) -> Self
    where
        T: PartialOrd + Bounded,
    {
        //  If we can't compare, we likely have a float NAN
        match upper.partial_cmp(&T::min_value()) {
            Some(Ordering::Greater) => Self {
                lower: Bound::LeftUnbounded,
                upper: Bound::LeftOf(upper),
            },
            None | Some(Ordering::Less | Ordering::Equal) => Interval::empty(),
        }
    }

    /// Construct a left-closed, right-unbounded intervals (`[A,)`)
    /// ```
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::new_closed_unbounded(10);
    ///    let intv2 = (10..).into();
    ///    let intv3 = interval!(10, "[inf");
    ///    let intv4: Interval<u32> = "[10,)".try_into()?;
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// #  Ok(())
    /// #  }
    /// ```
    pub fn new_closed_unbounded(lower: T) -> Self
    where
        T: PartialOrd + Bounded,
    {
        match lower.partial_cmp(&T::max_value()) {
            Some(Ordering::Less) => Self {
                lower: Bound::LeftOf(lower),
                upper: Bound::RightUnbounded,
            },
            None | Some(Ordering::Equal | Ordering::Greater) => {
                Interval::empty()
            }
        }
    }

    /// Construct a left-open, right-unbounded intervals (`(A,)`)
    /// ```
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    ///    use ::core::ops::Bound;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::new_open_unbounded(10);
    ///    let intv2 = interval!(10, "(inf");
    ///    let intv3: Interval<u32> = "(10,)".try_into()?;
    ///    let intv4 = Interval::from_range((
    ///        Bound::Excluded(10), Bound::Unbounded));
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  assert_eq!(intv1, intv4);
    /// #  Ok(())
    /// #  }
    /// ```
    pub fn new_open_unbounded(lower: T) -> Self
    where
        T: PartialOrd + Bounded,
    {
        match lower.partial_cmp(&T::max_value()) {
            Some(Ordering::Less) => Self {
                lower: Bound::RightOf(lower),
                upper: Bound::RightUnbounded,
            },
            None | Some(Ordering::Equal | Ordering::Greater) => {
                Interval::empty()
            }
        }
    }

    /// Construct a doubly unbounded intervals (`(,)`) that contains all
    /// possible values.
    /// ```
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::<u32>::doubly_unbounded();
    ///    let intv2: Interval::<u32> = (..).into();
    ///    let intv3: Interval<u32> = "(,)".try_into()?;
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  Ok(())
    /// #  }
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
    /// #  use rust_intervals::{interval, Interval, ParseError};
    /// #  use ::core::convert::TryInto;
    /// #  fn main() -> Result<(), ParseError<::core::num::ParseIntError>> {
    ///    let intv1 = Interval::<u32>::empty();
    ///    let intv2: Interval<u32> = interval!("empty");
    ///    let intv3: Interval<u32> = "empty".try_into()?;
    /// #  assert_eq!(intv1, intv2);
    /// #  assert_eq!(intv1, intv3);
    /// #  Ok(())
    /// #  }
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
        T: PartialOrd + Clone,
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
        T: PartialOrd + NothingBetween + Clone + Bounded,
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
    /// For an empty interval, it returns None.
    /// This value might not actually be valid for self, if we have an
    /// open bound for instance.
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
    /// For an empty interval, it returns None.
    /// This value might not actually be valid for self, if we have an
    /// open bound for instance.
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
    pub fn is_empty(&self) -> bool {
        match (&self.lower, &self.upper) {
            (Bound::RightUnbounded, Bound::LeftUnbounded) => true,
            _ => false,
        }
    }

    /// Whether value is contained in the interval.
    /// You can pass either a T or &T, for convenience.
    /// ```
    /// #  use rust_intervals::interval;
    ///    let intv1 = interval!(1, 10);
    ///    assert!(intv1.contains(2));
    ///    assert!(intv1.contains(&2));
    /// ```
    pub fn contains<V>(&self, value: V) -> bool
    where
        T: PartialOrd + NothingBetween,
        V: ::core::borrow::Borrow<T>,
    {
        let t = value.borrow();
        self.lower.left_of(t) && self.upper.right_of(t)
    }

    /// Whether self contains all values of the second interval (and possibly
    /// more).
    pub fn contains_interval<U>(&self, other: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        let s = other.borrow();
        self.lower <= s.lower && s.upper <= self.upper
    }

    /// Whether the two intervals contain the same set of values
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 10);
    ///    let intv2 = interval!(1, 9, "[]");
    ///    assert!(intv1.equivalent(&intv2));
    ///    assert!(intv1 == intv2);   //  same
    /// ```
    pub fn equivalent<U>(&self, other: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        let u = other.borrow();
        self.lower == u.lower && self.upper == u.upper
    }

    /// Whether every value in self is strictly less than (<) X
    /// (returns True is if self is empty).
    ///
    /// Note that the handling of empty ranges is different than in postgres
    /// where it would return False.  But this crate views ranges as sets of
    /// values, so it makes more sense to return true in this case.  Postgres
    /// treats empty ranges more like NaN, which cannot be compared.
    ///
    /// ```none
    ///    [------] .
    ///             X    => strictly left of the interval
    /// ```
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 10);
    ///    assert!(intv1.strictly_left_of(11));
    ///    assert!(intv1.strictly_left_of(&11));  //  can pass references
    /// ```
    pub fn strictly_left_of<K>(&self, x: K) -> bool
    where
        T: PartialOrd + NothingBetween,
        K: ::core::borrow::Borrow<T>,
    {
        self.upper.left_of(x.borrow())
    }

    /// Whether X is strictly less than (<) every value in self.
    /// (returns True is if self is empty).
    /// ```none
    ///    . [------]
    ///    X           => strictly right of the interval
    /// ```
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 10);
    ///    assert!(intv1.strictly_right_of(0));
    ///    assert!(intv1.strictly_right_of(&0));  //  can pass references
    /// ```
    pub fn strictly_right_of<K>(&self, x: K) -> bool
    where
        T: PartialOrd + NothingBetween,
        K: ::core::borrow::Borrow<T>,
    {
        self.lower.right_of(x.borrow())
    }

    /// Whether every value in self is less than (<=) X.
    /// (returns True is if self is empty).
    /// ```none
    ///    [------]
    ///           X    => left of the interval (but not strictly left of)
    /// ```
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 10);
    ///    assert!(intv1.left_of(9));
    ///    assert!(intv1.left_of(&9));  //  can pass references
    /// ```
    pub fn left_of<K>(&self, x: K) -> bool
    where
        T: PartialOrd + NothingBetween,
        K: ::core::borrow::Borrow<T>,
    {
        self.upper <= Bound::RightOf(x.borrow())
    }

    /// Whether X is less than (<=) every value in self.
    /// (returns True if self is empty).
    /// ```none
    ///      [------]
    ///      X           => right of the interval (but not strictly right of)
    /// ```
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 10);
    ///    assert!(intv1.right_of(1));
    ///    assert!(intv1.right_of(&1));  //  can pass references
    /// ```
    pub fn right_of<K>(&self, x: K) -> bool
    where
        T: PartialOrd + NothingBetween,
        K: ::core::borrow::Borrow<T>,
    {
        self.lower >= Bound::LeftOf(x.borrow())
    }

    /// Whether every value in self is less than or equal (<=) to every value
    /// in right (returns true if either interval is empty).
    pub fn left_of_interval<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();

        // Case of "..,5)" and "[4,..".  The bounds are different and yet
        // we should return true.
        // We end up comparing LeftOf(5) and LeftOf(4), which compares 5 and 4
        // and returns Greater.  But because there is nothing_between(4,5),
        // those should actually be equal.
        match (&self.upper, &r.lower) {
            // Either interval is empty
            (Bound::LeftUnbounded, _) | (_, Bound::RightUnbounded) => true,

            // "..,4]" and "[4,..."
            (Bound::RightOf(u), Bound::LeftOf(r)) => u <= r,

            // "..,5)" and "[4,.."
            (Bound::LeftOf(u), Bound::LeftOf(r)) => {
                u <= r || r.nothing_between(u)
            }
            _ => self.strictly_left_of_interval(r),
        }
    }

    /// Whether every value in self is strictly less than (<) every value in
    /// right (returns True if either interval is empty).
    pub fn strictly_left_of_interval<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        self.upper <= r.lower
    }

    /// Whether all values of self are strictly lower than every value in right,
    /// and there is some thing between the two intervals.
    pub fn strictly_left_not_contiguous<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        self.upper < r.lower
    }

    /// Whether every value in self is greater or equal (>=) to every value
    /// in right (returns true if either inverval is empty)
    pub fn right_of_interval<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        right.borrow().left_of_interval(self)
    }

    /// Whether every value in self is strictly greater than (>) every value in
    /// right (returns True if either interval is empty).
    pub fn strictly_right_of_interval<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        right.borrow().strictly_left_of_interval(self)
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
    pub fn convex_hull<U>(&self, right: U) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        let low = (&self.lower).partial_cmp(&r.lower); //  want min()
        let upp = (&self.upper).partial_cmp(&r.upper); //  want max()
        match (low, upp) {
            (None, _) | (_, None) => unreachable!(),
            (
                Some(Ordering::Less | Ordering::Equal),
                Some(Ordering::Less | Ordering::Equal),
            ) => Interval::from_bounds(&self.lower, &r.upper),
            (
                Some(Ordering::Less | Ordering::Equal),
                Some(Ordering::Greater),
            ) => self.clone(),
            (
                Some(Ordering::Greater),
                Some(Ordering::Less | Ordering::Equal),
            ) => r.clone(),
            (Some(Ordering::Greater), Some(Ordering::Greater)) => {
                Interval::from_bounds(&r.lower, &self.upper)
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
    pub fn difference<U>(&self, right: U) -> Pair<T>
    where
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        let up1 = (&r.lower).partial_cmp(&self.upper); //  want min()
        let lo2 = (&r.upper).partial_cmp(&self.lower); //  want max()
        match (up1, lo2) {
            (None, _)      // One of the intervals is empty
            | (_, None)    // One of the intervals is empty
            | (
                Some(Ordering::Less | Ordering::Equal),
                Some(Ordering::Less | Ordering::Equal),
            )
            | (Some(Ordering::Greater), _) => Pair::One(self.clone()),

            (
                Some(Ordering::Less | Ordering::Equal),
                Some(Ordering::Greater),
            ) => Pair::new_from_two(
                Interval::from_bounds(&self.lower, &r.lower),
                Interval::from_bounds(&r.upper, &self.upper),
            ),
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
    pub fn symmetric_difference<U>(&self, right: U) -> Pair<T>
    where
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        if self.is_empty() || r.is_empty() {
            return Pair::new_from_two(self.clone(), r.clone());
        }

        let (min_of_lowers, max_of_lowers) = match (&self.lower)
            .partial_cmp(&r.lower)
        {
            None => return Pair::new_from_two(self.clone(), r.clone()),
            Some(Ordering::Less | Ordering::Equal) => (&self.lower, &r.lower),
            Some(Ordering::Greater) => (&r.lower, &self.lower),
        };

        let (min_of_uppers, max_of_uppers) = match (&self.upper)
            .partial_cmp(&r.upper)
        {
            None => return Pair::new_from_two(self.clone(), r.clone()),
            Some(Ordering::Less | Ordering::Equal) => (&self.upper, &r.upper),
            Some(Ordering::Greater) => (&r.upper, &self.upper),
        };

        let (v1, v2) = match max_of_lowers.partial_cmp(min_of_uppers) {
            None => return Pair::new_from_two(self.clone(), r.clone()),
            Some(Ordering::Less | Ordering::Equal) => {
                (max_of_lowers, min_of_uppers)
            }
            Some(Ordering::Greater) => (min_of_uppers, max_of_lowers),
        };

        Pair::new_from_two(
            Interval::from_bounds(min_of_lowers, v1),
            Interval::from_bounds(v2, max_of_uppers),
        )
    }

    /// Whether the two intervals overlap, i.e. have at least one point in
    /// common.
    #[doc(alias = "overlaps")]
    pub fn intersects<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        self.lower < r.upper && r.lower < self.upper
    }

    /// Returns the intersection of the two intervals.  This is the same as the
    /// [`&`] operator.
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 20);
    ///    let intv2 = interval!(10, 30, "[]");
    ///    let res1 = intv1.intersection(&intv2);
    ///    let res2 = intv1 & intv2;
    ///    let r3 = intv1 & &intv2;  // all variants of refs
    ///    let r4 = &intv1 & &intv2;  // all variants of refs
    ///    let r5 = &intv1 & intv2;  // all variants of refs
    /// #  assert_eq!(res1, res2);
    /// #  assert_eq!(res1, r3);
    /// #  assert_eq!(res1, r4);
    /// #  assert_eq!(res1, r5);
    /// ```
    pub fn intersection<U>(&self, right: U) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        let low = (&self.lower).partial_cmp(&r.lower); //  want max()
        let upp = (&self.upper).partial_cmp(&r.upper); //  want min()
        match (low, upp) {
            (None, _) | (_, None) => Interval::empty(),
            (
                Some(Ordering::Less | Ordering::Equal),
                Some(Ordering::Less | Ordering::Equal),
            ) => Interval::from_bounds(&r.lower, &self.upper),
            (
                Some(Ordering::Less | Ordering::Equal),
                Some(Ordering::Greater),
            ) => r.clone(),
            (
                Some(Ordering::Greater),
                Some(Ordering::Less | Ordering::Equal),
            ) => self.clone(),
            (Some(Ordering::Greater), Some(Ordering::Greater)) => {
                Interval::from_bounds(&self.lower, &r.upper)
            }
        }
    }

    /// Returns the largest interval contained in the convex hull, that
    /// doesn't intersect with either self or right.
    /// This is empty if either of the two intervals is empty.
    /// If none of the intervals is empty, this consists of all values that
    /// are strictly between the given intervals
    pub fn between<U>(&self, right: U) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        let low = (&self.upper).partial_cmp(&r.upper); //  want min()
        let upp = (&self.lower).partial_cmp(&r.lower); //  want max()
        match (low, upp) {
            (None, _) | (_, None) => Interval::empty(),
            (
                Some(Ordering::Less | Ordering::Equal),
                Some(Ordering::Less | Ordering::Equal),
            ) => Interval::from_bounds(&self.upper, &r.lower),
            (
                Some(Ordering::Less | Ordering::Equal),
                Some(Ordering::Greater),
            ) => {
                if self.is_empty() {
                    Interval::empty()
                } else {
                    Interval::from_bounds(&self.upper, &self.lower)
                }
            }
            (
                Some(Ordering::Greater),
                Some(Ordering::Less | Ordering::Equal),
            ) => {
                if r.is_empty() {
                    Interval::empty()
                } else {
                    Interval::from_bounds(&r.upper, &r.lower)
                }
            }
            (Some(Ordering::Greater), Some(Ordering::Greater)) => {
                Interval::from_bounds(&r.upper, &self.lower)
            }
        }
    }

    /// If neither interval is empty, returns true if no value lies between
    /// them.  True if either of the intervals is empty.
    pub fn contiguous<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        self.is_empty()
            || r.is_empty()
            || (self.lower <= r.upper && r.lower <= self.upper)
    }

    /// Returns the union of the two intervals, if they are contiguous.
    /// If not, returns None.
    /// ```
    /// #  use rust_intervals::{interval, Interval};
    ///    let intv1 = interval!(1, 20);
    ///    let intv2 = interval!(20, 30);
    ///    let res1 = intv1.union(&intv2);
    ///    let res2 = intv1 | intv2;
    ///    let res3 = intv1 | &intv2;  // all variants of refs
    ///    let res4 = &intv1 | &intv2;  // all variants of refs
    ///    let res5 = &intv1 | intv2;  // all variants of refs
    /// #  assert_eq!(res1, res2);
    /// #  assert_eq!(res1, res3);
    /// #  assert_eq!(res1, res4);
    /// #  assert_eq!(res1, res5);
    /// ```
    pub fn union<U>(&self, right: U) -> Option<Self>
    where
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<Self>,
    {
        let r = right.borrow();
        if self.contiguous(r) {
            Some(self.convex_hull(r))
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
        T: Clone + Step + PartialOrd + NothingBetween,
    {
        IntervalIterator { intv: self.clone() }
    }

    /// Creates an interval from its bounds, and normalizes empty intervals
    pub(crate) fn from_bounds(lower: &Bound<T>, upper: &Bound<T>) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if lower >= upper {
            Interval::empty()
        } else {
            Interval {
                lower: lower.clone(),
                upper: upper.clone(),
            }
        }
    }
}

impl<T> IntoIterator for Interval<T>
where
    T: Step + Bounded + Clone + PartialOrd + NothingBetween,
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

///  &Interval ^ &Interval
///  and &Interval ^ Interval
impl<T, U> ::core::ops::BitXor<U> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = Pair<T>;

    fn bitxor(self, rhs: U) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

///  Interval ^ &Interval
///  and Interval ^ Interval
impl<T, U> ::core::ops::BitXor<U> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = Pair<T>;

    fn bitxor(self, rhs: U) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

///  &Interval | &Interval
///  and &Interval | Interval
impl<T, U> ::core::ops::BitOr<U> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = Option<Interval<T>>;

    fn bitor(self, rhs: U) -> Self::Output {
        self.union(rhs)
    }
}

///  Interval | Interval
///  and Interval | &Interval
impl<T, U> ::core::ops::BitOr<U> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = Option<Interval<T>>;

    fn bitor(self, rhs: U) -> Self::Output {
        self.union(rhs)
    }
}

///  &Interval & &Interval
///  and &Interval & Interval
impl<T, U> ::core::ops::BitAnd<U> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = Interval<T>;

    fn bitand(self, rhs: U) -> Self::Output {
        self.intersection(rhs)
    }
}

///  Interval & Interval
///  and Interval & &Interval
impl<T, U> ::core::ops::BitAnd<U> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = Interval<T>;

    fn bitand(self, rhs: U) -> Self::Output {
        self.intersection(rhs)
    }
}

///   &Interval - &Interval
///   and &Interval - Interval
impl<T, U> ::core::ops::Sub<U> for &Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = Pair<T>;

    /// Same as [`Interval::difference()`]
    fn sub(self, rhs: U) -> Self::Output {
        self.difference(rhs)
    }
}

///   Interval - &Interval
///   and Interval - Interval
impl<T, U> ::core::ops::Sub<U> for Interval<T>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = Pair<T>;

    /// Same as [`Interval::difference()`]
    fn sub(self, rhs: U) -> Self::Output {
        self.difference(rhs)
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
    /// This depends on the implementation of NothingBetween, and could have
    /// unexpected results for floats where the machine precision is limited.
    /// For instance
    /// ```
    /// #  use rust_intervals::{interval};
    ///    assert_eq!(
    ///        &interval!(1.0 + f32::EPSILON, 2.0, "[]"),
    ///        &interval!(1.0, 2.0, "(]")
    ///    );
    ///    assert_ne!(  // precision is good enough
    ///        &interval!(1.0 + 2.0 * f32::EPSILON, 2.0, "[]"),
    ///        &interval!(1.0, 2.0, "(]")
    ///    );
    ///    assert_eq!(   // precision of f32 is too low
    ///        &interval!(1E12 + 2.0 * f32::EPSILON, 2.0, "[]"),
    ///        &interval!(1E12, 2.0, "(]")
    ///    );
    /// ```
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
    /// An empty interval is always after another interval.
    /// It has no real geometrical meaning.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.lower.partial_cmp(&other.lower) {
            None => unreachable!(), // interval is empty
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Equal) => self.upper.partial_cmp(&other.upper),
        }
    }
}

impl<T> Ord for Interval<T>
where
    T: PartialOrd + Ord + NothingBetween,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            None => unreachable!(),
            Some(order) => order,
        }
    }
}

impl<T> ::core::hash::Hash for Interval<T>
where
    T: ::core::hash::Hash + Step + NothingBetween,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.lower.hash(state);
        self.upper.hash(state);
    }
}

impl<T> ::core::fmt::Debug for Interval<T>
where
    T: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "({:?},{:?})", self.lower, self.upper)?;
        Ok(())
    }
}

/// Also provides an implementation for ToString
impl<T> ::core::fmt::Display for Interval<T>
where
    T: ::core::fmt::Display + NothingBetween + PartialOrd,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match (&self.lower, &self.upper) {
            (Bound::RightUnbounded, _) | (_, Bound::LeftUnbounded) => {
                write!(f, "empty")?
            }
            (Bound::LeftUnbounded, Bound::LeftOf(p)) => write!(f, "(, {})", p)?,
            (Bound::LeftUnbounded, Bound::RightOf(p)) => {
                write!(f, "(, {}]", p)?
            }
            (Bound::LeftUnbounded, Bound::RightUnbounded) => write!(f, "(,)")?,
            (Bound::LeftOf(l), Bound::LeftOf(p)) => {
                write!(f, "[{}, {})", l, p)?
            }
            (Bound::LeftOf(l), Bound::RightOf(p)) => {
                write!(f, "[{}, {}]", l, p)?
            }
            (Bound::LeftOf(l), Bound::RightUnbounded) => write!(f, "[{},)", l)?,
            (Bound::RightOf(l), Bound::LeftOf(p)) => {
                write!(f, "({}, {})", l, p)?
            }
            (Bound::RightOf(l), Bound::RightOf(p)) => {
                write!(f, "({}, {}]", l, p)?
            }
            (Bound::RightOf(l), Bound::RightUnbounded) => {
                write!(f, "({},)", l)?
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError<E> {
    InvalidInput, // An invalid string was provided
    Bound(E),     // An error while parsing bounds
}

impl<T, E> ::core::str::FromStr for Interval<T>
where
    T: PartialOrd + Bounded + NothingBetween + ::core::str::FromStr<Err = E>,
{
    type Err = ParseError<E>;

    /// This may fail and return an Error.  It is used in general via `parse()`.
    /// It assumes the first occurrence of ',' in the string is the separator
    /// for the two bounds of the interval, and is not part of the display for
    /// one of the bounds.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "empty" || s.is_empty() {
            return Ok(Interval::empty());
        }

        let mut input = s.char_indices();
        let lo_incl = match input.next() {
            None => unreachable!(),
            Some((_, lo_incl)) => lo_incl,
        };
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

impl<T> ::core::convert::From<::core::ops::Range<T>> for Interval<T>
where
    T: Clone + PartialOrd + NothingBetween,
{
    fn from(value: ::core::ops::Range<T>) -> Self {
        Interval::new_closed_open(value.start.clone(), value.end.clone())
    }
}
impl<T> ::core::convert::From<::core::ops::RangeInclusive<T>> for Interval<T>
where
    T: Clone + PartialOrd,
{
    fn from(value: ::core::ops::RangeInclusive<T>) -> Self {
        Interval::new_closed_closed(value.start().clone(), value.end().clone())
    }
}
impl<T> ::core::convert::From<::core::ops::RangeTo<T>> for Interval<T>
where
    T: Clone + PartialOrd + Bounded,
{
    fn from(value: ::core::ops::RangeTo<T>) -> Self {
        Interval::new_unbounded_open(value.end.clone())
    }
}
impl<T> ::core::convert::From<::core::ops::RangeToInclusive<T>> for Interval<T>
where
    T: Clone + PartialOrd + Bounded,
{
    fn from(value: ::core::ops::RangeToInclusive<T>) -> Self {
        Interval::new_unbounded_closed(value.end.clone())
    }
}
impl<T> ::core::convert::From<::core::ops::RangeFrom<T>> for Interval<T>
where
    T: Clone + PartialOrd + Bounded,
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

impl<T, E> ::core::convert::TryFrom<&str> for Interval<T>
where
    T: PartialOrd + Bounded + NothingBetween + ::core::str::FromStr<Err = E>,
    E: ::core::fmt::Debug,
{
    type Error = ParseError<E>;

    /// Convert from a string to an interval.
    /// The format of the string is similar to what Display provides.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
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
