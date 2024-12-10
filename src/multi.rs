use crate::intervals::Interval;
use crate::multi_joining::Joining;
use crate::multi_separating::Separating;
use crate::nothing_between::NothingBetween;
use crate::pairs::Pair;
use ::core::marker::PhantomData;

pub trait Policy<T> {
    /// Internal implementation for extend().  It assumes that elements contains
    /// ordered intervals (possibly overlapping).
    /// elements always contains at least one interval.
    fn merge(vec: &mut Vec<Interval<T>>, elements: Vec<Interval<T>>)
    where
        T: Ord + NothingBetween + Clone;
}

/// A sorted list of non-overlapping intervals.
/// There are multiple ways to combine intervals, depending on the chosen
/// policy.
///
///  1. Joining
///     Intervals are joined on overlap or touch (in the case of maps: if
///     associated values are equal).
///     ```none
///        {[1------3)          }
///      +       [2------4)
///      +                 [4-5)
///      = {[1----------------5)}
///     ```
///
///  2. Separating
///     Intervals are joined on overlap, but not on touch
///     ```none
///        {[1------3)}         }
///      +       [2------4)
///      +                 [4-5)
///      = {[1-----------4)[4-5)}
///     ```
///
///  3. Splitting
///     Intervals are split on overlap.  All interval borders are
///     preserved.
///     ```none
///        {[1------3)          }
///      +       [2------4)
///      +                 [4-5)
///      = {[1-2)[2-3)[3-4)[4-5)}
///     ```
#[derive(Debug)]
pub struct IntervalSet<T, P: Policy<T> = Joining> {
    intvs: Vec<Interval<T>>,
    _policy: PhantomData<P>,
}

impl<T> IntervalSet<T, Joining> {
    pub fn empty_joining() -> Self {
        Default::default()
    }

    pub fn new_joining<I>(iter: I) -> Self
    where
        T: Ord + NothingBetween + Clone,
        I: IntoIterator<Item = Interval<T>>,
    {
        Self::new(iter)
    }

    pub fn new_single_joining(value: T) -> Self
    where
        T: Clone,
    {
        Self::new_single(value)
    }
}

impl<T> IntervalSet<T, Separating> {
    pub fn empty_separating() -> Self {
        Default::default()
    }

    pub fn new_separating<I>(iter: I) -> Self
    where
        T: Ord + NothingBetween + Clone,
        I: IntoIterator<Item = Interval<T>>,
    {
        Self::new(iter)
    }

    pub fn new_single_separating(value: T) -> Self
    where
        T: Clone,
    {
        Self::new_single(value)
    }
}

impl<T, P: Policy<T>> IntervalSet<T, P> {
    /// Returns an empty multi interval
    /// ```none
    ///    {}
    /// ```
    pub fn empty() -> Self {
        IntervalSet {
            intvs: Vec::new(),
            _policy: PhantomData,
        }
    }

    /// Create a multi-interval that contains a single value
    /// ```none
    ///    { [value, value] }
    /// ```
    pub fn new_single(value: T) -> Self
    where
        T: Clone,
    {
        IntervalSet {
            intvs: vec![Interval::new_single(value)],
            _policy: PhantomData,
        }
    }

    /// Create a multi-interval from a collection of intervals.
    /// Those intervals do not have to be sorted, or non-overlapping
    pub fn new<I>(iter: I) -> Self
    where
        T: Ord + NothingBetween + Clone,
        I: IntoIterator<Item = Interval<T>>,
    {
        let mut m = IntervalSet::empty();
        m.extend(iter);
        m
    }

    /// Create a multi-interval from a pair (returned by
    /// `Interval::difference()` for instance).
    /// It is assumed that the intervals in pair do not overlap and are
    /// sorted, as is the case when they are returned by difference().
    pub fn from_pair(pair: Pair<T>) -> Self
    where
        T: PartialOrd + NothingBetween,
    {
        IntervalSet {
            intvs: match pair {
                Pair::One(intv) => vec![intv],
                Pair::Two(intv1, intv2) => vec![intv1, intv2],
            },
            _policy: PhantomData,
        }
    }

    /// Return the lowest valid value amongst all the intervals, or None if
    /// self is empty or if the left-most interval is unbounded.
    /// This value might not actually be valid for self, if we have an
    /// open bound for instance.
    pub fn lower(&self) -> Option<&T> {
        match self.intvs.first() {
            None => None,
            Some(f) => f.lower(),
        }
    }

