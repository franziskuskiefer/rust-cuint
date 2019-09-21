//!
//! A bunch of utilities for constant time math.
//!
//! TODO: rotate, cadd, cmul, cswap etc.

// Helper functions.

// Expand 1 -> 0xFF.. (T::MAX)
#[inline]
fn expand(a: u32) -> u32 {
    !a + 1
}

// The main trait, implemented for u16, u32, u64, and u128
// TODO: u16, u128
pub(crate) trait Compare<T> {
    // Return 1 if a == b, 0 otherwise.
    #[inline]
    fn equal(a: &T, b: &T) -> T;
    // Return 1 if a >= b, 0 otherwise.
    #[inline]
    fn gte(a: &T, b: &T) -> T;
    // TODO: change to c == 1
    // Return a + b if c == T::MAX, a otherwise.
    // a + b MUST not overflow T.
    #[inline]
    fn cadd(a: &T, b: &T, c: &T) -> T;
}

impl Compare<u64> for u64 {
    fn equal(a: &u64, b: &u64) -> u64 {
        let mut x = !(a ^ b);
        x = x & (x << 32);
        x = x & (x << 16);
        x = x & (x << 8);
        x = x & (x << 4);
        x = x & (x << 2);
        x = x & (x << 1);
        return x >> 63;
    }
    fn gte(a: &u64, b: &u64) -> u64 {
        (!((((*a as i128) - (*b as i128)) >> 63) as u64)) >> 63
    }
    fn cadd(a: &u64, b: &u64, c: &u64) -> u64 {
        a + (b & c)
    }
}

impl Compare<u32> for u32 {
    fn equal(a: &u32, b: &u32) -> u32 {
        let mut x = !(a ^ b);
        x = x & (x << 16);
        x = x & (x << 8);
        x = x & (x << 4);
        x = x & (x << 2);
        x = x & (x << 1);
        return x >> 31;
    }
    fn gte(a: &u32, b: &u32) -> u32 {
        (!((((*a as i64) - (*b as i64)) >> 63) as u32)) >> 31
    }
    fn cadd(a: &u32, b: &u32, c: &u32) -> u32 {
        a + (b & c)
    }
}

// ===================== TESTS ========================
#[test]
fn test_equal() {
    fn test_inner<T>(a: T, b: T, expected: T)
    where
        T: PartialEq + std::fmt::Debug + Compare<T>,
    {
        let x = T::equal(&a, &b);
        println!("{:?} == {:?}: {:?}", a, b, x);
        assert_eq!(expected, x);
    }
    test_inner(0u32, 0u32, 1u32);
    test_inner(123u32, 123u32, 1u32);
    test_inner(123u32, 124u32, 0u32);
    test_inner(124u32, 123u32, 0u32);
    test_inner(0xFFFFFFFFu32, 0xFFFFFFFFu32, 1u32);
    test_inner(0xFFFFFFFFu32, 0xFFFFFFFEu32, 0u32);
    test_inner(0xEFFFFFFFu32, 0xFFFFFFFFu32, 0u32);

    test_inner(0u64, 0u64, 1u64);
    test_inner(123u64, 123u64, 1u64);
    test_inner(123u64, 124u64, 0u64);
    test_inner(124u64, 123u64, 0u64);
    test_inner(0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64, 1u64);
    test_inner(0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFEu64, 0u64);
    test_inner(0xEFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64, 0u64);
}

#[test]
fn test_gte() {
    fn test_inner<T>(a: T, b: T, expected: T)
    where
        T: PartialEq + std::fmt::Debug + Compare<T>,
    {
        let x = T::gte(&a, &b);
        println!("{:?} >= {:?}: {:?}", a, b, x);
        assert_eq!(expected, x);
    }
    test_inner(0u32, 0u32, 1u32);
    test_inner(123u32, 123u32, 1u32);
    test_inner(123u32, 124u32, 0u32);
    test_inner(124u32, 123u32, 1u32);
    test_inner(0xFFFFFFFFu32, 0xFFFFFFFFu32, 1u32);
    test_inner(0xFFFFFFFFu32, 0xFFFFFFFEu32, 1u32);
    test_inner(0xEFFFFFFFu32, 0xFFFFFFFFu32, 0u32);

    test_inner(0u64, 0u64, 1u64);
    test_inner(123u64, 123u64, 1u64);
    test_inner(123u64, 124u64, 0u64);
    test_inner(124u64, 123u64, 1u64);
    test_inner(0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64, 1u64);
    test_inner(0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFEu64, 1u64);
    test_inner(0xEFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64, 0u64);
}

#[test]
fn test_cadd() {
    fn test_inner<T>(a: T, b: T, c: T, expected: T)
    where
        T: PartialEq + std::fmt::Debug + Compare<T>,
    {
        let x = T::cadd(&a, &b, &c);
        println!("{:?} + {:?} => {:?}", a, b, x);
        assert_eq!(expected, x);
    }
    test_inner(456u32, 123u32, 0u32, 456u32);
    test_inner(456u32, 123u32, 0xFFFFFFFFu32, 579u32);
    test_inner(456u64, 123u64, 0u64, 456u64);
    test_inner(456u64, 123u64, 0xFFFFFFFFFFFFFFFFu64, 579u64);
}
