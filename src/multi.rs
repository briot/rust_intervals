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
        T: PartialOrd + Ord + NothingBetween + Clone,
    {
        self.extend([intv]);
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
                lower: intv.upper.max(&reminder.lower),
                upper: reminder.upper,
            };
            if reminder.is_empty() {
                return true;
            }
        }
        false
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
}

impl<T, P: Policy<T>> Default for IntervalSet<T, P> {
    fn default() -> Self {
        IntervalSet::empty()
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

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use crate::multi::Policy;
    use crate::*;

    fn insert_via_trait<T, E, I>(into: &mut E, from: I)
    where
        E: Extend<Interval<T>>,
        I: IntoIterator<Item = Interval<T>>,
    {
        into.extend(from);
    }

    fn check_bounded<P>(
        mut m: IntervalSet<u32, P>,
        single: IntervalSet<u32, P>,
        expected: &[Interval<u32>],
    ) where
        P: Policy<u32> + ::core::fmt::Debug,
    {
        let empty = IntervalSet::<u32, P>::default();

        insert_via_trait(
            &mut m,
            [
                interval!(1, 3, "[)"),
                interval!(2, 4, "[)"),
                interval!(4, 6, "[)"),
                interval!(8, 10, "[)"),
            ],
        );
        assert_eq!(
            m.iter().collect::<Vec<_>>(),
            expected.iter().collect::<Vec<_>>(),
        );
        assert_eq!(m.len(), expected.len());

        m.check_invariants();

        assert_eq!(m.lower(), Some(&1));
        assert_eq!(m.upper(), Some(&10));
        assert!(!m.lower_unbounded());
        assert!(!m.upper_unbounded());

        assert!(!m.contains(0));
        assert!(m.contains(1));
        assert!(m.contains(2));
        assert!(m.contains(3));
        assert!(m.contains(4));
        assert!(m.contains(5));
        assert!(!m.contains(6));
        assert!(!m.contains(7));
        assert!(m.contains(8));
        assert!(m.contains(9));
        assert!(!m.contains(10));

        assert!(!m.contains_interval(interval!(0, 1)));
        assert!(m.contains_interval(Interval::empty()));
        assert!(!IntervalSet::<u32, P>::empty()
            .contains_interval(interval!(100, 101)));
        assert!(empty.contains_interval(Interval::empty()));
        assert!(m.contains_interval(interval!(1, 2)));
        assert!(m.contains_interval(interval!(1, 5)));
        assert!(!m.contains_interval(interval!(1, 7)));
        assert!(!m.contains_interval(interval!(21, 27)));
        #[allow(clippy::needless_borrows_for_generic_args)]
        {
            assert!(!m.contains_interval(&interval!(6, 7))); //  accepts ref
        }

        assert_eq!(m.convex_hull(), interval!(1, 10, "[)"));

        // Add an interval that covers the whole set.
        m.add(interval!(0, 18));
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(0, 18)]);
        assert_eq!(m.len(), 1);

        // Same as above, but intervals are not sorted initially
        m.clear();
        m.extend([
            interval!(4, 6, "[)"),
            interval!(8, 10, "[)"),
            interval!(2, 4, "[)"),
            interval!(1, 3, "[)"),
        ]);
        assert_eq!(
            m.iter().collect::<Vec<_>>(),
            expected.iter().collect::<Vec<_>>(),
        );
        assert_eq!(m.len(), expected.len());

