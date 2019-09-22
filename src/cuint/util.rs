//!
//! A bunch of utilities for constant time math on Rust unsigned integers
//! u16, u32, u64, and u128.
//!
//! Instead of booleans, 0 and 1 are used for false and true.
//!
//! TODO: rotate etc.

// The main trait, implemented for u16, u32, u64, and u128
// TODO: u16, u128
pub(crate) trait Compare<T> {
    // Return 1 if a == b, 0 otherwise.
    fn equal(a: &T, b: &T) -> T;

    // Return 1 if a >= b, 0 otherwise.
    fn gte(a: &T, b: &T) -> T;

    // Return (a + b, carry) if c == 1, (a, 0) if c == 0, and rubbish otherwise.
    // a + b MUST not overflow T.
    fn cadd(a: &T, b: &T, c: &T) -> (T, T);

    // Return (a * b lower 64 bits, a * b higher 64 bits) if c == 1, (a, 0) if c == 0, and rubbish otherwise.
    fn cmul(a: &T, b: &T, c: &T) -> (T, T);

    // Return (b, a) if c == 1; (a, b) otherwise.
    fn cswap(a: &T, b: &T, c: &T) -> (T, T);
}

impl Compare<u64> for u64 {
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
    fn cmul(a: &u64, b: &u64, c: &u64) -> (u64, u64) {
        let mask = (!u128::from(*c)).overflowing_add(1).0;
        let r = (u128::from(*a) * u128::from(*b)) & mask;
        let r = (r & mask) ^ (u128::from(*a) & !mask);
        (r as u64, (r >> 64) as u64)
    }
    #[inline]
    fn cswap(a: &u64, b: &u64, c: &u64) -> (u64, u64) {
        let c = (!c).overflowing_add(1).0;
        let mask = (a ^ b) & c;
        (a ^ mask, b ^ mask)
    }
}

impl Compare<u32> for u32 {
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
    fn cmul(a: &u32, b: &u32, c: &u32) -> (u32, u32) {
        let mask = (!u64::from(*c)).overflowing_add(1).0;
        let r = (u64::from(*a) * u64::from(*b)) & mask;
        let r = (r & mask) ^ (u64::from(*a) & !mask);
        (r as u32, (r >> 32) as u32)
    }
    #[inline]
    fn cswap(a: &u32, b: &u32, c: &u32) -> (u32, u32) {
        let c = (!c).overflowing_add(1).0;
        let mask = (a ^ b) & c;
        (a ^ mask, b ^ mask)
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
    fn test_inner<T>(a: T, b: T, c: T, expected: (T, T))
    where
        T: PartialEq + std::fmt::Debug + Compare<T>,
    {
        let x = T::cadd(&a, &b, &c);
        println!("{:?} + {:?} => {:?}", a, b, x);
        assert_eq!(expected, x);
    }
    test_inner(0u32, 0u32, 0u32, (0u32, 0u32));
    test_inner(0u32, 0u32, 1u32, (0u32, 0u32));
    test_inner(456u32, 123u32, 0u32, (456u32, 0u32));
    test_inner(456u32, 123u32, 1u32, (579u32, 0u32));
    test_inner(0xFFFFFFFFu32, 0xFFFFFFFFu32, 0u32, (0xFFFFFFFFu32, 0u32));
    test_inner(0xFFFFFFFFu32, 0xFFFFFFFFu32, 1u32, (0xFFFFFFFEu32, 1u32));

    test_inner(0u64, 0u64, 0u64, (0u64, 0u64));
    test_inner(0u64, 0u64, 1u64, (0u64, 0u64));
    test_inner(456u64, 123u64, 0u64, (456u64, 0u64));
    test_inner(456u64, 123u64, 1u64, (579u64, 0u64));
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0u64,
        (0xFFFFFFFFFFFFFFFFu64, 0u64),
    );
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        1u64,
        (0xFFFFFFFFFFFFFFFEu64, 1u64),
    );
}

