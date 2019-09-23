//!
//! This is a constant time unsigned integer implementation using 32-bit
//! unsigned integers.
//! It is **work in progress** starting out with some rather slow and generic
//! implementation.
//!
//! This unit is called cuint32, so it uses a 32-bit integers in little-endian
//! representations.
//! So `0x123456789abcdef` would be stored as `[0x89abcdef, 0x1234567]`.
//!

use std::cmp::min;
use std::ops::Add;
use std::str::FromStr;

use cuint::util::*;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CUint32 {
    digits: Vec<u32>,
}

#[derive(Debug)]
pub enum CUintError {
    StringParsingError,
}

use cuint::cuint32::CUintError::StringParsingError;

impl FromStr for CUint32 {
    type Err = CUintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = CUint32::new();
        match r.encode(s) {
            Ok(_) => Ok(r),
            Err(err) => Err(err),
        }
    }
}

// Implement basic functionality for CUint32.
// TODO: All this should be abstracted out to CUint.
impl CUint32 {
    // TODO: return error?
    pub fn to_str(&self) -> String {
        match self.decode() {
            Ok(s) => s,
            Err(_) => "".to_string(),
        }
    }

    pub fn new() -> Self {
        Default::default()
    }

    fn clear(&mut self) {
        self.digits.clear();
    }

    // Read a hex string into a CUint32.
    // The string MUST be of the form "0xdeadbeef".
    // TODO: not only hex?
    fn encode(&mut self, s: &str) -> Result<&CUint32, CUintError> {
        if !s.starts_with("0x") {
            return Err(StringParsingError);
        }

        let mut x = &s[2..];
        let mut len = x.len();
        self.clear();

        // TODO: error prone, need to update len correctly
        while len > 0 {
            let cut = len - min(len, 8);
            let num = match u32::from_str_radix(&x[cut..], 16) {
                Ok(n) => n,
                Err(_) => return Err(StringParsingError),
            };
            self.digits.push(num);

            // Drop the part of the string we parsed.
            x = &x[..cut];
            len = x.len();
        }

        Ok(self)
    }

    // Get a CUint32 as a hex string of the form `0xdeadbeef`.
    pub fn decode(&self) -> Result<String, CUintError> {
        let mut res: String = String::from("");
        for i in (0..self.digits.len()).rev() {
            let d = &self.digits[i];
            res.push_str(&format!("{:08x}", d));
        }
        // remove leading 0s
        // TODO: not cool...
        res = res.trim_start_matches('0').to_string();
        let mut start = String::from("0x");
        start.push_str(&res);
        Ok(start)
    }

    pub fn add_cuint32(&self, other: &CUint32) -> CUint32 {
        let res = add_generic(&self.digits, &other.digits);
        Self { digits: res }
    }
}

// ===================== Implement + ===========================

impl<'a> Add<&'a CUint32> for CUint32 {
    type Output = CUint32;

    #[inline]
    fn add(self, other: &CUint32) -> CUint32 {
        self.add_cuint32(other)
    }
}

impl<'a, 'b> Add<&'b CUint32> for &'a CUint32 {
    type Output = CUint32;

    #[inline]
    fn add(self, other: &CUint32) -> CUint32 {
        self.add_cuint32(other)
    }
}

// ===================== ALGORITHMS ===========================

fn add_generic(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut res = Vec::<u32>::new();
    let mut carry = 0u32;
    let one = 1u32;
    let shorter_len = min(a.len(), b.len());

    // Iterate over min(a.len(), b.len()) elements
    for (ai, bi) in a.iter().zip(b.iter()) {
        let tmp = u32::add_with_carry(ai, bi);
        let c = u32::cadd(&tmp.0, &one, carry);
        carry = tmp.1;
        res.push(c.0);
    }

    // Sum up the carry and remaining values from the longer number.
    let longer = if a.len() > b.len() { a } else { b };
    for d in longer.iter().skip(shorter_len) {
        let c = u32::cadd(d, &one, carry);
        carry = c.1;
        res.push(c.0);
    }
    res.push(carry);
    res
}

// ===================== TESTING ==============================

extern crate rand;
#[allow(dead_code)]
fn random_hex_string(len: usize) -> String {
    use self::rand::{thread_rng, Rng};
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

#[test]
fn test_encode_decode() {
    fn enc_dec(s: &str) {
        let x = CUint32::from_str(s).unwrap();
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
    use std::process::Command;

    fn test_add_core(a: &String, b: &String) {
        let x = CUint32::from_str(&a).unwrap();
        let y = CUint32::from_str(&b).unwrap();
        let c = &x + &y;
        let expected = Command::new("python")
            .args(&["test_helper.py", &a, &b])
            .output()
            .expect("failed to execute python test helper");
        let expected = String::from_utf8_lossy(&expected.stdout)
            .replace("\n", "")
            .replace("\r", "");
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