        // Additional tests
        m.clear();
        m.add(interval!(1, 4));
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 4)],);
        m.check_invariants();

        m.add(interval!(1, 4)); //  Adding same interval has no effect
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 4)],);

        m.add(interval!(1, 6)); // extends the first interval
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 6)],);

        m.add(interval!(8, 10)); // disjoint
        assert_eq!(m.len(), 2);
        assert_eq!(
            m.iter().collect::<Vec<_>>(),
            vec![&interval!(1, 6), &interval!(8, 10)],
        );

        m.add(interval!(9, 10)); // subset of second interval
        assert_eq!(
            m.iter().collect::<Vec<_>>(),
            vec![&interval!(1, 6), &interval!(8, 10)],
        );

        m.add(interval!(4, 8, "[]")); // joins the two
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 10)],);
        m.check_invariants();

        // Inserting intervals only after the end of all existing ones
        m.clear();
        m.extend([interval!(1, 3), interval!(4, 5)]);
        m.extend([interval!(6, 7), interval!(8, 9)]);
        assert_eq!(m.len(), 4);
        m.check_invariants();

        // Single intervals
        {
            assert_eq!(single.len(), 1);
            assert!(single.contains(4));
            assert!(!single.contains(3));
            assert!(!single.contains(5));
        }

        // Empty intervals
        {
            let mut m = IntervalSet::<u32, P>::default();
            assert!(m.is_empty());
            assert_eq!(m.len(), 0);
            assert_eq!(m, empty);
            assert!(!m.lower_unbounded());
            assert!(!m.upper_unbounded());
            assert_eq!(m.lower(), None);
            assert_eq!(m.upper(), None);
            assert_eq!(m.convex_hull(), Interval::empty());

            m.add(Interval::empty());
            m.add(Interval::empty());
            assert!(m.is_empty());
        }

        // Unbounded intervals
        {
            let m = IntervalSet::<u32, P>::new([interval!(1, "inf")]);
            assert!(!m.lower_unbounded());
            assert!(m.upper_unbounded());
            assert_eq!(m.lower(), Some(&1));
            assert_eq!(m.upper(), None);

            let m = IntervalSet::<u32, P>::new([interval!("-inf", 1, "]")]);
            assert!(m.lower_unbounded());
            assert!(!m.upper_unbounded());
            assert_eq!(m.lower(), None);
            assert_eq!(m.upper(), Some(&1));
        }
    }

    #[test]
    fn test_joining() {
        check_bounded(
            IntervalSet::empty_joining(),
            IntervalSet::new_single_joining(4),
            &[interval!(1, 6), interval!(8, 10)],
        );

        assert_eq!(IntervalSet::new_joining([interval!(5, 6)]).len(), 1);
    }

    #[test]
    fn test_separating() {
        check_bounded(
            IntervalSet::empty_separating(),
            IntervalSet::new_single_separating(4),
            &[interval!(1, 4), interval!(4, 6), interval!(8, 10)],
        );
        assert_eq!(IntervalSet::new_separating([interval!(5, 6)]).len(), 1);
    }

    #[test]
    fn test_equals() {
        let m1 = IntervalSet::new_joining([
            interval!(3, 10, "[]"),
            interval!(15, 20, "()"),
        ]);
        let m2 = IntervalSet::new([
            interval!(2, 5, "()"),
            interval!(5, 11, "[)"),
            interval!(16, 19, "[]"),
        ]);
        assert_eq!(m1, m2);
        assert_eq!(m2, m1);
        assert!(m1.equivalent(&m2));
        assert!(m1.equivalent(m2));

        let m4 =
            IntervalSet::new([interval!(2, 5, "()"), interval!(5, 11, "[)")]);
        assert_ne!(m1, m4); // same length, different intervals
        assert_ne!(m4, m1); // same length, different intervals

        let m5 = IntervalSet::new([interval!(2, 5, "()")]);
        assert_ne!(m1, m5); // different lengths
        assert_ne!(m5, m1); // different lengths

        let m6 = IntervalSet::new_joining([
            interval!(3, 10, "[]"),
            interval!(15, 20, "()"),
            interval!(25, 30, "()"),
        ]);
        assert_ne!(m1, m6); // same initial intervals, but have more
        assert_ne!(m6, m1); // same initial intervals, but have more

        let intv1 = interval!(3, 20, "[)");
        let pairs = intv1 - interval!(10, 15, "(]");
        let m3 = IntervalSet::from_pair(pairs);
        assert_eq!(m1, m3);

        let intv1 = interval!(3, 20, "[)");
        let pairs = intv1 - interval!(10, 20, "()"); //  one interval
        let m3 = IntervalSet::from_pair(pairs);
        assert_eq!(
            m3,
            IntervalSet::<u8, Separating>::new([interval!(3, 10, "[]")])
        );
    }
}
