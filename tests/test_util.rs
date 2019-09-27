extern crate cuint;
extern crate time;

use time::PreciseTime;
use cuint::util::CTimeOperations;

#[test]
fn test_equal() {
    fn test_inner<T>(a: T, b: T, expected: T)
    where
        T: PartialEq + std::fmt::Debug + CTimeOperations<T>,
    {
        let x = T::equal(&a, &b);
        println!("{:?} == {:?}: {:?}", a, b, x);
        assert_eq!(expected, x);
    }
    test_inner(0u16, 0u16, 1u16);
    test_inner(123u16, 123u16, 1u16);
    test_inner(123u16, 124u16, 0u16);
    test_inner(124u16, 123u16, 0u16);
    test_inner(0xFFFFu16, 0xFFFFu16, 1u16);
    test_inner(0xFFFFu16, 0xFFFEu16, 0u16);
    test_inner(0xEFFFu16, 0xFFFFu16, 0u16);

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
        T: PartialEq + std::fmt::Debug + CTimeOperations<T>,
    {
        let x = T::gte(&a, &b);
        println!("{:?} >= {:?}: {:?}", a, b, x);
        assert_eq!(expected, x);
    }
    test_inner(0u16, 0u16, 1u16);
    test_inner(123u16, 123u16, 1u16);
    test_inner(123u16, 124u16, 0u16);
    test_inner(124u16, 123u16, 1u16);
    test_inner(0xFFFFu16, 0xFFFFu16, 1u16);
    test_inner(0xFFFFu16, 0xFFFEu16, 1u16);
    test_inner(0xEFFFu16, 0xFFFFu16, 0u16);

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
fn test_add() {
    fn test_inner<T>(a: T, b: T, c: T, expected_c: (T, T), expected: (T, T))
    where
        T: PartialEq + std::fmt::Debug + CTimeOperations<T>,
    {
        let x = T::cadd(&a, &b, &c);
        let (r1, r2) = T::add_with_carry(&a, &b);
        println!("{:x?} + {:x?} => {:x?}", a, b, x);
        println!("{:x?} + {:x?} => {:x?} | {:x?}", a, b, r2, r1);
        assert_eq!(expected_c, x);
        assert_eq!(expected, (r1, r2));
    }
    test_inner(0u16, 0u16, 0u16, (0u16, 0u16), (0u16, 0u16));
    test_inner(0u16, 0u16, 1u16, (0u16, 0u16), (0u16, 0u16));
    test_inner(456u16, 123u16, 0u16, (456u16, 0u16), (579u16, 0u16));
    test_inner(456u16, 123u16, 1u16, (579u16, 0u16), (579u16, 0u16));
    test_inner(
        0xFFFFu16,
        0xFFFFu16,
        0u16,
        (0xFFFFu16, 0u16),
        (0xFFFEu16, 1u16),
    );
    test_inner(
        0xFFFFu16,
        0xFFFFu16,
        1u16,
        (0xFFFEu16, 1u16),
        (0xFFFEu16, 1u16),
    );

    test_inner(0u32, 0u32, 0u32, (0u32, 0u32), (0u32, 0u32));
    test_inner(0u32, 0u32, 1u32, (0u32, 0u32), (0u32, 0u32));
    test_inner(456u32, 123u32, 0u32, (456u32, 0u32), (579u32, 0u32));
    test_inner(456u32, 123u32, 1u32, (579u32, 0u32), (579u32, 0u32));
    test_inner(
        0xFFFFFFFFu32,
        0xFFFFFFFFu32,
        0u32,
        (0xFFFFFFFFu32, 0u32),
        (0xFFFFFFFEu32, 1u32),
    );
    test_inner(
        0xFFFFFFFFu32,
        0xFFFFFFFFu32,
        1u32,
        (0xFFFFFFFEu32, 1u32),
        (0xFFFFFFFEu32, 1u32),
    );

    test_inner(0u64, 0u64, 0u64, (0u64, 0u64), (0u64, 0u64));
    test_inner(0u64, 0u64, 1u64, (0u64, 0u64), (0u64, 0u64));
    test_inner(456u64, 123u64, 0u64, (456u64, 0u64), (579u64, 0u64));
    test_inner(456u64, 123u64, 1u64, (579u64, 0u64), (579u64, 0u64));
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0u64,
        (0xFFFFFFFFFFFFFFFFu64, 0u64),
        (0xFFFFFFFFFFFFFFFEu64, 1u64),
    );
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        1u64,
        (0xFFFFFFFFFFFFFFFEu64, 1u64),
        (0xFFFFFFFFFFFFFFFEu64, 1u64),
    );

    test_inner(
        0xd77f530au32,
        0xa67ba0d3u32,
        1u32,
        (0x7dfaf3ddu32, 1u32),
        (0x7dfaf3ddu32, 1u32),
    );
}

