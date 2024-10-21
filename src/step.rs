/// Similar to std::iter::Step, but the latter is unstable and cannot be used
/// in this package.  It also doesn't provide support for starting from lowest
/// value valid for the type for instance.
pub trait Step
where
    Self: ::core::marker::Sized,
{
    /// Those two methods could also be from num_traits::Bounded
    fn min_value() -> Self;
    fn max_value() -> Self;

    fn forward(&self, step: usize) -> Option<Self>;
    fn backward(&self, step: usize) -> Option<Self>;
}

macro_rules! step_for_int {
    ($t:tt) => {
        impl Step for $t {
            fn min_value() -> Self {
                Self::MIN
            }
            fn max_value() -> Self {
                Self::MAX
            }
            fn forward(&self, step: usize) -> Option<Self> {
                self.checked_add(step as $t)
            }
            fn backward(&self, step: usize) -> Option<Self> {
                self.checked_sub(step as $t)
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