    /// True if the left-most interval is unbounded.
    /// This is false if self is empty.
    pub fn lower_unbounded(&self) -> bool {
        match self.intvs.first() {
            None => false,
            Some(f) => f.lower_unbounded(),
        }
    }

    /// Return the highest valid value amongst all the intervals, or None if
    /// self is empty or if the right-most interval is unbounded.
    /// This value might not actually be valid for self, if we have an
    /// open bound for instance.
    pub fn upper(&self) -> Option<&T> {
        match self.intvs.last() {
            None => None,
            Some(f) => f.upper(),
        }
    }

    /// True if the right-most interval is unbounded.
    /// This is false if self is empty.
    pub fn upper_unbounded(&self) -> bool {
        match self.intvs.last() {
            None => false,
            Some(f) => f.upper_unbounded(),
        }
    }

    /// Return the number of intervals in self.
    pub fn len(&self) -> usize {
        self.intvs.len()
    }

    /// True if there are not values in self
    pub fn is_empty(&self) -> bool {
        self.intvs.is_empty()
    }

    /// Removes all intervals from the set
    pub fn clear(&mut self) {
        self.intvs.clear();
    }

    /// Add an extra set of valid values to self.
    /// If you have multiple intervals to insert, it is more efficient to
    /// call `IntervalSet::extend()` as this requires less allocations.
    pub fn add(&mut self, intv: Interval<T>)
    where
        T: Ord + NothingBetween + Clone,
    {
        self.extend([intv]);
    }

