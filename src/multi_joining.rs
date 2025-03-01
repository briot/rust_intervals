use crate::intervals::Interval;
use crate::leftmostiter::LeftMostIter;
use crate::multi::Policy;
use crate::nothing_between::NothingBetween;

#[derive(Debug, Default)]
pub struct Joining;

impl Joining {
    fn do_merge<T, I>(vec: &mut Vec<Interval<T>>, iter: I)
    where
        T: PartialOrd + NothingBetween + Clone,
        I: IntoIterator<Item = Interval<T>>,
    {
        let mut to_insert = None;
        for e in iter {
            to_insert = match to_insert {
                None => Some(e),
                Some(ins) => match ins.union(&e) {
                    None => {
                        vec.push(ins); // left-most is inst
                        Some(e)
                    }
                    Some(u) => Some(u),
                },
            };
        }
        if let Some(ins) = to_insert {
            vec.push(ins);
        }
    }
}

impl<T> Policy<T> for Joining {
    fn merge(vec: &mut Vec<Interval<T>>, elements: Vec<Interval<T>>)
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        match elements.first() {
            None => {}
            Some(fi) => {
                match vec.last() {
                    None => Self::do_merge(vec, elements),
                    Some(la) if la.strictly_left_not_contiguous(fi) => {
                        // Special case: we are inserting at the end of self.  No need to
                        // create a new vector.
                        Self::do_merge(vec, elements)
                    }
                    _ => {
                        let mut old = Vec::new();
                        ::core::mem::swap(vec, &mut old);
                        Self::do_merge(
                            vec,
                            LeftMostIter::new(
                                old.into_iter(),
                                elements.into_iter(),
                            ),
                        );
                    }
                }
            }
        }
    }
}
