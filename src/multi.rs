use crate::intervals::Interval;
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

        let mut to_insert = None;
        let last = self.0.last();

        // Special case: we are inserting at the end of self.  No need to
        // create a new vector.
        if last.is_none()   // self is empty
           || last.unwrap().strictly_left_not_contiguous(
                elements.first().unwrap())
        {
            for e in elements {
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
        } else {
            let mut old = Vec::new();
            ::core::mem::swap(&mut self.0, &mut old);
            let mut self_iter = old.into_iter().peekable();
            let mut elem_iter = elements.into_iter().peekable();

            loop {
                let left = match (&self_iter.peek(), &elem_iter.peek()) {
                    (None, None) => break,
                    (None, Some(_)) => elem_iter.next().unwrap(),
                    (Some(_), None) => self_iter.next().unwrap(),
                    (Some(s), Some(e)) => {
                        if e <= s {
                            elem_iter.next().unwrap()
                        } else {
                            self_iter.next().unwrap()
                        }
                    }
                };
                to_insert = match to_insert {
                    None => Some(left),
                    Some(ins) => match ins.union(&left) {
                        None => {
                            self.0.push(ins); // left-most is inst
                            Some(left)
                        }
                        Some(u) => Some(u),
                    },
                };
            }
        }

        // to_insert can never be none, unless we had nothing to insert
        self.0.push(to_insert.unwrap());
    }

    /// Iterate over all intervals
    pub fn iter(&self) -> impl Iterator<Item = &Interval<T>> {
        self.0.iter()
    }

    /// Check that self is well-formed:
    /// - intervals are sorted
    /// - they do not overlap
    ///
    /// This is meant for tests, and should be useless in normal code, as the
    /// various functions preserve those invariants.
    pub fn check_invariants(&self)
    where
        T: PartialOrd + NothingBetween,
    {
        for (i1, i2) in self.iter().zip(self.iter().skip(1)) {
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