    /// Remove the value from Self, splitting intervals as needed.
    pub fn remove(&self, value: T) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        self.remove_interval(Interval::new_single(value))
    }

    /// Remove from self all values found in intv.
    /// ```
    /// #  use rust_intervals::{interval, IntervalSet};
    ///    let set1 = IntervalSet::new_joining([interval!(1, 20)]);
    ///    let intv1 = interval!(5, 10);
    ///    let diff = set1.remove_interval(&intv1);
    ///    assert_eq!(
    ///        diff,
    ///        IntervalSet::new_joining([interval!(1, 5), interval!(10, 20)]),
    ///    );
    ///
    ///    assert_eq!(&set1 - &intv1, diff);
    ///    assert_eq!(&set1 - intv1.clone(), diff);
    ///    assert_eq!(set1.clone() - &intv1, diff);
    ///    assert_eq!(set1 - intv1, diff);
    /// ```
    pub fn remove_interval<U>(&self, intv: U) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<Interval<T>>,
    {
        let u = intv.borrow();
        let mut result = IntervalSet::empty();
        for (idx, v) in self.intvs.iter().enumerate() {
            match v.difference(u) {
                Pair::One(p1) => {
                    result.intvs.push(p1);
                }
                Pair::Two(p1, p2) => {
                    result.intvs.push(p1);
                    result.intvs.push(p2);

                    // There will be no more difference now
                    result.intvs.extend_from_slice(
                        &self.intvs[idx + 1..]);
                    break;
                }
            }
        }
        result.intvs.retain(|v| !v.is_empty());
        result
    }

    /// Add multiple sets of valid values to self, via an iterator
    pub fn extend<I>(&mut self, iter: I)
    where
        T: Ord + NothingBetween + Clone,
        I: IntoIterator<Item = Interval<T>>,
    {
        let mut elements = iter
            .into_iter()
            .filter(|intv| !intv.is_empty())
            .collect::<Vec<_>>();
        if !elements.is_empty() {
            elements.sort();
            P::merge(&mut self.intvs, elements);
        }
    }

    /// Iterate over all intervals
    pub fn iter(&self) -> impl Iterator<Item = &Interval<T>> {
        self.intvs.iter()
    }

    /// Whether the two sets contain the same set of values
    pub fn equivalent<U>(&self, other: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Self>,
    {
        let u = other.borrow();
        self.iter().eq(u.iter())
    }

    /// Whether value is valid for any of the intervals in self
    pub fn contains<V>(&self, value: V) -> bool
    where
        T: PartialOrd + NothingBetween,
        V: ::core::borrow::Borrow<T>,
    {
        let t = value.borrow();
        for intv in self.iter() {
            if !intv.lower.left_of(t) {
                return false;
            } else if intv.upper.right_of(t) {
                return true;
            }
        }
        false
    }

    /// Whether all values in other are valid for self.
    /// All sets always contain the empty interval.
    pub fn contains_interval<U>(&self, other: U) -> bool
    where
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<Interval<T>>,
    {
        let u = other.borrow();
        if u.is_empty() {
            return true;
        }

        // In the case of joining, other must be fully contained in one of the
        // nested intervals.  We do not need T to be Clone either.
        // Unfortunately Rust doesn't let us specialize the function.
        //    self.iter().any(|v| v.contains_interval(u))

        if self.intvs.is_empty() {
            return false;
        }

        let first = self.intvs.first().unwrap();
        if u.lower < first.lower {
            return false;
        }
        let mut reminder = Interval {
            // simplified difference()
            lower: first.upper.max(&u.lower),
            upper: u.upper.clone(),
        };
        if reminder.is_empty() {
            return true;
        }

        for intv in self.iter().skip(1) {
            if reminder.lower < intv.lower {
                return false;
            }
            reminder = Interval {
                //  simplified difference()
                lower: intv.upper.max(&reminder.lower),
                upper: reminder.upper,
            };
            if reminder.is_empty() {
                return true;
            }
        }
        false
    }

    /// Returns the intersection of self and intv.
    /// This could return any number of intervals.
    pub fn intersection_interval<U>(&self, intv: U) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<Interval<T>>,
    {
        let mut result = IntervalSet::empty();
        let u = intv.borrow();
        if u.is_empty() || self.intvs.is_empty() {
            return result;
        }

        for v in &self.intvs {
            if u.upper < v.lower {
                break;
            }

            let inters = v.intersection(u);
            if !inters.is_empty() {
                result.intvs.push(inters);
            }
        }
        return result;
    }

    /// Returns the intersection of self and intv.
    /// This could return any number of intervals.
    pub fn intersection_set<U, P2>(&self, intv: U) -> Self
    where
        P2: Policy<T>,
        T: PartialOrd + NothingBetween + Clone,
        U: ::core::borrow::Borrow<IntervalSet<T, P2>>,
    {
        let mut result = IntervalSet::empty();
        let u = intv.borrow();
        if u.is_empty() || self.intvs.is_empty() {
            return result;
        }

        for v in &self.intvs {
            let inters = u.intersection_interval(v);
            if !inters.is_empty() {
                result.intvs.extend(inters.intvs);
            }
            if u.left_of_interval(v) {
                break;
            }
        }
        return result;
    }

    /// Returns the convex hull, i.e. the smallest intervals that contains
    /// all values in all intervals in self.  The result might contain
    /// additional values that were not valid for self.
    pub fn convex_hull(&self) -> Interval<T>
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if self.is_empty() {
            Interval::empty()
        } else {
            Interval {
                lower: self.intvs.first().unwrap().lower.clone(),
                upper: self.intvs.last().unwrap().upper.clone(),
            }
        }
    }

    /// Check that self is well-formed:
    /// - intervals are sorted
    /// - they do not overlap
    /// - none of the intervals is empty
    ///
    /// This is meant for tests, and should be useless in normal code, as the
    /// various functions preserve those invariants.
    #[cfg_attr(test, mutants::skip)]
    pub fn check_invariants(&self)
    where
        T: PartialOrd + NothingBetween,
    {
        let mut it = self.iter();
        if let Some(first) = it.next() {
            assert!(!first.is_empty());
        }
        for (i1, i2) in self.iter().zip(it) {
            assert!(!i2.is_empty());
            assert!(i1.strictly_left_of_interval(i2));
        }
    }

    /// Whether any value exists in both self and right.
    #[doc(alias = "overlaps")]
    pub fn intersects_interval<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Interval<T>>,
    {
        let u = right.borrow();
        self.iter().any(|v| v.intersects(u))
    }

    /// Whether any value exists in both self and right.
    #[doc(alias = "overlaps")]
    pub fn intersects_set<U, P2>(&self, right: U) -> bool
    where
        P2: Policy<T>,
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<IntervalSet<T, P2>>,
    {
        let u = right.borrow();
        self.iter().any(|v| u.intersects_interval(v))
    }

    /// Whether every value in self is less (<=) than right
    /// Returns True if either set is empty.
    pub fn left_of<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<T>,
    {
        match self.intvs.last() {
            None => true,
            Some(l) => l.left_of(right.borrow()),
        }
    }

    /// Whether every value in self is strictly less (<) than right
    /// Returns True if either set is empty.
    pub fn strictly_left_of<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<T>,
    {
        match self.intvs.last() {
            None => true,
            Some(l) => l.strictly_left_of(right.borrow()),
        }
    }

    /// Whether every value in self is less (<=) then all values in right.
    /// Returns True if either set is empty.
    pub fn left_of_interval<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Interval<T>>,
    {
        match self.intvs.last() {
            None => true,
            Some(l) => l.left_of_interval(right.borrow()),
        }
    }

    /// Whether every value in self is strictly less (<) then all values in
    /// right.
    /// Returns True if either set is empty.
    pub fn strictly_left_of_interval<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Interval<T>>,
    {
        match self.intvs.last() {
            None => true,
            Some(l) => l.strictly_left_of_interval(right.borrow()),
        }
    }

    /// Whether every value in self is less then (<=) all values in right.
    /// Returns True if either set is empty.
    pub fn left_of_set<U, P2>(&self, right: U) -> bool
    where
        P2: Policy<T>,
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<IntervalSet<T, P2>>,
    {
        match right.borrow().intvs.first() {
            None => true,
            Some(r) => self.left_of_interval(r),
        }
    }

    /// Whether every value in self is greater or equal (>=) to right
    /// Returns True if either set is empty.
    pub fn right_of<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<T>,
    {
        match self.intvs.first() {
            None => true,
            Some(l) => l.right_of(right.borrow()),
        }
    }

    /// Whether every value in self is strictly greater (>) then right
    /// Returns True if either set is empty.
    pub fn strictly_right_of<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<T>,
    {
        match self.intvs.first() {
            None => true,
            Some(l) => l.strictly_right_of(right.borrow()),
        }
    }

    /// Whether every value in self is greater or equal (>=) than all values
    /// in right.
    /// Returns True if either set is empty.
    pub fn right_of_interval<U>(&self, right: U) -> bool
    where
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<Interval<T>>,
    {
        match self.intvs.first() {
            None => true,
            Some(l) => l.right_of_interval(right.borrow()),
        }
    }

    /// Whether every value in self is greater or equal (>=) than all values
    /// in right.
    /// Returns True if either set is empty.
    pub fn right_of_set<U, P2>(&self, right: U) -> bool
    where
        P2: Policy<T>,
        T: PartialOrd + NothingBetween,
        U: ::core::borrow::Borrow<IntervalSet<T, P2>>,
    {
        right.borrow().left_of_set(self)
    }
}

