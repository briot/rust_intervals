use crate::bounds::Bound;
use crate::step::Step;

pub struct IntervalIterator<T> {
    current: Option<T>,
    max: Bound<T>,
}

impl<T> IntervalIterator<T>
    where T: Step + Clone + PartialOrd
{
    /// Create a new iterator
    pub(crate) fn new(lower: &Bound<T>, upper: &Bound<T>) -> Self {
        match lower {
            Bound::LeftUnbounded => IntervalIterator {
                current: Some(T::lowest()),
                max: upper.clone(),
            },
            Bound::LeftOf(lo) => IntervalIterator {
                current: Some(lo.clone()),
                max: upper.clone(),
            },
            Bound::RightOf(lo) => IntervalIterator {
                current: lo.forward(),
                max: upper.clone(),
            },
            Bound::RightUnbounded => IntervalIterator {
                current: None,
                max: upper.clone(),
            },
        }
    }
}

impl<T> Iterator for IntervalIterator<T>
where
    T: Step + Clone + PartialOrd
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            None => None,
            Some(c) => {
                let mut n = c.forward().and_then(|v| {
                    if self.max.left_of(&v) {
                        None
                    } else {
                        Some(v)
                    }
                });
                ::core::mem::swap(&mut self.current, &mut n);
                n
            }
        }
    }
}


