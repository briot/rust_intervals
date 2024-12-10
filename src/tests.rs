#[cfg(test)]
pub(crate) mod test {
    use crate::{bounds::Bound, *};
    use ::core::cmp::Ordering;
    use ::core::convert::{TryFrom, TryInto};
    use ::core::fmt::Debug;

    #[cfg(feature = "std")]
    use ::core::hash::{Hash, Hasher};

    // In the world of real, there is always something in-between, even if
    // we cannot represent it.  However, in this case we may have an interval
    // for which is_empty() return false, but which actually contain no
    // values, e.g.  (A, A + f32::EPSILON)
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    struct Mathf32(f32);
    impl NothingBetween for Mathf32 {
        fn nothing_between(&self, _other: &Self) -> bool {
            false
        }
    }

    macro_rules! assert_err {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!(
                  "expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
            }
        }
    }

    /// Compares the positions of an interval and a value
    macro_rules! assert_lr {
        ($intv:expr, $v:expr,
         $strictly_left_of:expr,
         $left_of:expr,
         $right_of:expr,
         $strictly_right_of:expr
        ) => {
            assert_eq!(
                $intv.strictly_left_of(&$v),
                $strictly_left_of,
                "{} strictly_left_of {} ({:?})",
                $intv,
                $v,
                $intv,
            );
            assert_eq!(
                $intv.left_of(&$v),
                $left_of,
                "{} left_of {} ({:?})",
                $intv,
                $v,
                $intv,
            );
            assert_eq!(
                $intv.right_of(&$v),
                $right_of,
                "{} right_of {} ({:?})",
                $intv,
                $v,
                $intv,
            );
            assert_eq!(
                $intv.strictly_right_of(&$v),
                $strictly_right_of,
                "{} strictly_right_of {} ({:?})",
                $intv,
                $v,
                $intv,
            );
        };
    }

    /// Compares the positions of two intervals
    macro_rules! assert_lr_intv {
        ($intv:expr, $v:expr,
         $strictly_left_of:expr,
         $left_of:expr,
         $right_of:expr,
         $strictly_right_of:expr
        ) => {
            assert_eq!(
                $intv.strictly_left_of_interval(&$v),
                $strictly_left_of,
                "{} strictly_left_of {} ({:?} and {:?})",
                $intv,
                $v,
                $intv,
                $v,
            );
            assert_eq!(
                $intv.left_of_interval(&$v),
                $left_of,
                "{} left_of {} ({:?} and {:?})",
                $intv,
                $v,
                $intv,
                $v,
            );
            assert_eq!(
                $intv.right_of_interval(&$v),
                $right_of,
                "{} right_of {} ({:?} and {:?})",
                $intv,
                $v,
                $intv,
                $v,
            );
            assert_eq!(
                $intv.strictly_right_of_interval(&$v),
                $strictly_right_of,
                "{} strictly_right_of {} ({:?} and {:?})",
                $intv,
                $v,
                $intv,
                $v,
            );
        };
    }

    fn assert_copy<T: ::core::marker::Copy>() {}

    fn assert_equivalent<T: PartialOrd + NothingBetween + Debug>(
        left: &Interval<T>,
        right: &Interval<T>,
    ) {
        assert_eq!(left, right);
        assert_eq!(right, left);
        assert!(left.equivalent(right));
        assert!(right.equivalent(left));
    }
    fn assert_not_equivalent<T: PartialOrd + NothingBetween + Debug>(
        left: &Interval<T>,
        right: &Interval<T>,
    ) {
        assert_ne!(left, right);
        assert_ne!(right, left);
        assert!(!left.equivalent(right));
        assert!(!right.equivalent(left));
    }

    fn assert_eq_and_hash<
        T: ::core::hash::Hash + PartialOrd + Debug + Step + NothingBetween,
    >(
        left: &Interval<T>,
        right: &Interval<T>,
    ) {
        assert_equivalent(left, right);

        #[cfg(feature = "std")]
        {
            let mut ls = ::std::hash::DefaultHasher::new();
            (*left).hash(&mut ls);

            let mut rs = ::std::hash::DefaultHasher::new();
            right.hash(&mut rs);

            assert_eq!(ls.finish(), rs.finish());
        }
    }

    fn assert_ne_and_hash<
        T: ::core::hash::Hash + PartialOrd + Debug + Step + NothingBetween,
    >(
        left: &Interval<T>,
        right: &Interval<T>,
    ) {
        assert_not_equivalent(left, right);

        #[cfg(feature = "std")]
        {
            let mut ls = ::std::hash::DefaultHasher::new();
            (*left).hash(&mut ls);

            let mut rs = ::std::hash::DefaultHasher::new();
            right.hash(&mut rs);

            assert_ne!(ls.finish(), rs.finish());
        }
    }

    pub fn check_empty<T: PartialOrd + NothingBetween + Copy>(
        name: &str,
        val: T,
        val_next: T,
        val_far: T,
        val_farther: T,
    ) {
        assert!(Interval::new_closed_open(val, val).is_empty(), "{}", name);
        assert!(
            !Interval::new_closed_closed(val, val).is_empty(),
            "{}",
            name
        );

        assert!(
            !Interval::new_closed_closed(val, val_next).is_empty(),
            "{}",
            name
        );
        assert!(
            Interval::new_open_open(val, val_next).is_empty(),
            "{}",
            name
        );
        assert!(
            Interval::new_open_open(val_next, val).is_empty(),
            "{}",
            name
        );

        assert!(
            !Interval::new_open_open(val, val_far).is_empty(),
            "{}",
            name
        );

        assert!(
            !Interval::new_open_open(val, val_farther).is_empty(),
            "{}",
            name
        );
    }

    #[test]
    fn test_copy() {
        assert_copy::<Interval<u32>>();
        //  assert_copy::<Interval<Vec<u32>>>();
    }

    #[test]
    fn test_contains() {
        let empty = Interval::empty();

        let intv = Interval::new_closed_open(1, 10); // [1,10)
        assert!(intv.contains(1));
        assert!(intv.contains(2));
        assert!(intv.contains(9));
        assert!(!intv.contains(10));
        assert!(!intv.contains(11));
        assert!(intv.contains_interval(empty));
        assert!(!empty.contains_interval(intv));

        let intv2 = Interval::new_closed_closed(1, 5); // [1,5]
        assert!(intv2.contains(1));
        assert!(intv2.contains(5));
        assert!(!intv2.contains(6));
        assert!(intv2.contains_interval(empty));
        assert!(!empty.contains_interval(intv2));
        assert!(intv.contains_interval(intv2));
        assert!(!intv2.contains_interval(intv));

        let intv3 = Interval::new_unbounded_closed(10); // (,10]
        assert!(intv3.contains(0));
        assert!(intv3.contains(9));
        assert!(intv3.contains(10));
        assert!(!intv3.contains(11));
        assert!(intv3.contains_interval(empty));
        assert!(!empty.contains_interval(intv3));
        assert!(intv3.contains_interval(intv));
        assert!(!intv.contains_interval(intv3));
        assert!(intv3.contains_interval(intv2));
        assert!(!intv2.contains_interval(intv3));

        let intv4 = Interval::new_unbounded_open(10); // (,10)
        assert!(intv4.contains(0));
        assert!(intv4.contains(9));
        assert!(!intv4.contains(10));
        assert!(!intv4.contains(11));
        assert!(intv4.contains_interval(empty));
        assert!(!empty.contains_interval(intv4));
        assert!(intv4.contains_interval(intv));
        assert!(!intv.contains_interval(intv4));
        assert!(intv4.contains_interval(intv2));
        assert!(!intv2.contains_interval(intv4));
        assert!(intv3.contains_interval(intv4));
        assert!(!intv4.contains_interval(intv3));

        let intv5 = Interval::new_closed_unbounded(1); // [1,)
        assert!(!intv5.contains(0));
        assert!(intv5.contains(1));
        assert!(intv5.contains(10));
        assert!(intv5.contains(11));
        assert!(intv5.contains_interval(empty));
        assert!(!empty.contains_interval(intv5));
        assert!(intv5.contains_interval(intv));
        assert!(!intv.contains_interval(intv5));
        assert!(intv5.contains_interval(intv2));
        assert!(!intv2.contains_interval(intv5));
        assert!(!intv3.contains_interval(intv5));
        assert!(!intv5.contains_interval(intv3));
        assert!(!intv4.contains_interval(intv5));
        assert!(!intv5.contains_interval(intv4));

        let intv6 = Interval::doubly_unbounded();
        assert!(intv6.contains(0));
        assert!(intv6.contains(1));
        assert!(intv6.contains(10));
        assert!(intv6.contains(11));
        assert!(intv6.contains_interval(empty));
        assert!(!empty.contains_interval(intv6));
        assert!(intv6.contains_interval(intv));
        assert!(!intv.contains_interval(intv6));
        assert!(intv6.contains_interval(intv2));
        assert!(!intv2.contains_interval(intv6));
        assert!(!intv3.contains_interval(intv6));
        assert!(intv6.contains_interval(intv3));
        assert!(!intv4.contains_interval(intv6));
        assert!(intv6.contains_interval(intv4));
        assert!(!intv5.contains_interval(intv6));
        assert!(intv6.contains_interval(intv5));

        // An interval with not comparable bounds is always empty
        let intv7 = Interval::new_closed_open(1.0, f32::NAN);
        assert!(!intv7.contains(1.0));

        let intv8 = Interval::new_open_closed(1, 10); // (1,10)
        assert!(!intv8.contains(1));
        assert!(intv8.contains(2));
        assert!(intv8.contains(9));
        assert!(intv8.contains(10));
        assert!(!intv8.contains(11));
        assert!(intv8.contains_interval(empty));
        assert!(!empty.contains_interval(intv8));
    }

    #[test]
    fn test_inclusive() {
        let intv = Interval::new_closed_open(1, 10);
        assert_eq!(intv.lower(), Some(&1));
        assert!(intv.lower_inclusive());
        assert_eq!(intv.upper(), Some(&10));
        assert!(!intv.upper_inclusive());

        let intv = Interval::new_closed_closed(1, 10);
        assert_eq!(intv.lower(), Some(&1));
        assert!(intv.lower_inclusive());
        assert_eq!(intv.upper(), Some(&10));
        assert!(intv.upper_inclusive());

        let intv = Interval::<f32>::doubly_unbounded();
        assert_eq!(intv.lower(), None);
        assert!(!intv.lower_inclusive());
        assert_eq!(intv.upper(), None);
        assert!(!intv.upper_inclusive());

        let intv = Interval::<f32>::new_open_unbounded(1.0); //  (1,)
        assert_eq!(intv.lower(), Some(&1.0));
        assert!(!intv.lower_inclusive());
        assert_eq!(intv.upper(), None);
        assert!(!intv.upper_inclusive());

        let intv = Interval::<f32>::new_unbounded_closed(10.0); //  (,10.0]
        assert_eq!(intv.lower(), None);
        assert!(!intv.lower_inclusive());
        assert_eq!(intv.upper(), Some(&10.0));
        assert!(intv.upper_inclusive());

        let intv = Interval::<f32>::empty();
        assert_eq!(intv.lower(), None); //  matches postgres
        assert!(!intv.lower_inclusive());
        assert_eq!(intv.upper(), None); //  matches postgres
        assert!(!intv.upper_inclusive());

        let empty2 = Interval::new_open_closed(3, 3);
        assert_eq!(empty2.lower(), Some(&3)); //  doesn't match postgres
        assert!(!empty2.lower_inclusive());
        assert_eq!(empty2.upper(), Some(&3)); //  doesn't match postgres
        assert!(empty2.upper_inclusive());

        let intv = Interval::<f32>::new_single(1.0);
        assert_eq!(intv.lower(), Some(&1.0));
        assert!(intv.lower_inclusive());
        assert_eq!(intv.upper(), Some(&1.0));
        assert!(intv.upper_inclusive());
    }

    #[test]
    fn test_empty() {
        check_empty("u8", &0_u8, &1, &2, &10);
        check_empty("u8", 0_u8, 1, 2, 10);
        check_empty("u16", 0_u16, 1, 2, 10);
        check_empty("u32", 0_u32, 1, 2, 10);
        check_empty("u64", 0_u64, 1, 2, 10);
        check_empty("u128", 0_u128, 1, 2, 10);
        check_empty("usize", 0_usize, 1, 2, 10);
        check_empty("i8", 0_i8, 1, 2, 10);
        check_empty("i16", 0_i16, 1, 2, 10);
        check_empty("i32", 0_i32, 1, 2, 10);
        check_empty("i64", 0_i64, 1, 2, 10);
        check_empty("isize", 0_isize, 1, 2, 10);
        check_empty("f32", 0_f32, f32::EPSILON, 2.0 * f32::EPSILON, 1.0);
        check_empty("f64", 0_f64, f64::EPSILON, 2.0 * f64::EPSILON, 1.0);
        check_empty("char", 'a', 'b', 'c', 'f');

        let empty = Interval::<f32>::empty();
        assert!(empty.is_empty());
        assert!(!empty.contains(1.1));

        let empty2 = Interval::new_closed_open(10.0_f32, 10.0);
        assert_equivalent(&empty, &empty2);

        // In mathematical representation, an infinite number of reals between
        // 1.0 and one_eps
        let real_1 = Mathf32(1.0);
        let real_1_eps = Mathf32(1.0 + f32::EPSILON);
        assert!(!Interval::new_closed_closed(real_1, real_1_eps).is_empty());
        assert!(!Interval::new_closed_open(real_1, real_1_eps).is_empty());
        assert!(!Interval::new_open_closed(real_1, real_1_eps).is_empty());
        assert!(!Interval::new_open_open(real_1, real_1_eps).is_empty());

        // When the bounds cannot be compared, the interval is empty
        assert!(Interval::new_closed_open(1.0, f32::NAN).is_empty());
        assert!(Interval::new_closed_closed(1.0, f32::NAN).is_empty());
        assert!(Interval::new_open_closed(1.0, f32::NAN).is_empty());
        assert!(Interval::new_open_open(1.0, f32::NAN).is_empty());
        assert!(Interval::new_closed_open(f32::NAN, 1.0).is_empty());
        assert!(Interval::new_closed_closed(f32::NAN, 1.0).is_empty());
        assert!(Interval::new_open_closed(f32::NAN, 1.0).is_empty());
        assert!(Interval::new_open_open(f32::NAN, 1.0).is_empty());

        assert!(!Interval::new_unbounded_closed(5.0).is_empty());
        assert!(!Interval::new_unbounded_open(5.0).is_empty());
        assert!(!Interval::new_closed_unbounded(5.0).is_empty());
        assert!(!Interval::new_open_unbounded(5.0).is_empty());
        assert!(!Interval::<u32>::doubly_unbounded().is_empty());

        #[cfg(feature = "std")]
        check_empty(
            "Duration",
            std::time::Duration::from_nanos(1),
            std::time::Duration::from_nanos(2),
            std::time::Duration::from_secs(1),
            std::time::Duration::from_secs(10),
        );
    }

    #[test]
    fn test_single() {
        let intv = Interval::new_single(4);
        assert!(!intv.is_empty());
        assert!(intv.is_single());
        assert!(intv.contains(4));
        assert!(!intv.contains(5));

        let intv = Interval::new_single(f32::NAN);
        assert!(intv.is_empty());
        assert!(!intv.is_single());

        assert!(!Interval::new_closed_open(1, 4).is_single());
        assert!(Interval::new_closed_closed(1, 1).is_single());
        assert!(Interval::new_closed_closed(1.0, 1.0).is_single());

        // An interval that contains a single element, but is not of the form
        // [A,A] will return false for is_single
        assert!(!Interval::new_open_open(0, 2).is_single());
    }

    #[test]
    fn test_equivalent() {
        let intv1 = Interval::new_closed_open(1, 4);
        let intv2 = Interval::new_closed_closed(1, 3);
        let intv4 = Interval::new_open_closed(0, 3);
        let intv5 = Interval::new_open_open(0, 4);
        let intv6 = Interval::new_open_open(-1, 3);
        let intv7 = Interval::new_closed_closed(1, 5);
        assert_eq_and_hash(&intv1, &intv1);
        assert_eq_and_hash(&intv1, &intv2);
        assert_eq_and_hash(&intv1, &intv4);
        assert_eq_and_hash(&intv1, &intv5);
        assert_eq_and_hash(&intv5, &intv2);
        assert_ne_and_hash(&intv1, &intv7);
        assert_ne_and_hash(&intv5, &intv6);

        let intv3 = Interval::new_closed_closed(1, 4);
        assert_ne_and_hash(&intv1, &intv3);
        assert_ne_and_hash(&intv2, &intv3);

        // Note: this will fail when using larger values than 1.0, because
        // f32 cannot distinguish between 4.0 and 4.0 - EPSILON for instance.
        // But that would be user-error, not an issue with intervals.
        let f1 = Interval::new_closed_open(0.0, 1.0);
        let f2 = Interval::new_closed_closed(0.0, 1.0);
        assert_not_equivalent(&f1, &f2);
        let f3 = Interval::new_closed_closed(0.0, 1.0 - f32::EPSILON);
        assert_equivalent(&f1, &f3);

        let r1 = Interval::new_closed_open(Mathf32(0.0), Mathf32(1.0));
        let r2 = Interval::new_closed_closed(Mathf32(0.0), Mathf32(1.0));
        assert_not_equivalent(&r1, &r2);
        let r3 = Interval::new_closed_closed(
            Mathf32(0.0),
            Mathf32(1.0 - f32::EPSILON),
        );
        assert_not_equivalent(&r1, &r3);

        let u1 = Interval::new_unbounded_open(10);
        let u2 = Interval::new_unbounded_closed(9);
        assert_eq_and_hash(&u1, &u2);
        assert_ne_and_hash(&u1, &intv1);

        let u1 = Interval::new_open_unbounded(9);
        let u2 = Interval::new_closed_unbounded(10);
        assert_eq_and_hash(&u1, &u2);
        assert_ne_and_hash(&u1, &intv1);

        let empty = Interval::default();
        assert_eq_and_hash(&empty, &empty);
        assert_ne_and_hash(&empty, &intv1);
    }

    /// Using FromStr trait
    #[test]
    fn test_fromstr() -> Result<(), ParseError<::core::num::ParseIntError>> {
        assert_eq!("[1,4]".parse::<Interval<u32>>()?, interval!(1, 4, "[]"));
        assert_eq!("[ 1, 4)".parse::<Interval<u32>>()?, interval!(1, 4, "[)"));
        assert_eq!("(1,4)".parse::<Interval<u8>>()?, interval!(1, 4, "()"));
        assert_eq!("(1,4  ]".parse::<Interval<u8>>()?, interval!(1, 4, "(]"));
        assert_eq!("(1,)".parse::<Interval<u8>>()?, interval!(1, "(inf"));
        assert_eq!("[1,  )".parse::<Interval<u64>>()?, interval!(1, "[inf"));
        assert_eq!("(,1)".parse::<Interval<u8>>()?, interval!("-inf", 1, ")"));
        assert_eq!("( ,1]".parse::<Interval<u8>>()?, interval!("-inf", 1, "]"));
        assert_eq!(
            "(,)".parse::<Interval<u8>>()?,
            Interval::doubly_unbounded()
        );
        assert_eq!("empty".parse::<Interval<i32>>()?, Interval::empty());
        assert_err!(
            "&1,2".parse::<Interval<i32>>(),
            Err(ParseError::InvalidInput)
        );
        assert!("&1,2".parse::<Interval<i32>>().is_err());
        Ok(())
    }

    /// Test From<Interval<T>> -> String
    #[cfg(feature = "std")]
    #[test]
    fn test_from_interval() {
        let intv = Interval::new_open_open(1, 4);
        assert_eq!(String::from(intv), "(1, 4)");

        // Implemented automatically via same trait
        let intv = Interval::new_open_open(1, 4);
        let s: String = intv.into();
        assert_eq!(s, "(1, 4)");
    }

    /// Test From<&str> -> Interval
    #[test]
    fn test_into_interval() -> Result<(), ParseError<::core::num::ParseIntError>>
    {
        // Will panic if the string is incorrect
        let intv = Interval::<u32>::try_from("[1,4]")?;
        assert_eq!(intv, interval!(1, 4, "[]"));

        // Using  Into<Interval<T>>  for String, via same trait
        let intv: Interval<u32> = "[1,4]".try_into()?;
        assert_eq!(intv, interval!(1, 4, "[]"));

        Ok(())
    }

    /// Test Display and ToString traits
    #[cfg(feature = "std")]
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Interval::new_closed_closed(1, 4)), "[1, 4]");
        assert_eq!(format!("{}", Interval::new_closed_open(1, 4)), "[1, 4)");
        assert_eq!(Interval::new_closed_open(1, 4).to_string(), "[1, 4)");
        assert_eq!(Interval::new_open_closed(1, 4).to_string(), "(1, 4]");
        assert_eq!(Interval::new_open_open(1, 4).to_string(), "(1, 4)");
        assert_eq!(Interval::new_closed_unbounded(1).to_string(), "[1,)");
        assert_eq!(Interval::new_open_unbounded(1).to_string(), "(1,)");
        assert_eq!(Interval::new_unbounded_closed(1).to_string(), "(, 1]");
        assert_eq!(Interval::new_unbounded_open(1).to_string(), "(, 1)");
        assert_eq!(Interval::<f32>::doubly_unbounded().to_string(), "(,)");
        assert_eq!(Interval::<f32>::empty().to_string(), "empty");
        assert_eq!(
            format!("{}", Interval::new_closed_closed(1.0_f32, 4.0 - 0.1)),
            "[1, 3.9]",
        );
        assert_eq!(
            format!("{}", Interval::new_closed_closed(1.0, 4.0 - f32::EPSILON)),
            "[1, 4]",
        );
        assert_eq!(
            format!(
                "{:?}",
                Interval::new_closed_closed(1.0, 4.0 - f32::EPSILON)
            ),
            "(LeftOf(1.0),RightOf(4.0))",
        );
        assert_eq!(
            format!("{:?}", Interval::<f32>::empty()),
            "(+infinity,-infinity)"
        );
        assert_eq!(
            format!("{:?}", Interval::<f32>::doubly_unbounded()),
            "(-infinity,+infinity)"
        );
    }

    #[test]
    fn test_ord() {
        let b1 = Bound::LeftOf(3); //  2 < b1 < 3 < b2 < 4
        let b2 = Bound::RightOf(3);
        assert!(b1 != b2);
        assert!(b1 < b2);

        let b3 = Bound::LeftOf(4);
        assert!(b3 == b2);
        assert!(b2 == b3);

        let intv1 = interval!(1, 20, "[]");
        let intv2 = interval!(1, 20, "()");
        assert_eq!(intv1.partial_cmp(&intv1), Some(Ordering::Equal));
        assert_eq!(intv1.partial_cmp(&intv2), Some(Ordering::Less));
        assert_eq!(intv2.partial_cmp(&intv1), Some(Ordering::Greater));
        assert_eq!(intv1.cmp(&intv1), Ordering::Equal);
        assert_eq!(intv1.cmp(&intv2), Ordering::Less);
        assert_eq!(intv2.cmp(&intv1), Ordering::Greater);
        assert!(intv1 < intv2);

        let intv3 = interval!(1, 30, "[]");
        assert_eq!(intv1.partial_cmp(&intv3), Some(Ordering::Less));
        assert_eq!(intv3.partial_cmp(&intv1), Some(Ordering::Greater));
        assert_eq!(intv1.cmp(&intv3), Ordering::Less);
        assert_eq!(intv3.cmp(&intv1), Ordering::Greater);
        assert!(intv1 < intv3);

        let intv4 = interval!("-inf", 20, "]");
        assert_eq!(intv1.partial_cmp(&intv4), Some(Ordering::Greater));
        assert_eq!(intv4.partial_cmp(&intv1), Some(Ordering::Less));
        assert_eq!(intv1.cmp(&intv4), Ordering::Greater);
        assert_eq!(intv4.cmp(&intv1), Ordering::Less);
        assert!(intv1 > intv4);

        let intv5 = interval!(1, "[inf");
        assert_eq!(intv1.partial_cmp(&intv5), Some(Ordering::Less));
        assert_eq!(intv5.partial_cmp(&intv1), Some(Ordering::Greater));
        assert_eq!(intv1.cmp(&intv5), Ordering::Less);
        assert_eq!(intv5.cmp(&intv1), Ordering::Greater);
        assert!(intv1 < intv5);

        let empty = Interval::empty();
        assert_eq!(intv1.partial_cmp(&empty), Some(Ordering::Greater));
        assert_eq!(empty.partial_cmp(&intv1), Some(Ordering::Less));
        assert_eq!(intv1.cmp(&empty), Ordering::Greater);
        assert_eq!(empty.cmp(&intv1), Ordering::Less);

        let intv1 = interval!(1.0, f32::NAN); // actually empty
        let intv2 = interval!(1.0, 3.0);
        assert_eq!(intv1.partial_cmp(&intv2), Some(Ordering::Less));
        assert_eq!(intv2.partial_cmp(&intv1), Some(Ordering::Greater));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_map() {
        let mut map = std::collections::HashMap::new();
        map.insert(interval!(1, 2, "[]"), 2);
        map.insert(interval!(1, 10, "()"), 3); // same lower, different upper
        map.insert(interval!(2, 10, "[)"), 3); // overrides
        map.insert(interval!(1, "(inf"), 4);
        map.insert(interval!("-inf", 20, "]"), 5);
        assert_eq!(map.len(), 4);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_hash() {
        assert_ne_and_hash(&interval!(1, 2, "[]"), &interval!(1, 2, "(]"));
        assert_ne_and_hash(&interval!(1, 2, "[]"), &interval!(1, 2, "[)"));

        // Intervals are equal, so the hash must also be equal
        assert_eq_and_hash(&interval!(1, 2, "[]"), &interval!(0, 2, "(]"));
        assert_eq_and_hash(&interval!(1, 2, "[]"), &interval!(0, 3, "()"));
        assert_eq_and_hash(&interval!(1, "[inf"), &interval!(0, "(inf"));
        assert_eq_and_hash(
            &interval!("-inf", 2, "]"),
            &interval!("-inf", 3, ")"),
        );
    }

    #[test]
    fn test_left_of() {
        fn check<T>(
            lower_m: T,
            lower: T,
            lower_p: T,
            upper_m: T,
            upper: T,
            upper_p: T,
        ) where
            T: PartialOrd
                + NothingBetween
                + Clone
                + Debug
                + ::core::fmt::Display,
        {
            for lower_inc in [true, false] {
                for upper_inc in [true, false] {
                    let intv = match (lower_inc, upper_inc) {
                        (true, true) => Interval::new_closed_closed(
                            lower.clone(),
                            upper.clone(),
                        ),
                        (true, false) => Interval::new_closed_open(
                            lower.clone(),
                            upper.clone(),
                        ),
                        (false, true) => Interval::new_open_closed(
                            lower.clone(),
                            upper.clone(),
                        ),
                        (false, false) => Interval::new_open_open(
                            lower.clone(),
                            upper.clone(),
                        ),
                    };
                    assert_lr!(intv, lower_m, false, false, true, true);
                    assert_lr!(intv, lower, false, false, true, !lower_inc);
                    assert_lr!(intv, lower_p, false, false, !lower_inc, false);
                    assert_lr!(intv, upper_m, false, !upper_inc, false, false);
                    assert_lr!(intv, upper, !upper_inc, true, false, false);
                    assert_lr!(intv, upper_p, true, true, false, false);

                    let v = Interval::new_closed_unbounded(upper_m.clone());
                    assert_lr_intv!(intv, v, false, !upper_inc, false, false);

                    let v = Interval::new_closed_unbounded(upper.clone());
                    assert_lr_intv!(intv, v, !upper_inc, true, false, false);

                    let v = Interval::new_closed_unbounded(upper_p.clone());
                    assert_lr_intv!(intv, v, true, true, false, false);

                    let v = Interval::new_open_unbounded(upper_m.clone());
                    assert_lr_intv!(
                        intv, v, !upper_inc, !upper_inc, false, false
                    );

                    let v = Interval::new_open_unbounded(upper.clone());
                    assert_lr_intv!(intv, v, true, true, false, false);

                    let v = Interval::new_open_unbounded(upper_p.clone());
                    assert_lr_intv!(intv, v, true, true, false, false);

                    let v = Interval::new_unbounded_closed(lower_m.clone());
                    assert_lr_intv!(intv, v, false, false, true, true);

                    let v = Interval::new_unbounded_closed(lower.clone());
                    assert_lr_intv!(intv, v, false, false, true, !lower_inc);

                    let v = Interval::new_unbounded_closed(lower_p.clone());
                    assert_lr_intv!(intv, v, false, false, false, false);

                    let v = Interval::new_unbounded_open(lower_m.clone());
                    assert_lr_intv!(intv, v, false, false, true, true);

                    let v = Interval::new_unbounded_open(lower.clone());
                    assert_lr_intv!(intv, v, false, false, true, true);

                    let v = Interval::new_unbounded_open(lower_p.clone());
                    assert_lr_intv!(intv, v, false, false, true, !lower_inc);

                    let empty = Interval::empty();
                    assert!(empty.strictly_left_of_interval(&intv));
                    assert!(intv.strictly_left_of_interval(&empty));
                    assert!(empty.left_of_interval(&intv));
                    assert!(intv.left_of_interval(&empty));
                    assert!(empty.right_of_interval(&intv));
                    assert!(intv.right_of_interval(&empty));
                    assert!(intv.strictly_right_of_interval(&empty));
                    assert!(empty.strictly_right_of_interval(&intv));
                }
            }
        }

        check(3, 4, 5, 7, 8, 9);
        check('c', 'd', 'e', 'm', 'n', 'o');
        check(
            // Note this test will fail with larger floats, since EPSILON
            // is too small.
            0.0 - f32::EPSILON,
            0.0,
            0.0 + f32::EPSILON,
            1.0 - f32::EPSILON,
            1.0,
            1.0 + f32::EPSILON,
        );

        let empty = Interval::<i8>::empty();
        assert!(empty.strictly_left_of(1));
        assert!(empty.left_of(1));
        assert!(empty.strictly_right_of(1));
        assert!(empty.right_of(1));

        let intv7 = Interval::new_unbounded_closed(10_i16);
        assert!(!intv7.right_of(0));
        assert!(!intv7.strictly_right_of(0));

        let intv8 = Interval::new_closed_unbounded(10_i16);
        assert!(!intv8.left_of(0));
        assert!(!intv8.strictly_left_of(0));
    }

    #[test]
    fn test_ref() {
        let intv1 = Interval::<&char>::new_closed_closed(&'A', &'Z');
        assert!(!intv1.is_empty());
        assert!(intv1.contains(&'B'));
        assert!(!intv1.contains(&'a'));

        let intv2 = Interval::<char>::new_closed_closed('A', 'Z');
        assert!(intv2.as_ref().contains_interval(intv1));
    }

    #[test]
    fn test_convex_hull() {
        let intv1 = Interval::new_closed_closed(10, 30);
        let intv2 = Interval::new_closed_closed(40, 50);
        assert_eq!(
            intv1.convex_hull(intv2),
            Interval::new_closed_closed(10, 50)
        );
        assert_eq!(
            intv2.convex_hull(intv1),
            Interval::new_closed_closed(10, 50)
        );

        let intv1 = Interval::new_closed_closed(10, 30);
        let intv2 = Interval::new_closed_closed(20, 30); // nested
        assert_eq!(intv1.convex_hull(intv2), intv1);
        assert_eq!(intv2.convex_hull(intv1), intv1);
        assert_eq!(intv2.union(intv1), Some(intv1));

        let intv1 = Interval::new_open_open(10, 30);
        let intv2 = Interval::new_open_open(40, 50); // nested
        assert_eq!(intv1.convex_hull(intv2), Interval::new_open_open(10, 50));
        assert_eq!(intv2.convex_hull(intv1), Interval::new_open_open(10, 50));
        assert_eq!(intv2.union(intv1), None); //  not contiguous

        let intv1 = Interval::empty();
        let intv2 = Interval::new_open_open(40, 50); // nested
        assert_eq!(intv1.convex_hull(intv2), intv2);
        assert_eq!(intv2.convex_hull(intv1), intv2);
        assert_eq!(intv2.union(intv1), Some(intv2));

        let intv1 = Interval::new_open_unbounded(10);
        let intv2 = Interval::new_open_open(40, 50); // nested
        assert_eq!(intv1.convex_hull(intv2), intv1);
        assert_eq!(intv2.convex_hull(intv1), intv1);
        assert_eq!(intv2.union(intv1), Some(intv1));

        let intv1 = Interval::new_unbounded_open(10);
        let intv2 = Interval::new_open_open(40, 50); // nested
        assert_eq!(intv1.convex_hull(intv2), Interval::new_unbounded_open(50));
        assert_eq!(intv2.convex_hull(intv1), Interval::new_unbounded_open(50));
        assert_eq!(intv2.union(intv1), None);
    }

    #[test]
    fn test_difference() {
        let intv1 = Interval::new_closed_closed(10, 30);
        let empty = Interval::<i32>::empty();
        assert_eq!(intv1.difference(empty), Pair::One(intv1));
        assert_eq!(empty.difference(intv1), Pair::One(empty));

        let intv2 = Interval::new_closed_closed(1, 50); //  larger
        assert_eq!(intv1.difference(intv2), Pair::One(empty));
        assert_eq!(
            intv2.difference(intv1),
            Pair::Two(
                Interval::new_closed_open(1, 10),
                Interval::new_open_closed(30, 50),
            )
        );

        #[cfg(feature = "std")]
        assert_eq!(
            format!("{:?}", intv2.difference(intv1)),
            "((LeftOf(1),LeftOf(10)) + (RightOf(30),RightOf(50)))"
        );

        let intv3 = Interval::new_closed_closed(1, 5); // disjoint
        assert_eq!(intv1.difference(intv3), Pair::One(intv1));
        assert_eq!(intv3.difference(intv1), Pair::One(intv3));

        #[cfg(feature = "std")]
        assert_eq!(
            format!("{:?}", intv1.difference(intv3)),
            "(LeftOf(10),RightOf(30))"
        );

        let intv4 = Interval::new_closed_closed(1, 15); // overlaps left
        assert_eq!(
            intv1.difference(intv4),
            Pair::One(Interval::new_open_closed(15, 30))
        );

        let intv5 = Interval::new_closed_closed(25, 40); // overlaps right
        assert_eq!(
            intv1.difference(intv5),
            Pair::One(Interval::new_closed_open(10, 25))
        );
    }

    #[test]
    fn test_unusual_bounds() {
        // We can actually declare intervals for types that we can't even
        // compare, although a lot of the functions are not available
        let intv1 = Interval::new_closed_open("abc", "def");
        assert_eq!(intv1.lower(), Some(&"abc"));
        assert!(intv1.lower_inclusive());
        assert!(!intv1.lower_unbounded());
        assert_eq!(intv1.upper(), Some(&"def"));
        assert!(!intv1.upper_inclusive());
        assert!(!intv1.upper_unbounded());

        let intv2 = Interval::new_closed_unbounded("abc");
        assert_eq!(intv2.lower(), Some(&"abc"));
        assert!(intv2.lower_inclusive());
        assert!(!intv2.lower_unbounded());
        assert_eq!(intv2.upper(), None);
        assert!(!intv2.upper_inclusive());
        assert!(intv2.upper_unbounded());

        #[cfg(feature = "std")]
        {
            let intv3 =
                Interval::new_closed_open("abc".to_string(), "def".to_string());
            let _intv4 = intv3.as_ref();
        }

        let intv5 = Interval::new_closed_open('a', 'c');
        assert!(!intv5.is_empty());

        // With references
        let intv5 = Interval::new_closed_open(&'a', &'c');
        assert!(!intv5.is_empty());
    }

    #[test]
    fn test_between() {
        let intv1 = Interval::new_closed_closed(10, 30);
        let intv2 = Interval::new_closed_closed(40, 50);
        let intv3 = Interval::new_open_unbounded(35);
        let empty = Interval::empty();
        assert_eq!(intv1.between(intv2), Interval::new_open_open(30, 40),);
        assert_eq!(intv1.between(intv3), Interval::new_open_closed(30, 35),);
        assert_eq!(intv2.between(intv3), empty.clone(),);
        assert_eq!(intv1.between(empty), empty.clone(),);
        assert_eq!(empty.between(intv1), empty.clone(),);
        assert!(intv1.contiguous(intv1));
        assert!(!intv1.contiguous(intv2));
        assert!(!intv1.contiguous(intv3));
        assert!(intv2.contiguous(intv3));
        assert!(empty.contiguous(intv1));
        assert!(intv1.contiguous(empty));
    }

    #[test]
    fn test_intersection() {
        let intv1 = Interval::new_closed_closed(10_u8, 30);
        let intv2 = Interval::new_closed_open(40_u8, 50);
        let intv3 = Interval::new_open_unbounded(35_u8);
        let empty = Interval::empty();
        assert!(!intv1.intersects(intv2));
        assert_eq!(intv1.intersection(intv2), empty.clone());
        assert!(intv2.intersects(intv3));
        assert_eq!(
            intv2.intersection(intv3),
            Interval::new_closed_open(40, 50)
        );
    }

    #[test]
    fn test_symmetric_difference() {
        let intv1 = Interval::new_closed_closed(10, 30);
        let empty = Interval::<i32>::empty();
        assert_eq!(intv1.symmetric_difference(empty), Pair::One(intv1));
        assert_eq!(empty.symmetric_difference(intv1), Pair::One(intv1));

        let intv2 = Interval::new_closed_closed(1, 50); //  larger
        assert_eq!(
            intv1.symmetric_difference(intv2),
            Pair::Two(
                Interval::new_closed_open(1, 10),
                Interval::new_open_closed(30, 50),
            ),
        );
        assert_eq!(
            intv2.symmetric_difference(intv1),
            Pair::Two(
                Interval::new_closed_open(1, 10),
                Interval::new_open_closed(30, 50),
            )
        );

        let intv3 = Interval::new_closed_closed(1, 5); // disjoint
        assert_eq!(intv1.symmetric_difference(intv3), Pair::Two(intv3, intv1,),);
        assert_eq!(intv3.symmetric_difference(intv1), Pair::Two(intv3, intv1,),);

        let intv4 = Interval::new_closed_closed(1, 15); // overlaps left
        assert_eq!(
            intv1.symmetric_difference(intv4),
            Pair::Two(
                Interval::new_closed_open(1, 10),
                Interval::new_open_closed(15, 30),
            ),
        );

        let intv5 = Interval::new_closed_closed(25, 40); // overlaps right
        assert_eq!(
            intv1.symmetric_difference(intv5),
            Pair::Two(
                Interval::new_closed_open(10, 25),
                Interval::new_open_closed(30, 40),
            ),
        );
    }

    #[test]
    fn test_macro() {
        let intv1 = interval!(1, 2);
        assert!(intv1.equivalent(Interval::new_closed_open(1, 2)));

        let intv1 = interval!(1, 2, "[)");
        assert!(intv1.equivalent(Interval::new_closed_open(1, 2)));

        let intv1 = interval!(1, 2, "[]");
        assert!(intv1.equivalent(Interval::new_closed_closed(1, 2)));

        let intv1 = interval!(1, 2, "(]");
        assert!(intv1.equivalent(Interval::new_open_closed(1, 2)));

        let intv1 = interval!(1, 2, "()");
        assert!(intv1.equivalent(Interval::new_open_open(1, 2)));

        let intv1 = interval!("empty");
        assert!(intv1.equivalent(Interval::<f32>::empty()));

        let intv1 = interval!(1, "[inf");
        assert!(intv1.equivalent(Interval::new_closed_unbounded(1)));

        let intv1 = interval!(1, "inf");
        assert!(intv1.equivalent(Interval::new_closed_unbounded(1)));

        let intv1 = interval!(1, "(inf");
        assert!(intv1.equivalent(Interval::new_open_unbounded(1)));

        let intv1 = interval!("-inf", 1, "]");
        assert!(intv1.equivalent(Interval::new_unbounded_closed(1)));

        let intv1 = interval!("-inf", 1);
        assert!(intv1.equivalent(Interval::new_unbounded_open(1)));

        let intv1 = interval!("-inf", 1, ")");
        assert!(intv1.equivalent(Interval::new_unbounded_open(1)));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_iter() {
        let intv1 = interval!(1_u32, 4, "[)");
        assert_eq!(intv1.iter().collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(intv1.iter().rev().collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(intv1.iter().len(), 3);
        assert_eq!(intv1.iter().rev().len(), 3);
        let mut iter = intv1.iter();
        let _ = iter.next();
        let _ = iter.next_back();
        assert_eq!(iter.as_interval(), interval!(2, 3, "[)"));

        let mut iter = intv1.iter();
        assert_eq!(
            &[iter.next(), iter.next_back(), iter.next(), iter.next_back()],
            &[Some(1), Some(3), Some(2), None],
        );

        let intv1 = interval!(1, 4, "[]");
        assert_eq!(intv1.iter().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
        assert_eq!(intv1.iter().rev().collect::<Vec<_>>(), vec![4, 3, 2, 1]);
        assert_eq!(intv1.iter().len(), 4);
        assert_eq!(intv1.iter().rev().len(), 4);
        assert_eq!(intv1.iter().size_hint(), (4, Some(4)));

        let intv1 = interval!(1, 4, "(]");
        assert_eq!(intv1.iter().collect::<Vec<_>>(), vec![2, 3, 4]);
        assert_eq!(intv1.iter().rev().collect::<Vec<_>>(), vec![4, 3, 2]);
        assert_eq!(intv1.iter().len(), 3);
        assert_eq!(intv1.iter().rev().len(), 3);
        assert_eq!(intv1.iter().size_hint(), (3, Some(3)));
        let mut iter = intv1.iter();
        let _ = iter.next();
        let _ = iter.next_back();
        assert_eq!(iter.as_interval(), interval!(2, 3, "(]"));

        let intv1 = interval!(1_i8, 4, "()");
        assert_eq!(intv1.iter().collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(intv1.iter().rev().collect::<Vec<_>>(), vec![3, 2]);
        assert_eq!(intv1.iter().len(), 2);
        assert_eq!(intv1.iter().rev().len(), 2);
        assert_eq!(intv1.iter().size_hint(), (2, Some(2)));
        let mut iter = intv1.iter();
        let _ = iter.next();
        let _ = iter.next_back();
        assert_eq!(iter.as_interval(), interval!(2, 3, "()"));

        let intv1 = interval!("-inf", 4_u16, "]");
        assert_eq!(intv1.iter().collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);
        assert_eq!(intv1.iter().rev().collect::<Vec<_>>(), vec![4, 3, 2, 1, 0]);
        assert_eq!(intv1.iter().len(), 5);
        assert_eq!(intv1.iter().rev().len(), 5);
        assert_eq!(intv1.iter().size_hint(), (5, Some(5)));
        let mut iter = intv1.iter();
        let _ = iter.next();
        let _ = iter.next_back();
        assert_eq!(iter.as_interval(), interval!(1, 3, "[]"));

        let intv1 = interval!("-inf", 4_u16, ")");
        assert_eq!(intv1.iter().collect::<Vec<_>>(), vec![0, 1, 2, 3]);
        assert_eq!(intv1.iter().rev().collect::<Vec<_>>(), vec![3, 2, 1, 0]);
        assert_eq!(intv1.iter().len(), 4);
        assert_eq!(intv1.iter().rev().len(), 4);
        assert_eq!(intv1.iter().size_hint(), (4, Some(4)));

        let intv1 = interval!(2, "[inf");
        assert_eq!(
            intv1.iter().take(10).collect::<Vec<_>>(),
            vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        );
        assert_eq!(
            intv1.iter().rev().take(3).collect::<Vec<_>>(),
            vec![u32::MAX, u32::MAX - 1, u32::MAX - 2],
        );
        assert_eq!(intv1.iter().take(10).len(), 10);
        assert_eq!(intv1.iter().rev().take(3).len(), 3);
        assert_eq!(
            intv1.iter().size_hint(),
            (u32::MAX as usize - 2, Some(u32::MAX as usize - 2))
        );
        assert_eq!(intv1.iter().take(10).size_hint(), (10, Some(10)));

        let intv1 = interval!(250_u8, "(inf");
        assert_eq!(intv1.iter().size_hint(), (5, Some(5)));
        assert_eq!(
            intv1.iter().collect::<Vec<_>>(),
            vec![251, 252, 253, 254, 255],
        );

        let intv1 = interval!(2, "(inf");
        assert_eq!(
            intv1.iter().take(10).collect::<Vec<_>>(),
            vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
        );
        assert_eq!(
            intv1.iter().rev().take(3).collect::<Vec<_>>(),
            vec![u32::MAX, u32::MAX - 1, u32::MAX - 2],
        );
        assert_eq!(intv1.iter().take(10).len(), 10);
        assert_eq!(intv1.iter().rev().take(10).len(), 10);

        let intv1 = Interval::<u32>::empty();
        assert_eq!(intv1.iter().collect::<Vec<_>>(), Vec::<u32>::new(),);
        assert_eq!(intv1.iter().rev().collect::<Vec<_>>(), Vec::<u32>::new(),);
        assert_eq!(intv1.iter().len(), 0);
        assert_eq!(intv1.iter().rev().len(), 0);
        assert_eq!(intv1.iter().size_hint(), (0, Some(0)));

        let intv1 = Interval::<u32>::doubly_unbounded();
        assert_eq!(
            intv1.iter().take(10).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        );
        assert_eq!(
            intv1.iter().rev().take(3).collect::<Vec<_>>(),
            vec![u32::MAX, u32::MAX - 1, u32::MAX - 2],
        );
        assert_eq!(intv1.iter().take(10).len(), 10);
        assert_eq!(intv1.iter().len(), u32::MAX as usize);
        assert_eq!(intv1.iter().rev().take(3).len(), 3);

        let intv1 = Interval::<u64>::doubly_unbounded();
        assert_eq!(intv1.iter().len(), u64::MAX as usize);

        // Test that skipping is efficient.  We do not need Rust to call
        // next() a billion times.
        let intv1 = Interval::<u32>::doubly_unbounded();
        assert_eq!(
            intv1
                .iter()
                .skip(1_000_000_000)
                .step_by(1_000_000_000)
                .take(3)
                .collect::<Vec<_>>(),
            vec![1_000_000_000, 2_000_000_000, 3_000_000_000],
        );
        assert_eq!(
            intv1
                .iter()
                .rev()
                .skip(1_000_000_000)
                .step_by(1_000_000_000)
                .take(3)
                .collect::<Vec<_>>(),
            vec![
                u32::MAX - 1_000_000_000,
                u32::MAX - 2_000_000_000,
                u32::MAX - 3_000_000_000
            ],
        );
    }

    #[test]
    #[should_panic]
    fn test_len_panic() {
        let intv1 = Interval::<i64>::doubly_unbounded();
        assert_eq!(intv1.iter().len(), usize::MAX);
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod multi {
    use crate::multi::Policy;
    use crate::*;

    fn insert_via_trait<T, E, I>(into: &mut E, from: I)
    where
        E: Extend<Interval<T>>,
        I: IntoIterator<Item = Interval<T>>,
    {
        into.extend(from);
    }

    fn check_bounded<P>(
        mut m: IntervalSet<u32, P>,
        single: IntervalSet<u32, P>,
        expected: &[Interval<u32>],
    ) where
        P: Policy<u32> + ::core::fmt::Debug,
    {
        let empty = IntervalSet::<u32, P>::default();

        insert_via_trait(
            &mut m,
            [
                interval!(1, 3, "[)"),
                interval!(2, 4, "[)"),
                interval!(4, 6, "[)"),
                interval!(8, 10, "[)"),
            ],
        );
        assert_eq!(
            m.iter().collect::<Vec<_>>(),
            expected.iter().collect::<Vec<_>>(),
        );
        assert_eq!(m.len(), expected.len());

        m.check_invariants();

        assert_eq!(m.lower(), Some(&1));
        assert_eq!(m.upper(), Some(&10));
        assert!(!m.lower_unbounded());
        assert!(!m.upper_unbounded());

        assert!(!m.contains(0));
        assert!(m.contains(1));
        assert!(m.contains(2));
        assert!(m.contains(3));
        assert!(m.contains(4));
        assert!(m.contains(5));
        assert!(!m.contains(6));
        assert!(!m.contains(7));
        assert!(m.contains(8));
        assert!(m.contains(9));
        assert!(!m.contains(10));

        assert!(!m.contains_interval(interval!(0, 1)));
        assert!(m.contains_interval(Interval::empty()));
        assert!(!IntervalSet::<u32, P>::empty()
            .contains_interval(interval!(100, 101)));
        assert!(empty.contains_interval(Interval::empty()));
        assert!(m.contains_interval(interval!(1, 2)));
        assert!(m.contains_interval(interval!(1, 5)));
        assert!(!m.contains_interval(interval!(1, 7)));
        assert!(!m.contains_interval(interval!(21, 27)));
        #[allow(clippy::needless_borrows_for_generic_args)]
        {
            assert!(!m.contains_interval(&interval!(6, 7))); //  accepts ref
        }

        assert_eq!(m.convex_hull(), interval!(1, 10, "[)"));

        // Add an interval that covers the whole set.
        m.add(interval!(0, 18));
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(0, 18)]);
        assert_eq!(m.len(), 1);

        // contains set
        {
            m.clear();
            m.extend([
                interval!(1, 3, "[)"),
                interval!(2, 4, "[)"),
                interval!(4, 6, "[)"),
                interval!(8, 10, "[)"),
            ]);

            assert!(m.contains_set(&empty));
            assert!(!empty.contains_set(&m));
            assert!(m.contains_set(&m));

            let mut m2 = m.clone();
            m2.extend([interval!(100, 200)]);
            assert!(!m.contains_set(&m2));

            let mut m2 = m.clone();
            m2 -= interval!(2, 5);
            assert!(m.contains_set(&m2));
        }


        // Same as above, but intervals are not sorted initially
        m.clear();
        m.extend([
            interval!(4, 6, "[)"),
            interval!(8, 10, "[)"),
            interval!(2, 4, "[)"),
            interval!(1, 3, "[)"),
        ]);
        assert_eq!(
            m.iter().collect::<Vec<_>>(),
            expected.iter().collect::<Vec<_>>(),
        );
        assert_eq!(m.len(), expected.len());

        // Additional tests
        m.clear();
        m.add(interval!(1, 4));
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 4)],);
        m.check_invariants();

        m.add(interval!(1, 4)); //  Adding same interval has no effect
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 4)],);

        m.add(interval!(1, 6)); // extends the first interval
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 6)],);

        m.add(interval!(8, 10)); // disjoint
        assert_eq!(m.len(), 2);
        assert_eq!(
            m.iter().collect::<Vec<_>>(),
            vec![&interval!(1, 6), &interval!(8, 10)],
        );

        m.add(interval!(9, 10)); // subset of second interval
        assert_eq!(
            m.iter().collect::<Vec<_>>(),
            vec![&interval!(1, 6), &interval!(8, 10)],
        );

        m.add(interval!(4, 8, "[]")); // joins the two
        assert_eq!(m.len(), 1);
        assert_eq!(m.iter().collect::<Vec<_>>(), vec![&interval!(1, 10)],);
        m.check_invariants();

        // Inserting intervals only after the end of all existing ones
        m.clear();
        m.extend([interval!(1, 3), interval!(4, 5)]);
        m.extend([interval!(6, 7), interval!(8, 9)]);
        assert_eq!(m.len(), 4);
        m.check_invariants();

        // Single intervals
        {
            assert_eq!(single.len(), 1);
            assert!(single.contains(4));
            assert!(!single.contains(3));
            assert!(!single.contains(5));
        }

        // Empty intervals
        {
            let mut m = IntervalSet::<u32, P>::default();
            assert!(m.is_empty());
            assert_eq!(m.len(), 0);
            assert_eq!(m, empty);
            assert!(!m.lower_unbounded());
            assert!(!m.upper_unbounded());
            assert_eq!(m.lower(), None);
            assert_eq!(m.upper(), None);
            assert_eq!(m.convex_hull(), Interval::empty());

            m.add(Interval::empty());
            m.add(Interval::empty());
            assert!(m.is_empty());
        }

        // Unbounded intervals
        {
            let m = IntervalSet::<u32, P>::new([interval!(1, "inf")]);
            assert!(!m.lower_unbounded());
            assert!(m.upper_unbounded());
            assert_eq!(m.lower(), Some(&1));
            assert_eq!(m.upper(), None);

            let m = IntervalSet::<u32, P>::new([interval!("-inf", 1, "]")]);
            assert!(m.lower_unbounded());
            assert!(!m.upper_unbounded());
            assert_eq!(m.lower(), None);
            assert_eq!(m.upper(), Some(&1));
        }

        // Remove
        {
            m.clear();
            m.extend([interval!(5, 10), interval!(15, 20)]);

            assert_eq!(
                m.remove(6).iter().collect::<Vec<_>>(),
                vec![
                    &interval!(5, 6, "[)"),
                    &interval!(6, 10, "()"),
                    &interval!(15, 20)
                ],
            );
            assert_eq!(m.remove(0), m);
            assert_eq!(m.remove(10), m);
            assert_eq!(m.remove(1000), m);
            assert_eq!(
                m.remove_interval(interval!(8, 17))
                    .iter()
                    .collect::<Vec<_>>(),
                vec![&interval!(5, 8, "[)"), &interval!(17, 20)],
            );
            assert_eq!(m.remove_interval(Interval::empty()), m,);

            assert_eq!(
                m.remove_interval(interval!(0, 100)),
                IntervalSet::empty(),
            );
        }

        // Intersects
        {
            m.clear();
            m.extend([interval!(5, 10), interval!(15, 20)]);
            assert!(m.intersects_interval(interval!(9, 10)));
            assert!(m.intersects_interval(interval!(9, 16)));
            assert!(!m.intersects_interval(interval!(10, 12)));

            let mut m2 = m.clone();
            assert!(m.intersects_set(&m2));

            m2.clear();
            assert!(!m.intersects_set(&m2));

            m2.extend([interval!(1, 3), interval!(20, 30)]);
            assert!(!m.intersects_set(&m2));

            m2.extend([interval!(19, 21)]);
            assert!(m.intersects_set(m2));
        }

        // left_of, right_of, ...
        {
            m.clear();
            let mut m2 = m.clone();

            m.extend([interval!(5, 10), interval!(15, 20)]);
            m2.extend([interval!(25, 30), interval!(40, 50)]);
            assert!(m.left_of_set(&m2));
            assert!(!m2.left_of_set(&m));
            assert!(!m.right_of_set(&m2));
            assert!(m2.right_of_set(&m));

            assert!(m.left_of_interval(interval!(20, 30)));
            assert!(m.left_of_interval(interval!(19, 30)));
            assert!(!m.left_of_interval(interval!(18, 30)));
            assert!(m.strictly_left_of_interval(interval!(20, 30)));
            assert!(!m.strictly_left_of_interval(interval!(19, 30)));
            assert!(!m.strictly_left_of_interval(interval!(18, 30)));

            assert!(m.right_of_interval(interval!(1, 5)));
            assert!(m.right_of_interval(interval!(1, 6)));

            assert!(m.left_of(20));
            assert!(m.strictly_left_of(20));
            assert!(m.left_of(19));
            assert!(!m.strictly_left_of(19));
            assert!(!m.left_of(18));

            assert!(m.right_of(5));
            assert!(!m.right_of(6));
            assert!(!m.strictly_right_of(5));
            assert!(m.strictly_right_of(4));
        }
    }

    #[test]
    fn test_joining() {
        check_bounded(
            IntervalSet::empty_joining(),
            IntervalSet::new_single_joining(4),
            &[interval!(1, 6), interval!(8, 10)],
        );

        assert_eq!(IntervalSet::new_joining([interval!(5, 6)]).len(), 1);
    }

    #[test]
    fn test_separating() {
        check_bounded(
            IntervalSet::empty_separating(),
            IntervalSet::new_single_separating(4),
            &[interval!(1, 4), interval!(4, 6), interval!(8, 10)],
        );
        assert_eq!(IntervalSet::new_separating([interval!(5, 6)]).len(), 1);
    }

    #[test]
    fn test_equals() {
        let m1 = IntervalSet::new_joining([
            interval!(3, 10, "[]"),
            interval!(15, 20, "()"),
        ]);
        let m2 = IntervalSet::new([
            interval!(2, 5, "()"),
            interval!(5, 11, "[)"),
            interval!(16, 19, "[]"),
        ]);
        assert_eq!(m1, m2);
        assert_eq!(m2, m1);
        assert!(m1.equivalent(&m2));
        assert!(m1.equivalent(m2));

        let m4 =
            IntervalSet::new([interval!(2, 5, "()"), interval!(5, 11, "[)")]);
        assert_ne!(m1, m4); // same length, different intervals
        assert_ne!(m4, m1); // same length, different intervals

        let m5 = IntervalSet::new([interval!(2, 5, "()")]);
        assert_ne!(m1, m5); // different lengths
        assert_ne!(m5, m1); // different lengths

        let m6 = IntervalSet::new_joining([
            interval!(3, 10, "[]"),
            interval!(15, 20, "()"),
            interval!(25, 30, "()"),
        ]);
        assert_ne!(m1, m6); // same initial intervals, but have more
        assert_ne!(m6, m1); // same initial intervals, but have more

        let intv1 = interval!(3, 20, "[)");
        let pairs = intv1 - interval!(10, 15, "(]");
        let m3 = IntervalSet::from_pair(pairs);
        assert_eq!(m1, m3);

        let intv1 = interval!(3, 20, "[)");
        let pairs = intv1 - interval!(10, 20, "()"); //  one interval
        let m3 = IntervalSet::from_pair(pairs);
        assert_eq!(
            m3,
            IntervalSet::<u8, Separating>::new([interval!(3, 10, "[]")])
        );
    }

    #[test]
    fn test_intersection() {
        let m1 = IntervalSet::new_joining([
            interval!(3, 10, "[]"),
            interval!(15, 20, "()"),
            interval!(25, 40, "[)"),
        ]);
        assert_eq!(m1.intersection_interval(interval!(1, 50)), m1);
        assert_eq!(
            m1.intersection_interval(interval!(4, 35)),
            IntervalSet::new_joining([
                interval!(4, 10, "[]"),
                interval!(15, 20, "()"),
                interval!(25, 35, "[)"),
            ]),
        );
        assert_eq!(
            m1.intersection_interval(interval!(16, 17)),
            IntervalSet::new_joining([interval!(16, 17, "[)"),]),
        );
        assert_eq!(
            m1.intersection_interval(Interval::empty()),
            IntervalSet::empty(),
        );
        assert_eq!(
            IntervalSet::<u32>::empty().intersection_interval(interval!(4, 5)),
            IntervalSet::empty(),
        );
        assert_eq!(
            m1.intersection_set(IntervalSet::new_separating([
                interval!(4, 8),
                interval!(16, 27),
                interval!(38, 50),
            ])),
            IntervalSet::new_joining([
                interval!(4, 8),
                interval!(16, 20),
                interval!(25, 27),
                interval!(38, 40),
            ]),
        );
    }
}
