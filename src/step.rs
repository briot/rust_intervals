/// Similar to std::iter::Step, but the latter is unstable and cannot be used
/// in this package.  It also doesn't provide support for starting from lowest
/// value valid for the type for instance.
pub trait Step
where
    Self: ::core::marker::Sized,
{
    fn lowest() -> Self;
    fn forward(&self) -> Option<Self>;
}

macro_rules! step_for_int {
    ($t:tt) => {
        impl Step for $t {
            fn lowest() -> Self {
                Self::MIN
            }
            fn forward(&self) -> Option<Self> {
                self.checked_add(1)
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
