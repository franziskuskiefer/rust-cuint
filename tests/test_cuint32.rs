#[macro_use]
extern crate cuint;
extern crate rand;

use rand::{thread_rng, Rng};
use std::str::FromStr;
// Trait with all the functions.
use cuint::base::{Uint, UintTrait};

#[allow(dead_code)]
fn random_hex_string(len: usize) -> String {
    const HEX_CHARS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let mut res = "".to_string();
    for _ in 0..len {
        res.push(HEX_CHARS[thread_rng().gen_range(0, HEX_CHARS.len())]);
    }
    res = res.trim_start_matches('0').to_string();
    let mut start = String::from("0x");
    start.push_str(&res);
    start
}

#[allow(dead_code)]
fn get_expected(op: &'static str, a: &String, b: &String) -> String {
    let expected = std::process::Command::new("python")
        .args(&["test_helper.py", op, &a, &b])
        .output()
        .expect("failed to execute python test helper");
    let expected = String::from_utf8_lossy(&expected.stdout)
        .replace("\n", "")
        .replace("\r", "");
    // Python2 appends an L.
    expected.replace("L", "")
}

#[test]
fn test_encode_decode() {
    fn enc_dec(s: &str) {
        let x = Uint::<u32>::from_str(s).unwrap();
        let s_dec = x.to_str();
        println!("{:?} := {:x?} => {:?}", s, x, s_dec);
        assert_eq!(s, s_dec);
    }
    let s = "0x123456789abcdef";
    enc_dec(s);
    for i in 0..100 {
        enc_dec(&random_hex_string(128 + 3 * i));
    }
}

#[test]
fn test_add() {
    fn test_add_core(a: &String, b: &String) {
        let x = Uint::<u32>::from_str(&a).unwrap();
        let y = Uint::<u32>::from_str(&b).unwrap();
        let c = &x + &y;
        let expected = get_expected("add", a, b);
        println!("{:?} + {:?} = {:?}", a, b, expected);
        println!("my result: {:?}", c.to_str());
        assert_eq!(expected, c.to_str());
    }

    // Full limb
    let a = "0xffffffff".to_string();
    let b = "0xffffffff".to_string();
    test_add_core(&a, &b);

    // Two full limbs
    let a = "0xffffffffffffffff".to_string();
    let b = "0xffffffffffffffff".to_string();
    test_add_core(&a, &b);

    // Two different limbs
    let a = "0xffffffffffffffff".to_string();
    let b = "0x1ffffffff".to_string();
    test_add_core(&a, &b);

    // Single limb
    let a = random_hex_string(8);
    let b = random_hex_string(8);
    test_add_core(&a, &b);

    // Two  limbs
    let a = random_hex_string(16);
    let b = random_hex_string(16);
    test_add_core(&a, &b);

    // Different lengths
    let a = random_hex_string(8);
    let b = random_hex_string(16);
    test_add_core(&a, &b);
    let a = random_hex_string(16);
    let b = random_hex_string(8);
    test_add_core(&a, &b);

    // Longer
    for i in 5..50 {
        let a = random_hex_string(i);
        let b = random_hex_string(i);
        test_add_core(&a, &b);
    }
}

#[test]
fn test_mul() {
    // TODO: move out and re-use.
    fn test_mul_core(a: &String, b: &String) {
        let x = Uint::<u32>::from_str(&a).unwrap();
        let y = Uint::<u32>::from_str(&b).unwrap();
        let c = &x * &y;
        let expected = get_expected("mul", a, b);
        println!("{:?} * {:?} = {:?}", a, b, expected);
        println!("my result: {:?}", c.to_str());
        assert_eq!(expected, c.to_str());
    }

    // Full limb
    let a = "0xffffffff".to_string();
    let b = "0xffffffff".to_string();
    test_mul_core(&a, &b);

    // Two full limbs
    let a = "0xffffffffffffffff".to_string();
    let b = "0xffffffffffffffff".to_string();
    test_mul_core(&a, &b);

    // Two different length limbs
    let a = "0xffffffffffffffff".to_string();
    let b = "0x1ffffffff".to_string();
    test_mul_core(&a, &b);

    // Special test
    let a = "0x7ab8264d38136f083a486008a7a5f8b1".to_string();
    let b = "0xf22fb9d176c381f93aa9e84938bb29dff".to_string();
    test_mul_core(&a, &b);

    // Single limb
    let a = random_hex_string(8);
    let b = random_hex_string(8);
    test_mul_core(&a, &b);

    // Two  limbs
    let a = random_hex_string(16);
    let b = random_hex_string(16);
    test_mul_core(&a, &b);

    // Different lengths
    let a = random_hex_string(8);
    let b = random_hex_string(16);
    test_mul_core(&a, &b);
    let a = random_hex_string(16);
    let b = random_hex_string(8);
    test_mul_core(&a, &b);

    // Longer
    for i in 5..50 {
        let a = random_hex_string(i);
        let b = random_hex_string(i);
        test_mul_core(&a, &b);
    }
}

#[test]
fn test_cuint32() {
    create_cuint!(CUint32_256, 256, u32);
    let cuint32_256 = CUint32_256::default();
}
