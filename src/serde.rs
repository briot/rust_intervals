use crate::bounds::Bound;
use crate::nothing_between::NothingBetween;
use ::core::cmp::PartialOrd;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "Interval")]
enum SerdeInterval<T> {
    ClosedClosed(T, T),
    ClosedOpen(T, T),
    ClosedUnbounded(T),
    DoublyUnbounded,
    Empty,
    OpenClosed(T, T),
    OpenOpen(T, T),
    OpenUnbounded(T),
    UnboundedClosed(T),
    UnboundedOpen(T),
}

impl<T> Serialize for crate::intervals::Interval<T>
where
    T: PartialOrd + NothingBetween + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.is_empty() {
            SerdeInterval::<T>::Empty.serialize(serializer)
        } else {
            let intv = match (&self.lower, &self.upper) {
                (Bound::LeftOf(lo), Bound::LeftOf(up)) => {
                    SerdeInterval::ClosedOpen(lo, up)
                }
                (Bound::LeftOf(lo), Bound::RightOf(up)) => {
                    SerdeInterval::ClosedClosed(lo, up)
                }
                (Bound::LeftOf(lo), Bound::RightUnbounded) => {
                    SerdeInterval::ClosedUnbounded(lo)
                }
                (Bound::LeftUnbounded, Bound::RightUnbounded) => {
                    SerdeInterval::DoublyUnbounded
                }
                (Bound::LeftUnbounded, Bound::LeftOf(up)) => {
                    SerdeInterval::UnboundedOpen(up)
                }
                (Bound::LeftUnbounded, Bound::RightOf(up)) => {
                    SerdeInterval::UnboundedClosed(up)
                }
                (Bound::RightOf(lo), Bound::LeftOf(up)) => {
                    SerdeInterval::OpenOpen(lo, up)
                }
                (Bound::RightOf(lo), Bound::RightOf(up)) => {
                    SerdeInterval::OpenClosed(lo, up)
                }
                (Bound::RightOf(lo), Bound::RightUnbounded) => {
                    SerdeInterval::OpenUnbounded(lo)
                }
                (_, Bound::LeftUnbounded) | (Bound::RightUnbounded, _) => {
                    SerdeInterval::Empty
                }
            };
            intv.serialize(serializer)
        }
    }
}

impl<'de, T> Deserialize<'de> for crate::intervals::Interval<T>
where
    T: PartialOrd + NothingBetween + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Ok(match SerdeInterval::<T>::deserialize(deserializer)? {
            SerdeInterval::Empty => crate::intervals::Interval::empty(),
            SerdeInterval::ClosedOpen(lo, up) => {
                crate::intervals::Interval::new_closed_open(lo, up)
            }
            SerdeInterval::ClosedClosed(lo, up) => {
                crate::intervals::Interval::new_closed_closed(lo, up)
            }
            SerdeInterval::OpenClosed(lo, up) => {
                crate::intervals::Interval::new_open_closed(lo, up)
            }
            SerdeInterval::OpenOpen(lo, up) => {
                crate::intervals::Interval::new_open_open(lo, up)
            }
            SerdeInterval::OpenUnbounded(lo) => {
                crate::intervals::Interval::new_open_unbounded(lo)
            }
            SerdeInterval::ClosedUnbounded(lo) => {
                crate::intervals::Interval::new_closed_unbounded(lo)
            }
            SerdeInterval::UnboundedOpen(up) => {
                crate::intervals::Interval::new_unbounded_open(up)
            }
            SerdeInterval::UnboundedClosed(up) => {
                crate::intervals::Interval::new_unbounded_closed(up)
            }
            SerdeInterval::DoublyUnbounded => {
                crate::intervals::Interval::doubly_unbounded()
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::nothing_between::NothingBetween;
    use crate::*;
    use ::core::cmp::PartialOrd;
    use ::core::fmt::Debug;
    use ::serde::{de::DeserializeOwned, Serialize};

    fn roundtrip<
        T: PartialOrd + NothingBetween + Serialize + DeserializeOwned + Debug,
    >(
        intv: Interval<T>,
        json_str: &str,
        ron_str: &str,
    ) {
        assert_eq!(serde_json::to_string(&intv).unwrap(), json_str,);
        assert_eq!(ron::to_string(&intv).unwrap(), ron_str);
        assert_eq!(
            serde_json::from_str::<Interval<T>>(json_str).unwrap(),
            intv,
        );
        assert_eq!(ron::from_str::<Interval<T>>(ron_str).unwrap(), intv,);
    }

    #[test]
    fn test_serde() {
        roundtrip(interval!(1, 2), "{\"ClosedOpen\":[1,2]}", "ClosedOpen(1,2)");
        roundtrip(
            interval!(1, 2, "[]"),
            "{\"ClosedClosed\":[1,2]}",
            "ClosedClosed(1,2)",
        );
        roundtrip(
            interval!(1, 2, "[)"),
            "{\"ClosedOpen\":[1,2]}",
            "ClosedOpen(1,2)",
        );
        roundtrip(
            interval!(1, 3, "()"),
            "{\"OpenOpen\":[1,3]}",
            "OpenOpen(1,3)",
        );
        roundtrip(
            interval!(1, 2, "(]"),
            "{\"OpenClosed\":[1,2]}",
            "OpenClosed(1,2)",
        );
        roundtrip(
            interval!(1, "[inf"),
            "{\"ClosedUnbounded\":1}",
            "ClosedUnbounded(1)",
        );
        roundtrip(
            interval!(1, "(inf"),
            "{\"OpenUnbounded\":1}",
            "OpenUnbounded(1)",
        );
        roundtrip(
            interval!("-inf", 1.0, "]"),
            "{\"UnboundedClosed\":1.0}",
            "UnboundedClosed(1.0)",
        );
        roundtrip(
            interval!("-inf", 1.0, ")"),
            "{\"UnboundedOpen\":1.0}",
            "UnboundedOpen(1.0)",
        );
        roundtrip(Interval::<f32>::empty(), "\"Empty\"", "Empty");
        roundtrip(
            Interval::<f32>::doubly_unbounded(),
            "\"DoublyUnbounded\"",
            "DoublyUnbounded",
        );
    }
}
