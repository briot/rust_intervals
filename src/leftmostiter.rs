/// An iterator that merges the items from two other iterators, sorted.
/// For instance, with
/// ```none
///     iter1 = [1, 3, 4, 8]
///     iter2 = [2, 3, 7]
/// ```
/// the output is
/// ```none
///     [1, 2, 3, 3, 4, 7, 8]
/// ```
/// The two iterators could be different, as long as they return the same item.
pub(crate) struct LeftMostIter<T, X, Y>
where
    X: ::core::iter::Iterator<Item = T>,
    Y: ::core::iter::Iterator<Item = T>,
{
    iter1: ::core::iter::Peekable<X>,
    iter2: ::core::iter::Peekable<Y>,
}

impl<T, X, Y> LeftMostIter<T, X, Y>
where
    X: ::core::iter::Iterator<Item = T>,
    Y: ::core::iter::Iterator<Item = T>,
{
    pub fn new(iter1: X, iter2: Y) -> Self {
        LeftMostIter {
            iter1: iter1.peekable(),
            iter2: iter2.peekable(),
        }
    }
}

impl<T, X, Y> Iterator for LeftMostIter<T, X, Y>
where
    T: PartialOrd,
    X: ::core::iter::Iterator<Item = T>,
    Y: ::core::iter::Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match (&self.iter1.peek(), &self.iter2.peek()) {
            (None, None) => None,
            (None, Some(_)) => self.iter2.next(),
            (Some(_), None) => self.iter1.next(),
            (Some(s), Some(e)) => {
                if e <= s {
                    self.iter2.next()
                } else {
                    self.iter1.next()
                }
            }
        }
    }
}


