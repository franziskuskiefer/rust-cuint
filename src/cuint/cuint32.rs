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

use cuint::util::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CUint32 {
    digits: Vec<u32>,
}

pub enum CUintError {
    StringParsingError,
}

use cuint::cuint32::CUintError::StringParsingError;

// Implement basic functionality for CUint32.
// TODO: All this should be abstracted out to CUint.
impl CUint32 {
    // TODO: returns an "empty" number if an error occurs.
    // TODO: return error?
    pub fn from_str(s: &str) -> CUint32 {
        let mut r = CUint32::new();
        if r.encode(s).is_err() {
            return CUint32::new();
        }
        r
    }

    // TODO: return error?
    pub fn to_str(&self) -> String {
        match self.decode() {
            Ok(s) => s,
            Err(_) => "".to_string(),
        }
    }

    pub fn new() -> CUint32 {
        Self { digits: vec![] }
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
            let mut num = match u32::from_str_radix(&x[cut..], 16) {
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

impl<'a> Add<&'a CUint32> for CUint32 {
    type Output = CUint32;

    fn add(self, other: &CUint32) -> CUint32 {
        self.add_cuint32(other)
    }
}

// ===================== ALGORITHMS ===========================

// TODO: make ct
fn add_generic(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    let mut res = Vec::<u32>::new();
    let mut carry = false;
    let shorter_len = min(a.len(), b.len());
    let u32_max = std::u32::MAX as u64;

    // Iterate over min(a.len(), b.len()) elements
    for (ai, bi) in a.iter().zip(b.iter()) {
        let mut c = (*ai as u64) + (*bi as u64);
        if carry {
            c = 1 + c;
            carry = false;
        }
        if c > u32_max {
            carry = true;
        }
        res.push((c & u32_max) as u32);
    }

    // Sum up the carry and remaining values from the longer number.
    let longer = if a.len() > b.len() { a } else { b };
    for d in longer.iter().skip(shorter_len) {
        let mut c = *d as u64;
        if carry {
            c = 1 + (*d as u64);
            if c <= u32_max {
                carry = false;
            }
        }
        res.push((c & u32_max) as u32);
    }
    if carry {
        res.push(0x1u32);
    }
    res
}

// ===================== TESTING ==============================

extern crate rand;
use self::rand::{thread_rng, Rng};
const HEX_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

fn random_hex_string(len: usize) -> String {
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
        let x = CUint32::from_str(s);
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
        let c = CUint32::from_str(&a) + &CUint32::from_str(&b);
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
