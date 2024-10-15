pub trait NothingBetween {
    fn nothing_between(&self, other: &Self) -> bool;
    //  Should return True if no value exists between self and other in this
    //  type.
    //  This is only called with self < other.
}

impl NothingBetween for u8 {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
impl NothingBetween for u16 {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
impl NothingBetween for u32 {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
impl NothingBetween for u64 {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
impl NothingBetween for u128 {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
impl NothingBetween for i8 {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
impl NothingBetween for i16 {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
impl NothingBetween for i32 {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
impl NothingBetween for i64 {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
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
impl NothingBetween for usize {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}
impl NothingBetween for isize {
    fn nothing_between(&self, other: &Self) -> bool {
        other - self <= 1
    }
}

#[cfg(feature = "std")]
impl NothingBetween for std::time::Duration {
    fn nothing_between(&self, other: &Self) -> bool {
        other.as_nanos() - self.as_nanos() <= 1
    }
}

// Blanket implementation so that we can create intervals of references (used in
// particular to avoid cloning in the implementation of left_of)
impl<T: NothingBetween> NothingBetween for &T {
    fn nothing_between(&self, other: &Self) -> bool {
        (*self).nothing_between(*other)
    }
}

#[cfg(feature = "chrono")]
impl<T: chrono::TimeZone> NothingBetween for chrono::DateTime<T> {
    fn nothing_between(&self, other: &Self) -> bool {
        other.clone().signed_duration_since(self)
            <= chrono::TimeDelta::nanoseconds(1)
    }
}

#[cfg(feature = "chrono")]
impl NothingBetween for chrono::NaiveDate {
    fn nothing_between(&self, other: &Self) -> bool {
        other.signed_duration_since(*self) <= chrono::TimeDelta::days(1)
    }
}
