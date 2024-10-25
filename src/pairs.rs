use crate::intervals::Interval;
use crate::nothing_between::NothingBetween;

/// The result of the difference between two intervals.  This might be a
/// single interval, or two (in which case the first one is always the
/// left-most).
#[derive(PartialEq)]
pub enum Pair<T>
where
    T: PartialOrd + NothingBetween,
{
    One(Interval<T>),
    Two(Interval<T>, Interval<T>),
}

impl<T> ::core::fmt::Debug for Pair<T>
where
    T: ::core::fmt::Debug + PartialOrd + NothingBetween,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Pair::One(i1) => write!(f, "{:?}", *i1)?,
            Pair::Two(i1, i2) => write!(f, "({:?} + {:?})", i1, i2)?,
        };
        Ok(())
    }
}

impl<T> Pair<T>
where
    T: PartialOrd + NothingBetween,
{
    pub(crate) fn new_from_two(intv1: Interval<T>, intv2: Interval<T>) -> Self {
        if intv1.is_empty() {
            Pair::One(intv2)
        } else if intv2.is_empty() {
            Pair::One(intv1)
        } else {
            Pair::Two(intv1, intv2)
        }
    }
}
