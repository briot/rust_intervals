use rust_intervals::*;

#[derive(Debug, arbitrary::Arbitrary)]
pub enum RangeType {
    OpenClosed,
    OpenOpen,
    ClosedOpen,
    ClosedClosed,
    ClosedUnbounded,
    OpenUnbounded,
    UnboundedClosed,
    UnboundedOpen,
    Empty,
}
impl RangeType {
    pub fn build<T>(&self, lower: T, upper: T) -> Interval<T> {
        match self {
            RangeType::OpenClosed => {
                Interval::new_open_closed(lower, upper)
            }
            RangeType::OpenOpen => Interval::new_open_open(lower, upper),
            RangeType::ClosedOpen => {
                Interval::new_closed_open(lower, upper)
            }
            RangeType::ClosedClosed => {
                Interval::new_closed_closed(lower, upper)
            }
            RangeType::ClosedUnbounded => {
                Interval::new_closed_unbounded(lower)
            }
            RangeType::OpenUnbounded => Interval::new_open_unbounded(lower),
            RangeType::UnboundedClosed => {
                Interval::new_unbounded_closed(upper)
            }
            RangeType::UnboundedOpen => Interval::new_unbounded_open(upper),
            RangeType::Empty => Interval::empty(),
        }
    }
}