#[test]
fn test_cmul() {
    fn test_inner<T>(a: T, b: T, c: T, expected: (T, T))
    where
        T: PartialEq + std::fmt::Debug + Compare<T>,
    {
        let x = T::cmul(&a, &b, &c);
        println!("{:?} + {:?} => {:?}", a, b, x);
        assert_eq!(expected, x);
    }
    test_inner(0u32, 0u32, 0u32, (0u32, 0u32));
    test_inner(0u32, 0u32, 1u32, (0u32, 0u32));
    test_inner(456u32, 123u32, 0u32, (456u32, 0u32));
    test_inner(456u32, 123u32, 1u32, (0xdb18u32, 0u32));
    test_inner(0xFFFFFFFFu32, 0xFFFFFFFFu32, 0u32, (0xFFFFFFFFu32, 0u32));
    test_inner(
        0xFFFFFFFFu32,
        0xFFFFFFFFu32,
        1u32,
        (0x00000001u32, 0xfffffffeu32),
    );

    test_inner(0u64, 0u64, 0u64, (0u64, 0u64));
    test_inner(0u64, 0u64, 1u64, (0u64, 0u64));
    test_inner(456u64, 123u64, 0u64, (456u64, 0u64));
    test_inner(456u64, 123u64, 1u64, (0xdb18u64, 0u64));
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0u64,
        (0xFFFFFFFFFFFFFFFFu64, 0u64),
    );
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        1u64,
        (0x0000000000000001u64, 0xfffffffffffffffeu64),
    );
}

#[test]
fn test_cswap() {
    fn test_inner<T>(a: T, b: T, c: T, expected: (T, T))
    where
        T: PartialEq + std::fmt::Debug + Compare<T>,
    {
        let x = T::cswap(&a, &b, &c);
        println!("{:?} + {:?} => {:?}", a, b, x);
        assert_eq!(expected, x);
    }
    test_inner(0u32, 0u32, 0u32, (0u32, 0u32));
    test_inner(0u32, 0u32, 1u32, (0u32, 0u32));
    test_inner(456u32, 123u32, 0u32, (456u32, 123u32));
    test_inner(456u32, 123u32, 1u32, (123u32, 456u32));
    test_inner(
        0xFFFFFFFFu32,
        0xFFFFFFFFu32,
        0u32,
        (0xFFFFFFFFu32, 0xFFFFFFFFu32),
    );
    test_inner(
        0xFFFFFFFFu32,
        0xFFFFFFFFu32,
        1u32,
        (0xFFFFFFFFu32, 0xFFFFFFFFu32),
    );
    test_inner(
        0xFFFFFFFFu32,
        0x12345678u32,
        1u32,
        (0x12345678u32, 0xFFFFFFFFu32),
    );

    test_inner(0u64, 0u64, 0u64, (0u64, 0u64));
    test_inner(0u64, 0u64, 1u64, (0u64, 0u64));
    test_inner(456u64, 123u64, 0u64, (456u64, 123u64));
    test_inner(456u64, 123u64, 1u64, (123u64, 456u64));
    test_inner(
        0xFFFFFFFFu64,
        0xFFFFFFFFu64,
        0u64,
        (0xFFFFFFFFu64, 0xFFFFFFFFu64),
    );
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        1u64,
        (0xFFFFFFFFFFFFFFFFu64, 0xFFFFFFFFFFFFFFFFu64),
    );
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0x12345678deadbeefu64,
        1u64,
        (0x12345678deadbeefu64, 0xFFFFFFFFFFFFFFFFu64),
    );
}

extern crate time;
#[test]
fn test_overflowing_add_timings() {
    fn overflowing_add_time_inner(x: u32, y: u32) -> i64 {
        use self::time::PreciseTime;

        let start = PreciseTime::now();
        let _z = x.overflowing_add(y);
        let end = PreciseTime::now();

        let runtime_nanos = start
            .to(end)
            .num_nanoseconds()
            .expect("Benchmark iter took greater than 2^63 nanoseconds");
        return runtime_nanos;
    }

    let times = 100000;
    for i in vec![0, 1, 0, 1234567, std::u32::MAX] {
        let mut t = 0i64;
        for _ in 0..times {
            t = t + overflowing_add_time_inner(i, i);
        }
        t = t / times;
        println!("{:?}: {:?}", i, t);
    }
}
