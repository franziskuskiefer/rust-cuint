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
//! TODO: implement efficient algorithms (start with the sorts of Karatsuba and improve from there).
//!

use std::cmp::min;
use std::ops::{Add, Mul};
use std::str::FromStr;

use util::*;

/// The struct holding the u32 vector representing the unsigned integer.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CUint32 {
    digits: Vec<u32>,
}

/// Errors specific to CUint32 operations like encoding, decoding, and arithmetic errors.
#[derive(Debug)]
pub enum CUintError {
    StringParsingError,
}

use cuint32::CUintError::StringParsingError;

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

// TODO: All this should be abstracted out to CUint.
/// Implement basic functionality for CUint32.
impl CUint32 {
    // TODO: return error?
    pub fn to_str(&self) -> String {
        match self.decode() {
            Ok(s) => s,
            Err(_) => "".to_string(),
        }
    }

    /// Get a new, empty CUint32 == 0.
    pub fn new() -> Self {
        Default::default()
    }

    /// Clear a CUint32, i.e. this CUint32 == 0 after this operation.
    fn clear(&mut self) {
        self.digits.clear();
    }

    // TODO: not only hex?
    /// Read a hex string into a CUint32.
    /// The string MUST be of the form "0xdeadbeef".
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

    /// Get a CUint32 as a hex string of the form `0xdeadbeef`.
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

    /// Add two CUint32.
    /// This uses a generic, slow addition algorithm at this time.
    ///
    /// # Example:
    /// ```rust,ignore
    ///     let a = CUint32::from_str("0x123");
    ///     let b = CUint32::from_str("0x456");
    ///     let c = a.add_cuint32(&b);
    /// ```
    fn add_cuint32(&self, other: &CUint32) -> CUint32 {
        let res = add_generic(&self.digits, &other.digits);
        Self { digits: res }
    }

    /// Multiply two CUint32.
    /// This uses a generic, slow multiplication algorithm at this time.
    ///
    /// # Example:
    /// ```rust,ignore
    ///     let a = CUint32::from_str("0x123");
    ///     let b = CUint32::from_str("0x456");
    ///     let c = a.mul_cuint32(&b);
    /// ```
    fn mul_cuint32(&self, other: &CUint32) -> CUint32 {
        let res = mul_generic(&self.digits, &other.digits);
        Self { digits: res }
    }

    // FIXME: implement mod_pow
    pub fn mod_pow(&self, exp: &CUint32, modulus: &CUint32) -> CUint32 {
        unimplemented!();
    }

    // FIXME: implement mul_pow
    pub fn mod_mul(&self, other: &CUint32, modulus: &CUint32) -> CUint32 {
        unimplemented!();
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

// ===================== Implement * ===========================
// let c = &a * &b;
// let c = a * &b;

impl<'a> Mul<&'a CUint32> for CUint32 {
    type Output = CUint32;

    #[inline]
    fn mul(self, other: &CUint32) -> CUint32 {
        self.mul_cuint32(other)
    }
}

impl<'a, 'b> Mul<&'b CUint32> for &'a CUint32 {
    type Output = CUint32;

    #[inline]
    fn mul(self, other: &CUint32) -> CUint32 {
        self.mul_cuint32(other)
    }
}

// ===================== ALGORITHMS ===========================

/// A very generic way of summing up two vectors of u32.
fn add_generic(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut res = Vec::<u32>::new();
    let mut carry = 0u32;

    // Iterate over min(a.len(), b.len()) elements
    for (ai, bi) in a.iter().zip(b.iter()) {
        let tmp = u32::add_with_carry(ai, bi);
        let c = u32::add_with_carry(&tmp.0, &carry);
        carry = tmp.1;
        res.push(c.0);
    }

    // Sum up the carry and remaining values from the longer number.
    let longer = if a.len() > b.len() { a } else { b }; // This is ok, no sensitive information here.
    for d in longer.iter().skip(min(a.len(), b.len())) {
        let c = u32::add_with_carry(d, &carry);
        carry = c.1;
        res.push(c.0);
    }
    res.push(carry);
    res
}

/// A very generic way of multiplying two vectors of u32.
fn mul_generic(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut res = vec![0u32; a.len() + b.len()];
    let (longer, shorter) = if a.len() > b.len() { (a, b) } else { (b, a) };
    let shorter_len = min(a.len(), b.len());

    fn looping(
        outer_start: usize,
        outer_end: usize,
        inner_end: usize,
        a: &[u32],
        b: &[u32],
        res: &mut Vec<u32>,
    ) {
        // TODO: make nicer loops
        for i in outer_start..outer_end {
            // longer.iter().skip(shorter_len)
            let mut inner_carry = 0u32;
            for j in 0..inner_end {
                // (higher, lower) = res[i+j] + ai * bi + inner_carry
                let (r_lower, r_higher) = u32::mul_with_carry(&a[i], &b[j]);
                let (tmp_lower, tmp_carry_higher) = u32::add_with_carry(&inner_carry, &res[i + j]);
                let (r_lower, add_carry_higher) = u32::add_with_carry(&r_lower, &tmp_lower);
                let r_higher = r_higher + tmp_carry_higher + add_carry_higher;

                res[i + j] = r_lower;
                inner_carry = r_higher; // TODO: simplify
            }
            res[i + inner_end] = inner_carry;
        }
    }

    // Iterate over min(a.len(), b.len()) elements
    looping(0, shorter_len, shorter_len, longer, shorter, &mut res);

    // Add the carry and remaining values from the longer number to the result.
    looping(
        shorter_len,
        longer.len(),
        shorter_len,
        longer,
        shorter,
        &mut res,
    );

    // TODO: trim output vector (remove zeroes we don't need).
    res
}
