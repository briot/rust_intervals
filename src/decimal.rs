use crate::nothing_between::NothingBetween;

impl NothingBetween for rust_decimal::Decimal {
    fn nothing_between(&self, _other: &Self) -> bool {
        false // there is always a decimal between two others
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_decimal() {
        let dec1 = rust_decimal::Decimal::ONE;
        let dec2 = rust_decimal::Decimal::new(101, 2); // 1.01
        assert!(interval!(dec1, dec1).is_empty());
        assert!(!interval!(dec1, dec2, "()").is_empty());
    }
}
