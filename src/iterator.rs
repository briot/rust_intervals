use crate::bounds::Bound;
use crate::intervals::Interval;
use crate::nothing_between::NothingBetween;
use crate::step::Step;

pub struct IntervalIterator<T> {
    pub(crate) intv: Interval<T>,
}

impl<T> IntervalIterator<T> {
    /// Return an interval matching what the iterators will return
    pub fn as_interval(&self) -> Interval<T>
    where
        T: Clone,
    {
        self.intv.clone()
    }

    /// Internal implementation for nth() and next()
    fn internal_nth(&mut self, n: usize) -> Option<T>
    where
        T: Step + Clone + PartialOrd + NothingBetween,
    {
        if self.intv.is_empty() {
            return None;
        }

        match &self.intv.lower {
            Bound::LeftUnbounded => {
                let current = T::min_value().forward(n);
                self.intv.lower =
                    match current.clone().and_then(|c| c.forward(1)) {
                        None => Bound::RightUnbounded,
                        Some(ref c) => Bound::LeftOf(c.clone()),
                    };
                current
            }
            Bound::RightUnbounded => {
                panic!("Can only happen when interval is empty");
            }
            Bound::LeftOf(lo) => {
                let current = lo.forward(n);
                self.intv.lower =
                    match current.clone().and_then(|c| c.forward(1)) {
                        None => Bound::RightUnbounded,
                        Some(c) => Bound::LeftOf(c),
                    };
                current
            }
            Bound::RightOf(lo) => {
                let current = lo.forward(n).and_then(|c| c.forward(1));
                self.intv.lower = match current {
                    None => Bound::RightUnbounded,
                    Some(ref c) => Bound::RightOf(c.clone()),
                };
                current
            }
        }
    }

    /// Internal implementation for nth_back() and next_back()
    fn internal_nth_back(&mut self, n: usize) -> Option<T>
    where
        T: Step + Clone + PartialOrd + NothingBetween,
    {
        if self.intv.is_empty() {
            return None;
        }

        match &self.intv.upper {
            Bound::LeftUnbounded => {
                panic!("Can only happen when interval is empty");
            }
            Bound::RightUnbounded => {
                let current = T::max_value().backward(n);
                self.intv.upper =
                    match current.clone().and_then(|c| c.backward(1)) {
                        None => Bound::LeftUnbounded,
                        Some(c) => Bound::RightOf(c),
                    };
                current
            }
            Bound::RightOf(up) => {
                let current = up.backward(n);
                self.intv.upper =
                    match current.clone().and_then(|c| c.backward(1)) {
                        None => Bound::LeftUnbounded,
                        Some(c) => Bound::RightOf(c),
                    };
                current
            }
            Bound::LeftOf(lo) => {
                let current = lo.backward(n).and_then(|c| c.backward(1));
                self.intv.upper = match current {
                    None => Bound::LeftUnbounded,
                    Some(ref c) => Bound::LeftOf(c.clone()),
                };
                current
            }
        }
    }
}

impl<T> Iterator for IntervalIterator<T>
where
    T: Step + Clone + PartialOrd + NothingBetween,
{
    type Item = T; // ??? Should this be &T to match what vectors do

    /// Removes and returns an element from the start of the interval
    fn next(&mut self) -> Option<Self::Item> {
        self.internal_nth(0)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.internal_nth(n)
    }

    /// Used to compute the result of `ExactSizeIterator::len()`, and
    /// optimize calls to collect() by pre-allocating when possible.
    #[cfg_attr(test, mutants::skip)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.intv.is_empty() {
            (0, Some(0))
        } else {
            let len = match (&self.intv.lower, &self.intv.upper) {
                (Bound::RightUnbounded, _) | (_, Bound::LeftUnbounded) => {
                    Some(0)
                }
                (Bound::LeftUnbounded, Bound::RightUnbounded) => {
                    T::min_value().elements_between(&T::max_value())
                }
                (Bound::LeftUnbounded, Bound::LeftOf(up)) => {
                    T::min_value().elements_between(up)
                }
                (Bound::LeftUnbounded, Bound::RightOf(up)) => {
                    T::min_value().elements_between(up).map(|c| c + 1)
                }
                (Bound::LeftOf(lo), Bound::RightUnbounded) => {
                    lo.elements_between(&T::max_value())
                }
                (Bound::LeftOf(lo), Bound::LeftOf(up)) => {
                    lo.elements_between(up)
                }
                (Bound::LeftOf(lo), Bound::RightOf(up)) => {
                    lo.elements_between(up).map(|c| c + 1)
                }
                (Bound::RightOf(lo), Bound::RightUnbounded) => {
                    lo.elements_between(&T::max_value())
                }
                (Bound::RightOf(lo), Bound::LeftOf(up)) => {
                    lo.elements_between(up).map(|c| c - 1)
                }
                (Bound::RightOf(lo), Bound::RightOf(up)) => {
                    lo.elements_between(up)
                }
            };
            match len {
                None => (usize::MAX, None),
                Some(l) => (l, Some(l)),
            }
        }
    }
}

impl<T> DoubleEndedIterator for IntervalIterator<T>
where
    T: Step + Clone + PartialOrd + NothingBetween,
{
    /// Removes and returns an element from the end of the interval
    fn next_back(&mut self) -> Option<Self::Item> {
        self.internal_nth_back(0)
    }

    fn nth_back(&mut self, n: usize) -> Option<T> {
        self.internal_nth_back(n)
    }
}

/// len() will panic! if the number of values in the range is
/// greater than usize::MAX.
impl<T> ExactSizeIterator for IntervalIterator<T> where
    T: Step + Clone + PartialOrd + NothingBetween
{
}
