//!
//! A bunch of utilities for constant time math on Rust unsigned integers
//! u16, u32, u64, and u128.
//!
//! Instead of booleans, 0 and 1 are used for false and true.
//!
//! TODO: rotate etc.

// TODO: u128
// TODO: pass values (no &)?
/// The main trait, implemented for u16, u32, u64, and u128
pub trait CTimeOperations<T> {
    /// Return 1 if a == b, 0 otherwise.
    fn equal(a: &T, b: &T) -> T;

    /// Return 1 if a >= b, 0 otherwise.
    fn gte(a: &T, b: &T) -> T;

    /// Return (a + b, carry) if c == 1, (a, 0) if c == 0, and rubbish otherwise.
    /// a + b MUST not overflow T.
    fn cadd(a: &T, b: &T, c: &T) -> (T, T);

    /// Return (a + b, carry).
    fn add_with_carry(a: &T, b: &T) -> (T, T);

    /// Return (a * b lower 64 bits, a * b higher 64 bits) if c == 1, (a, 0) if c == 0, and rubbish otherwise.
    fn cmul(a: &T, b: &T, c: &T) -> (T, T);

    /// Return (a * b lower 64 bits, a * b higher 64 bits).
    fn mul_with_carry(a: &T, b: &T) -> (T, T);

    /// Return (b, a) if c == 1; (a, b) otherwise.
    fn cswap(a: &T, b: &T, c: &T) -> (T, T);
}

impl CTimeOperations<u64> for u64 {
    #[inline]
    fn equal(a: &u64, b: &u64) -> u64 {
        let mut x = !(a ^ b);
        x = x & (x << 32);
        x = x & (x << 16);
        x = x & (x << 8);
        x = x & (x << 4);
        x = x & (x << 2);
        x = x & (x << 1);
        x >> 63
    }

    #[inline]
    fn gte(a: &u64, b: &u64) -> u64 {
        (!(((i128::from(*a) - i128::from(*b)) >> 63) as u64)) >> 63
    }

    #[inline]
    fn cadd(a: &u64, b: &u64, c: &u64) -> (u64, u64) {
        let c = (!c).overflowing_add(1).0;
        let r = a.overflowing_add(b & c);
        (r.0, r.1 as u64)
    }

    #[inline]
    fn add_with_carry(a: &u64, b: &u64) -> (u64, u64) {
        let r = a.overflowing_add(*b);
        (r.0, r.1 as u64)
    }

    #[inline]
    fn cmul(a: &u64, b: &u64, c: &u64) -> (u64, u64) {
        let mask = (!u128::from(*c)).overflowing_add(1).0;
        let r = (u128::from(*a) * u128::from(*b)) & mask;
        let r = (r & mask) ^ (u128::from(*a) & !mask);
        (r as u64, (r >> 64) as u64)
    }

    #[inline]
    fn mul_with_carry(a: &u64, b: &u64) -> (u64, u64) {
        let r = u128::from(*a) * u128::from(*b);
        (r as u64, (r >> 64) as u64)
    }

    #[inline]
    fn cswap(a: &u64, b: &u64, c: &u64) -> (u64, u64) {
        let c = (!c).overflowing_add(1).0;
        let mask = (a ^ b) & c;
        (a ^ mask, b ^ mask)
    }
}

impl CTimeOperations<u32> for u32 {
    #[inline]
    fn equal(a: &u32, b: &u32) -> u32 {
        let mut x = !(a ^ b);
        x = x & (x << 16);
        x = x & (x << 8);
        x = x & (x << 4);
        x = x & (x << 2);
        x = x & (x << 1);
        x >> 31
    }

    #[inline]
    fn gte(a: &u32, b: &u32) -> u32 {
        (!(((i64::from(*a) - i64::from(*b)) >> 63) as u32)) >> 31
    }

    #[inline]
    fn cadd(a: &u32, b: &u32, c: &u32) -> (u32, u32) {
        let r = a.overflowing_add(b & ((!c).overflowing_add(1).0));
        (r.0, r.1 as u32)
    }

    #[inline]
    fn add_with_carry(a: &u32, b: &u32) -> (u32, u32) {
        let r = a.overflowing_add(*b);
        (r.0, r.1 as u32)
    }

    #[inline]
    fn cmul(a: &u32, b: &u32, c: &u32) -> (u32, u32) {
        let mask = (!u64::from(*c)).overflowing_add(1).0;
        let r = (u64::from(*a) * u64::from(*b)) & mask;
        let r = (r & mask) ^ (u64::from(*a) & !mask);
        (r as u32, (r >> 32) as u32)
    }

    #[inline]
    fn mul_with_carry(a: &u32, b: &u32) -> (u32, u32) {
        let r = u64::from(*a) * u64::from(*b);
        (r as u32, (r >> 32) as u32)
    }

    #[inline]
    fn cswap(a: &u32, b: &u32, c: &u32) -> (u32, u32) {
        let c = (!c).overflowing_add(1).0;
        let mask = (a ^ b) & c;
        (a ^ mask, b ^ mask)
    }
}

impl CTimeOperations<u16> for u16 {
    #[inline]
    fn equal(a: &u16, b: &u16) -> u16 {
        let mut x = !(a ^ b);
        x = x & (x << 8);
        x = x & (x << 4);
        x = x & (x << 2);
        x = x & (x << 1);
        x >> 15
    }

    #[inline]
    fn gte(a: &u16, b: &u16) -> u16 {
        (!(((i32::from(*a) - i32::from(*b)) >> 31) as u16)) >> 15
    }

    #[inline]
    fn cadd(a: &u16, b: &u16, c: &u16) -> (u16, u16) {
        let r = a.overflowing_add(b & ((!c).overflowing_add(1).0));
        (r.0, r.1 as u16)
    }

    #[inline]
    fn add_with_carry(a: &u16, b: &u16) -> (u16, u16) {
        let r = a.overflowing_add(*b);
        (r.0, r.1 as u16)
    }

    #[inline]
    fn cmul(a: &u16, b: &u16, c: &u16) -> (u16, u16) {
        let mask = (!u32::from(*c)).overflowing_add(1).0;
        let r = (u32::from(*a) * u32::from(*b)) & mask;
        let r = (r & mask) ^ (u32::from(*a) & !mask);
        (r as u16, (r >> 16) as u16)
    }

    #[inline]
    fn mul_with_carry(a: &u16, b: &u16) -> (u16, u16) {
        let r = u32::from(*a) * u32::from(*b);
        (r as u16, (r >> 16) as u16)
    }

    #[inline]
    fn cswap(a: &u16, b: &u16, c: &u16) -> (u16, u16) {
        let c = (!c).overflowing_add(1).0;
        let mask = (a ^ b) & c;
        (a ^ mask, b ^ mask)
    }
}