#[test]
fn test_cmul() {
    fn test_inner<T>(a: T, b: T, c: T, expected_c: (T, T), expected: (T, T))
    where
        T: PartialEq + std::fmt::Debug + CTimeOperations<T>,
    {
        let x = T::cmul(&a, &b, &c);
        let (r1, r2) = T::mul_with_carry(&a, &b);
        println!("{:?} * {:?} => {:?}", a, b, x);
        println!("{:?} * {:?} => {:?} | {:?}", a, b, r2, r1);
        assert_eq!(expected_c, x);
        assert_eq!(expected, (r1, r2));
    }
    test_inner(0u16, 0u16, 0u16, (0u16, 0u16), (0u16, 0u16));
    test_inner(0u16, 0u16, 1u16, (0u16, 0u16), (0u16, 0u16));
    test_inner(456u16, 123u16, 0u16, (456u16, 0u16), (0xdb18u16, 0u16));
    test_inner(456u16, 123u16, 1u16, (0xdb18u16, 0u16), (0xdb18u16, 0u16));
    test_inner(
        0xFFFFu16,
        0xFFFFu16,
        0u16,
        (0xFFFFu16, 0u16),
        (0x0001u16, 0xfffeu16),
    );
    test_inner(
        0xFFFFu16,
        0xFFFFu16,
        1u16,
        (0x0001u16, 0xfffeu16),
        (0x0001u16, 0xfffeu16),
    );

    test_inner(0u32, 0u32, 0u32, (0u32, 0u32), (0u32, 0u32));
    test_inner(0u32, 0u32, 1u32, (0u32, 0u32), (0u32, 0u32));
    test_inner(456u32, 123u32, 0u32, (456u32, 0u32), (0xdb18u32, 0u32));
    test_inner(456u32, 123u32, 1u32, (0xdb18u32, 0u32), (0xdb18u32, 0u32));
    test_inner(
        0xFFFFFFFFu32,
        0xFFFFFFFFu32,
        0u32,
        (0xFFFFFFFFu32, 0u32),
        (0x00000001u32, 0xfffffffeu32),
    );
    test_inner(
        0xFFFFFFFFu32,
        0xFFFFFFFFu32,
        1u32,
        (0x00000001u32, 0xfffffffeu32),
        (0x00000001u32, 0xfffffffeu32),
    );

    test_inner(0u64, 0u64, 0u64, (0u64, 0u64), (0u64, 0u64));
    test_inner(0u64, 0u64, 1u64, (0u64, 0u64), (0u64, 0u64));
    test_inner(456u64, 123u64, 0u64, (456u64, 0u64), (0xdb18u64, 0u64));
    test_inner(456u64, 123u64, 1u64, (0xdb18u64, 0u64), (0xdb18u64, 0u64));
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0u64,
        (0xFFFFFFFFFFFFFFFFu64, 0u64),
        (0x0000000000000001u64, 0xfffffffffffffffeu64),
    );
    test_inner(
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        1u64,
        (0x0000000000000001u64, 0xfffffffffffffffeu64),
        (0x0000000000000001u64, 0xfffffffffffffffeu64),
    );
}

#[test]
fn test_cswap() {
    fn test_inner<T>(a: T, b: T, c: T, expected: (T, T))
    where
        T: PartialEq + std::fmt::Debug + CTimeOperations<T>,
    {
        let x = T::cswap(&a, &b, &c);
        println!("{:?} + {:?} => {:?}", a, b, x);
        assert_eq!(expected, x);
    }
    test_inner(0u16, 0u16, 0u16, (0u16, 0u16));
    test_inner(0u16, 0u16, 1u16, (0u16, 0u16));
    test_inner(456u16, 123u16, 0u16, (456u16, 123u16));
    test_inner(456u16, 123u16, 1u16, (123u16, 456u16));
    test_inner(0xFFFFu16, 0xFFFFu16, 0u16, (0xFFFFu16, 0xFFFFu16));
    test_inner(0xFFFFu16, 0xFFFFu16, 1u16, (0xFFFFu16, 0xFFFFu16));
    test_inner(0xFFFFu16, 0x1234u16, 1u16, (0x1234u16, 0xFFFFu16));

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

#[test]
fn test_overflowing_add_timings() {
    fn overflowing_add_time_inner(x: u32, y: u32) -> i64 {

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
