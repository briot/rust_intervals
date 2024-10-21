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

impl<T> Iterator for IntervalIterator<T>
where
    T: Step + Clone + PartialOrd + NothingBetween,
{
    type Item = T;

    /// Removes and returns an element from the start of the interval
    fn next(&mut self) -> Option<Self::Item> {
        if self.intv.is_empty() {
            return None;
        }

        match &self.intv.lower {
            Bound::LeftUnbounded => {
                let current = T::min_value();
                self.intv.lower = match current.forward() {
                    None => Bound::RightUnbounded,
                    Some(c) => Bound::LeftOf(c),
                };
                Some(current)
            }
            Bound::RightUnbounded => {
                panic!("Can only happen when interval is empty");
            }
            Bound::LeftOf(lo) => {
                let current = Some(lo.clone());
                self.intv.lower = match lo.forward() {
                    None => Bound::RightUnbounded,
                    Some(c) => Bound::LeftOf(c),
                };
                current
            }
            Bound::RightOf(lo) => {
                let current = lo.forward();
                self.intv.lower = match current {
                    None => Bound::RightUnbounded,
                    Some(ref c) => Bound::RightOf(c.clone()),
                };
                current
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
        if self.intv.is_empty() {
            return None;
        }

        match &self.intv.upper {
            Bound::LeftUnbounded => {
                panic!("Can only happen when interval is empty");
            }
            Bound::RightUnbounded => {
                let current = T::max_value();
                self.intv.upper = match current.backward() {
                    None => Bound::LeftUnbounded,
                    Some(c) => Bound::RightOf(c),
                };
                Some(current)
            }
            Bound::RightOf(up) => {
                let current = Some(up.clone());
                self.intv.upper = match up.backward() {
                    None => Bound::LeftUnbounded,
                    Some(c) => Bound::RightOf(c),
                };
                current
            }
            Bound::LeftOf(lo) => {
                let current = lo.backward();
                self.intv.upper = match current {
                    None => Bound::LeftUnbounded,
                    Some(ref c) => Bound::LeftOf(c.clone()),
                };
                current
            }
        }
    }
}
