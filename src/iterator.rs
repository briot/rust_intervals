use crate::bounds::Bound;
use crate::intervals::Interval;
use crate::nothing_between::NothingBetween;
use crate::step::Step;

pub struct IntervalIterator<T>
where
    T: Step + Clone + PartialOrd + NothingBetween,
{
    pub(crate) intv: Interval<T>,
}

impl<T> IntervalIterator<T>
where
    T: Step + Clone + PartialOrd + NothingBetween,
{
    /// Return an interval matching what the iterators will return
    pub fn as_interval(&self) -> Interval<T> {
        self.intv.clone()
    }

    /// Internal implementation for nth() and next()
    fn internal_nth(&mut self, n: usize) -> Option<T> {
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
    fn internal_nth_back(&mut self, n: usize) -> Option<T> {
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
    type Item = T;

    /// Removes and returns an element from the start of the interval
    fn next(&mut self) -> Option<Self::Item> {
        self.internal_nth(0)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.internal_nth(n)
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