impl<T, P: Policy<T>> Default for IntervalSet<T, P> {
    fn default() -> Self {
        IntervalSet::empty()
    }
}

impl<T, P: Policy<T>> ::core::clone::Clone for IntervalSet<T, P>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            intvs: self.intvs.clone(),
            _policy: self._policy,
        }
    }
}

impl<T, P: Policy<T>> Extend<Interval<T>> for IntervalSet<T, P>
where
    T: PartialOrd + Ord + NothingBetween + Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Interval<T>>,
    {
        self.extend(iter);
    }
}

impl<T, P: Policy<T>> PartialEq for IntervalSet<T, P>
where
    T: PartialOrd + NothingBetween,
{
    fn eq(&self, other: &Self) -> bool {
        self.equivalent(other)
    }
}

///   &IntervalSet - &Interval
///   and &IntervalSet - Interval
impl<T, U, P: Policy<T>> ::core::ops::Sub<U> for &IntervalSet<T, P>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = IntervalSet<T, P>;

    /// Same as [`IntervalSet::remove_interval()`]
    fn sub(self, rhs: U) -> Self::Output {
        self.remove_interval(rhs)
    }
}

///   IntervalSet - &Interval
///   and IntervalSet - Interval
impl<T, U, P: Policy<T>> ::core::ops::Sub<U> for IntervalSet<T, P>
where
    T: PartialOrd + NothingBetween + Clone,
    U: ::core::borrow::Borrow<Interval<T>>,
{
    type Output = IntervalSet<T, P>;

    /// Same as [`IntervalSet::remove_interval()`]
    fn sub(self, rhs: U) -> Self::Output {
        self.remove_interval(rhs)
    }
}
