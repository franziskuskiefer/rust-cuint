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
//! TODO: add fixed-length versions (no Vec, dynamic allocations)
//!

use std::cmp::min;
use std::ops::{Add, Mul};
use std::str::FromStr;

use base::*;
use util::*;

// ===============================
impl FromStr for Uint<u32> {
    type Err = UintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = Uint::<u32>::default();
        match r.encode(s) {
            Ok(_) => Ok(r),
            Err(err) => Err(err),
        }
    }
}

impl UintTrait for Uint<u32> {
    // TODO: not only hex?
    /// Read a hex string into a Uint<u32>.
    /// The string MUST be of the form "0xdeadbeef".
    fn encode(&mut self, s: &str) -> Result<&Uint<u32>, UintError> {
        if !s.starts_with("0x") {
            return Err(UintError::StringParsingError);
        }

        let mut x = &s[2..];
        let mut len = x.len();
        self.clear();

        // TODO: error prone, need to update len correctly
        while len > 0 {
            let cut = len - min(len, 8);
            let num = match u32::from_str_radix(&x[cut..], 16) {
                Ok(n) => n,
                Err(_) => return Err(UintError::StringParsingError),
            };
            self.digits.push(num);

            // Drop the part of the string we parsed.
            x = &x[..cut];
            len = x.len();
        }

        Ok(self)
    }

    /// Get a Uint<u32> as a hex string of the form `0xdeadbeef`.
    fn decode(&self) -> Result<String, UintError> {
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

    /// Add two Uint<u32>.
    /// This uses a generic, slow addition algorithm at this time.
    ///
    /// # Example:
    /// ```rust,ignore
    ///     let a = Uint::<u32>::from_str("0x123");
    ///     let b = Uint::<u32>::from_str("0x456");
    ///     let c = a.add_cuint32(&b);
    /// ```
    fn add_uint(&self, other: &Self) -> Self {
        let res = add_generic(&self.digits, &other.digits);
        Self { digits: res }
    }

    /// Multiply two Uint<u32>.
    /// This uses a generic, slow multiplication algorithm at this time.
    ///
    /// # Example:
    /// ```rust,ignore
    ///     let a = Uint::<u32>::from_str("0x123");
    ///     let b = Uint::add_generic(a: &[u32], b: &[u32])<u32>::from_str("0x456");
    ///     let c = a.mul_cuint32(&b);
    /// ```
    fn mul_uint(&self, other: &Self) -> Self {
        let res = mul_generic(&self.digits, &other.digits);
        Self { digits: res }
    }

    // FIXME: implement mod_pow
    fn mod_pow(&self, exp: &Self, modulus: &Self) -> Self {
        unimplemented!();
    }

    // FIXME: implement mul_pow
    fn mod_mul(&self, other: &Self, modulus: &Self) -> Self {
        unimplemented!();
    }

    /// Clear a Uint<u32>, i.e. this Uint<u32> == 0 after this operation.
    fn clear(&mut self) {
        self.digits.clear();
    }
}

impl_add!(Uint<u32>);
impl_mul!(Uint<u32>);

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
