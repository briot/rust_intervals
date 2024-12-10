use crate::intervals::Interval;
use crate::leftmostiter::LeftMostIter;
use crate::multi::Policy;
use crate::nothing_between::NothingBetween;

#[derive(Debug, Default)]
pub struct Separating;

impl Separating {
    fn do_merge<T, I>(vec: &mut Vec<Interval<T>>, iter: I)
    where
        T: PartialOrd + NothingBetween + Clone,
        I: IntoIterator<Item = Interval<T>>,
    {
        let mut to_insert = None;
        for e in iter {
            to_insert = match to_insert {
                None => Some(e),
                Some(ins) => {
                    if ins.strictly_left_of_interval(&e) {
                        vec.push(ins); // left-most is inst
                        Some(e)
                    } else {
                        Some(ins.convex_hull(e))
                    }
                }
            };
        }
        vec.push(to_insert.unwrap());
    }
}

impl<T> Policy<T> for Separating {
    fn merge(vec: &mut Vec<Interval<T>>, elements: Vec<Interval<T>>)
    where
        T: PartialOrd + NothingBetween + Clone,
    {
        // Special case: we are inserting at the end of self.  No need to
        // create a new vector.
        let last = vec.last();
        if last.is_none()
            || last
                .unwrap()
                .strictly_left_of_interval(elements.first().unwrap())
        {
            Self::do_merge(vec, elements);
        } else {
            let mut old = Vec::new();
            ::core::mem::swap(vec, &mut old);
            Self::do_merge(
                vec,
                LeftMostIter::new(old.into_iter(), elements.into_iter()),
            );
        }
    }
}
