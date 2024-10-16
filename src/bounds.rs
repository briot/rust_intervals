use crate::nothing_between::NothingBetween;
use ::core::cmp::{Ordering, PartialOrd};

/// One bound of an interval
/// LeftOf, applied to value, represents a conceptual point halfway between
/// the value and its predecessor value.
/// Likewise, RightOf represents a conceptual point halfway between the value
/// and its successor.
pub(crate) enum Bound<T> {
    LeftUnbounded,
    LeftOf(T),
    RightOf(T),
    RightUnbounded,
}

impl<T> Bound<T> {
    /// True if value is to the right of the bound
    pub(crate) fn left_of(&self, value: &T) -> bool
    where
        T: PartialOrd,
    {
        match self {
            Bound::LeftUnbounded => true,
            Bound::LeftOf(point) => *point <= *value,
            Bound::RightOf(point) => *point < *value,
            Bound::RightUnbounded => false,
        }
    }

    /// True if the value is to the left of the bound
    pub(crate) fn right_of(&self, value: &T) -> bool
    where
        T: PartialOrd,
    {
        match self {
            Bound::LeftUnbounded => false,
            Bound::LeftOf(point) => *value < *point,
            Bound::RightOf(point) => *value <= *point,
            Bound::RightUnbounded => true,
        }
    }

    pub(crate) fn min(&self, right: &Self) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if self < right {
            self.clone()
        } else {
            right.clone()
        }
    }

    pub(crate) fn max(&self, right: &Self) -> Self
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        if self > right {
            self.clone()
        } else {
            right.clone()
        }
    }

    /// Return the bound's value (which might be included in the interval
    /// or not).  This returns None for an unbounded bound.
    pub(crate) fn value(&self) -> Option<&T> {
        match self {
            Bound::LeftUnbounded | Bound::RightUnbounded => None,
            Bound::LeftOf(p) | Bound::RightOf(p) => Some(p),
        }
    }

    /// Converts from `Bound<T>` to `Bound<&T>`
    pub(crate) fn as_ref(&self) -> Bound<&T> {
        match self {
            Bound::LeftUnbounded => Bound::LeftUnbounded,
            Bound::LeftOf(point) => Bound::LeftOf(point),
            Bound::RightOf(point) => Bound::RightOf(point),
            Bound::RightUnbounded => Bound::RightUnbounded,
        }
    }
}

impl<T> ::core::fmt::Debug for Bound<T>
where
    T: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Bound::LeftUnbounded => write!(f, "-infinity")?,
            Bound::LeftOf(point) => write!(f, "LeftOf({point:?})")?,
            Bound::RightOf(point) => write!(f, "RightOf({point:?})")?,
            Bound::RightUnbounded => write!(f, "+infinity")?,
        }
        Ok(())
    }
}

impl<T> ::core::hash::Hash for Bound<T>
where
    T: ::core::hash::Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Bound::LeftUnbounded => {}
            Bound::LeftOf(point) => point.hash(state),
            Bound::RightOf(point) => point.hash(state),
            Bound::RightUnbounded => {}
        }
    }
}

impl<T> PartialEq<Bound<&T>> for Bound<T>
where
    T: PartialOrd + NothingBetween,
{
    //  Bound is never equal to an exact value.  Doesn't matter since we only
    //  compare for strict inequality
    fn eq(&self, other: &Bound<&T>) -> bool {
        match (self, other) {
            (Bound::LeftUnbounded, Bound::LeftUnbounded)
            | (Bound::RightUnbounded, Bound::RightUnbounded) => true,
            (Bound::LeftOf(s), Bound::LeftOf(o))
            | (Bound::RightOf(s), Bound::RightOf(o)) => *s == **o,
            (Bound::LeftOf(s), Bound::RightOf(o)) => match s.partial_cmp(o) {
                None | Some(Ordering::Less | Ordering::Equal) => false,
                Some(Ordering::Greater) => (*o).nothing_between(s),
            },
            (Bound::RightOf(s), Bound::LeftOf(o)) => match s.partial_cmp(o) {
                None | Some(Ordering::Equal | Ordering::Greater) => false,
                Some(Ordering::Less) => s.nothing_between(o),
            },
            (Bound::LeftUnbounded, _)
            | (_, Bound::LeftUnbounded)
            | (_, Bound::RightUnbounded)
            | (Bound::RightUnbounded, _) => false,
        }
    }
}

impl<T> PartialEq for Bound<T>
where
    T: PartialOrd + NothingBetween,
{
    //  Bound is never equal to an exact value.  Doesn't matter since we only
    //  compare for strict inequality
    fn eq(&self, other: &Bound<T>) -> bool {
        self.eq(&other.as_ref())
    }
}

impl<T> Eq for Bound<T> where T: PartialOrd + NothingBetween {}

impl<T> PartialOrd for Bound<T>
where
    T: PartialOrd + NothingBetween,
{
    fn partial_cmp(&self, other: &Bound<T>) -> Option<Ordering> {
        self.partial_cmp(&other.as_ref())
    }
}

impl<T> PartialOrd<Bound<&T>> for Bound<T>
where
    T: PartialOrd + NothingBetween,
{
    /// Two bounds (either lower and upper of same interval, or possibly
    /// lowers from two intervals) might be equivalent if there is nothing
    /// between them.
    /// For instance, for f32:
    ///     RightOf(1.0) is equivalent to LeftOf(1.0 + EPSILON)
    ///     since there is nothing between 1.0 and 1.0 + EPSILON
    /// (this would not be true when talking about mathematical reals for
    /// instance).
    /// This function returns Equal if there is nothing between the two
    /// bounds.
    fn partial_cmp(&self, other: &Bound<&T>) -> Option<Ordering> {
        match (self, other) {
            (Bound::LeftUnbounded, Bound::LeftUnbounded)
            | (Bound::RightUnbounded, Bound::RightUnbounded) => {
                Some(Ordering::Equal)
            }
            (Bound::LeftOf(s), Bound::LeftOf(o))
            | (Bound::RightOf(s), Bound::RightOf(o)) => s.partial_cmp(*o),
            (Bound::LeftOf(s), Bound::RightOf(o)) => match s.partial_cmp(o) {
                None => None,
                Some(Ordering::Less | Ordering::Equal) => Some(Ordering::Less),
                Some(Ordering::Greater) => Some(if (*o).nothing_between(s) {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }),
            },
            (Bound::RightOf(s), Bound::LeftOf(o)) => match s.partial_cmp(*o) {
                None => None,
                Some(Ordering::Less) => Some(if s.nothing_between(*o) {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }),
                Some(Ordering::Equal | Ordering::Greater) => {
                    Some(Ordering::Greater)
                }
            },
            (Bound::LeftUnbounded, _) => Some(Ordering::Less),
            (_, Bound::LeftUnbounded) => Some(Ordering::Greater),
            (_, Bound::RightUnbounded) => Some(Ordering::Less),
            (Bound::RightUnbounded, _) => Some(Ordering::Greater),
        }
    }
}

impl<T> ::core::clone::Clone for Bound<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Bound::LeftUnbounded => Bound::LeftUnbounded,
            Bound::RightUnbounded => Bound::RightUnbounded,
            Bound::LeftOf(point) => Bound::LeftOf(point.clone()),
            Bound::RightOf(point) => Bound::RightOf(point.clone()),
        }
    }
}

impl<T> Copy for Bound<T> where T: Copy {}
