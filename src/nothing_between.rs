/// Trait to compute whether intervals are empty.
///
/// For instance, for f32, we consider the numbers as representable on
/// the machine.  So an interval like:
/// `[1.0, 1.0 + f32::EPSILON)`
/// is empty, since we cannot represent any number from this interval.
///
/// ```
///    use rust_intervals::Interval;
///    assert!(Interval::new_open_open(1.0, 1.0 + f32::EPSILON)
///        .is_empty());
/// ```
///
/// But if you implement your own wrapper type as
/// ```
///    use rust_intervals::Interval;
///    use rust_intervals::NothingBetween;
///    #[derive(PartialEq, PartialOrd)]
///    struct Real(f32);
///    impl NothingBetween for Real {
///        fn nothing_between(&self, _other: &Self) -> bool {
///            false
///        }
///    }
///    assert!(!Interval::new_open_open(Real(1.0), Real(1.0 + f32::EPSILON))
///        .is_empty());
/// ```
/// then the same interval `[Real(1.0), Real(1.0 + f32::EPSILON)]` is
/// no longer empty, even though we cannot represent any number from this
/// interval.
pub trait NothingBetween {
    fn nothing_between(&self, other: &Self) -> bool;
    //  Should return True if no value exists between self and other in this
    //  type.
    //  This is only called with self < other.
}

macro_rules! nothing_between_for_int {
    ($t:tt) => {
        impl NothingBetween for $t {
            fn nothing_between(&self, other: &Self) -> bool {
                other - self <= 1
            }
        }
    };
}

nothing_between_for_int!(u8);
nothing_between_for_int!(u16);
nothing_between_for_int!(u32);
nothing_between_for_int!(u64);
nothing_between_for_int!(u128);
nothing_between_for_int!(i8);
nothing_between_for_int!(i16);
nothing_between_for_int!(i32);
nothing_between_for_int!(i64);
nothing_between_for_int!(i128);
nothing_between_for_int!(usize);
nothing_between_for_int!(isize);

impl NothingBetween for f32 {
    fn nothing_between(&self, other: &Self) -> bool {
        // In general, comparing with EPSILON is wrong.  There are however two
        // cases:
        // * the user has used V + EPSILON with a large V.  The addition had no
        //   effect, and this sum is equal to V.

        // Note that this is incorrect for large values of floats, since adding
        // EPSILON has no effect.
        self + f32::EPSILON >= *other
    }
}
impl NothingBetween for f64 {
    fn nothing_between(&self, other: &Self) -> bool {
        self + f64::EPSILON >= *other
    }
}
impl NothingBetween for char {
    fn nothing_between(&self, other: &Self) -> bool {
        (*other as u32) - (*self as u32) <= 1
    }
}

#[cfg(feature = "std")]
impl NothingBetween for std::time::Duration {
    fn nothing_between(&self, other: &Self) -> bool {
        other.as_nanos() - self.as_nanos() <= 1
    }
}

/// Blanket implementation so that we can create intervals of references (used in
/// particular to avoid cloning in the implementation of left_of)
impl<T: NothingBetween> NothingBetween for &T {
    fn nothing_between(&self, other: &Self) -> bool {
        (*self).nothing_between(*other)
    }
}
