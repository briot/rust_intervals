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
    pub fn add<U>(&mut self, intv: U)
    where
        T: PartialOrd + NothingBetween + Clone + ::core::fmt::Debug,
        U: ::core::borrow::Borrow<Interval<T>>,
    {
        let element_ref = intv.borrow();

        if element_ref.is_empty() {
            return;
        }

        let mut element = (*element_ref).clone();
        let mut output = Vec::new();
        let mut merged = false;

        let mut iter = self.0.iter();
        while let Some(v) = iter.next() {
            match v.union(&element) {
                None => {
                    if element.strictly_left_of_interval(v) {
                        output.push(element.clone());
                        output.push(v.clone()); // push current element
                        output.extend(iter.cloned()); // push remaining elems
                        merged = true;
                        break;
                    }
                    output.push(v.clone());
                }
                Some(u) => {
                    element = u;
                }
            }
        }

        if !merged {
            output.push(element);
        }
        self.0 = output;
    }

    /// Add multiple sets of valid valid to self, via an iterator
    pub fn extend<I>(&mut self, iter: I)
    where
        T: PartialOrd + NothingBetween + Clone + ::core::fmt::Debug,
        I: IntoIterator<Item = Interval<T>>,
    {
        for intv in iter {
            self.add(intv);
        }
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
    T: PartialOrd + NothingBetween + Clone + ::core::fmt::Debug,
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
        m.extend([interval!(1, 3), interval!(2, 4), interval!(4, 5)]);
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

        m.add(interval!(4, 8, "[]")); // joins the two
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 10)],);
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
