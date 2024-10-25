use crate::intervals::Interval;
use crate::leftmostiter::LeftMostIter;
use crate::nothing_between::NothingBetween;

/// A sorted list of non-overlapping intervals.
///
/// So for instance, if the MultiInterval initially contains
/// ```none
///    [-------)(-----]      [--------]
/// ```
/// and then extend it with a new interval
/// ```none
///          [----]
/// ```
/// We end up with
/// ```none
///    [--------------]      [--------]
/// ```
#[derive(Default, Debug)]
pub struct MultiInterval<T>(Vec<Interval<T>>);

impl<T> MultiInterval<T> {
    /// Return the number of intervals in self.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// True if there are not values in self
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add an extra set of valid values to self.
    /// If you have multiple intervals to insert, it is more efficient to
    /// call `MultiInterval::extend()` as this requires less allocations.
    pub fn add(&mut self, intv: Interval<T>)
    where
        T: PartialOrd + Ord + NothingBetween + Clone,
    {
        self.extend([intv]);
    }

    /// Internal implementation for extend().  It assumes that iter returns
    /// ordered intervals (possibly overlapping).  It will append those
    /// intervals at the end of self, so it also assumes that the intervals
    /// returned by I should be to the right of self.
    /// Also assumes iter returns at least one element.
    fn extend_internal<I>(&mut self, iter: I)
    where
        T: Ord + NothingBetween + Clone,
        I: IntoIterator<Item = Interval<T>>,
    {
        let mut to_insert = None;
        for e in iter {
            to_insert = match to_insert {
                None => Some(e),
                Some(ins) => match ins.union(&e) {
                    None => {
                        self.0.push(ins); // left-most is inst
                        Some(e)
                    }
                    Some(u) => Some(u),
                },
            };
        }
        self.0.push(to_insert.unwrap());
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
        if elements.is_empty() {
            return;
        }
        elements.sort();

        // Special case: we are inserting at the end of self.  No need to
        // create a new vector.
        let last = self.0.last();
        if last.is_none()   // self is empty
           || last.unwrap().strictly_left_not_contiguous(
                elements.first().unwrap())
        {
            self.extend_internal(elements);
        } else {
            let mut old = Vec::new();
            ::core::mem::swap(&mut self.0, &mut old);
            self.extend_internal(LeftMostIter::new(
                old.into_iter(),
                elements.into_iter(),
            ));
        }
    }

    /// Iterate over all intervals
    pub fn iter(&self) -> impl Iterator<Item = &Interval<T>> {
        self.0.iter()
    }

    /// Check that self is well-formed:
    /// - intervals are sorted
    /// - they do not overlap
    /// - none of the intervals is empty
    ///
    /// This is meant for tests, and should be useless in normal code, as the
    /// various functions preserve those invariants.
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

impl<T> Extend<Interval<T>> for MultiInterval<T>
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

impl<T> PartialEq for MultiInterval<T>
where
    T: PartialOrd + NothingBetween,
{
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn insert_via_extend<T, E, I>(into: &mut E, from: I)
    where
        E: Extend<Interval<T>>,
        I: IntoIterator<Item = Interval<T>>,
    {
        into.extend(from);
    }

    #[test]
    fn test_joining() {
        // From Boost ICL library:
        // There are multiple ways to combine intervals.
        //
        //  1. Joining
        //     Intervals are joined on overlap or touch (in the case of maps: if
        //     associated values are equal).
        //        {[1      3)          }
        //      +       [2      4)
        //      +                 [4 5)
        //      = {[1                5)}

        let mut m = MultiInterval::default();
        insert_via_extend(
            &mut m,
            [interval!(1, 3), interval!(2, 4), interval!(4, 5)],
        );
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 5)],);
        assert_eq!(m.len(), 1);

        //  Same as above, but intervals are not sorted initially
        let mut m = MultiInterval::default();
        m.extend([interval!(4, 5), interval!(2, 4), interval!(1, 3)]);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 5)],);
        assert_eq!(m.len(), 1);

        // Additional tests

        let mut m = MultiInterval::default();
        m.add(interval!(1, 4));
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 4)],);

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
        let mut m = MultiInterval::default();
        m.extend([interval!(1, 3), interval!(4, 5)]);
        m.extend([interval!(6, 7), interval!(8, 9)]);
        assert_eq!(m.len(), 4);
        m.check_invariants();
    }

    #[test]
    fn test_separating() {
        //  2. Separating
        //     Intervals are joined on overlap, but not on touch
        //        {[1      3)}         }
        //      +       [2      4)
        //      +                 [4 5)
        //      = {[1           4)[4 5)}
        //
        //  3. Splitting
        //     Intervals are split on overlap.  All interval borders are
        //     preserved.
        //        {[1      3)          }
        //      +       [2      4)
        //      +                 [4 5)
        //      = {[1 2)[2 3)[3 4)[4 5)}
    }

    #[test]
    fn test_empty() {
        let mut m = MultiInterval::<u32>::default();
        let empty = MultiInterval::<u32>::default();

        assert!(m.is_empty());
        assert_eq!(m.len(), 0);
        assert_eq!(m, empty);

        m.add(Interval::empty());
        m.add(Interval::empty());
        assert!(m.is_empty());
    }
}
