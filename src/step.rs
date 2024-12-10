use ::core::convert::TryInto;

pub trait Bounded {
    /// Those two methods could also be from num_traits::Bounded
    fn min_value() -> Self;
    fn max_value() -> Self;
}

/// Similar to std::iter::Step, but the latter is unstable and cannot be used
/// in this package.  It also doesn't provide support for starting from lowest
/// value valid for the type for instance.
pub trait Step
where
    Self: ::core::marker::Sized,
{
    fn forward(&self, step: usize) -> Option<Self>;
    fn backward(&self, step: usize) -> Option<Self>;

    /// Computes the number of elements from self to other.  This function
    /// returns None if the diff cannot be computed or is larger than what
    /// usize allows.
    /// The result of this function is only used for optimization by some
    /// rust iterators functions (via `Iterator::size_hint()`).
    /// ```
    /// use rust_intervals::Step;
    /// assert_eq!(1_u8.elements_between(&3), Some(2));
    /// ```
    fn elements_between(&self, other: &Self) -> Option<usize>;
}

macro_rules! step_for_int {
    ($t:tt) => {
        impl Step for $t {
            fn forward(&self, step: usize) -> Option<Self> {
                self.checked_add(step as Self)
            }
            fn backward(&self, step: usize) -> Option<Self> {
                self.checked_sub(step as Self)
            }
            fn elements_between(&self, other: &Self) -> Option<usize> {
                match other.checked_sub(*self) {
                    None => None,
                    Some(d) => d.try_into().ok(),
                }
            }
        }
    };
}

macro_rules! bounded_for_type {
    ($t:tt) => {
        impl Bounded for $t {
            fn min_value() -> Self {
                Self::MIN
            }
            fn max_value() -> Self {
                Self::MAX
            }
        }
    };
}

step_for_int!(u8);
step_for_int!(u16);
step_for_int!(u32);
step_for_int!(u64);
step_for_int!(u128);
step_for_int!(i8);
step_for_int!(i16);
step_for_int!(i32);
step_for_int!(i64);
step_for_int!(isize);
step_for_int!(usize);

bounded_for_type!(u8);
bounded_for_type!(u16);
bounded_for_type!(u32);
bounded_for_type!(u64);
bounded_for_type!(u128);
bounded_for_type!(i8);
bounded_for_type!(i16);
bounded_for_type!(i32);
bounded_for_type!(i64);
bounded_for_type!(isize);
bounded_for_type!(usize);
bounded_for_type!(f32);
bounded_for_type!(f64);

impl Bounded for char {
    fn min_value() -> Self {
        '\0'
    }
    fn max_value() -> Self {
        Self::MAX
    }
}
