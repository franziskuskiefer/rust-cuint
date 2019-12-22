use std::ops::*;
use std::str::FromStr;

/// Uint errors
/// * StringParsingError when a string can't be parsed into a Uint.
#[derive(Debug)]
pub enum UintError {
    StringParsingError,
}

// TODO: restrict T?
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Uint<T> {
    pub(crate) digits: Vec<T>,
}

/// Trait defining all public functions on Uints
/// Operators are implemented with macros `impl_add`, `impl_mul`.
pub trait UintTrait: Default + PartialEq + Eq + Clone + FromStr {
    fn clear(&mut self);

    fn encode(&mut self, s: &str) -> Result<&Self, UintError>;
    fn decode(&self) -> Result<String, UintError>;

    fn add_uint(&self, other: &Self) -> Self;
    fn mul_uint(&self, other: &Self) -> Self;
    fn mod_pow(&self, exp: &Self, modulus: &Self) -> Self;
    fn mod_mul(&self, other: &Self, modulus: &Self) -> Self;

    // TODO: return error?
    fn to_str(&self) -> String {
        match self.decode() {
            Ok(s) => s,
            Err(_) => "".to_string(),
        }
    }
}

// ===================== Implement + ===========================
// let c = &a + &b;
// let c = a + &b;
// let c = a + b;
#[macro_export]
macro_rules! impl_add {
    ($($t:ty)*) => ($(
        impl Add<$t> for $t {
            type Output = $t;

            #[inline]
            fn add(self, other: $t) -> $t {
                self.add_uint(&other)
            }
        }

        impl<'a> Add<&'a $t> for $t {
            type Output = $t;

            #[inline]
            fn add(self, other: &$t) -> $t {
                self.add_uint(other)
            }
        }

        impl<'a, 'b> Add<&'b $t> for &'a $t {
            type Output = $t;

            #[inline]
            fn add(self, other: &$t) -> $t {
                self.add_uint(other)
            }
        }
    )*)
}

// ===================== Implement * ===========================
// let c = &a * &b;
// let c = a * &b;
// let c = a * b;
#[macro_export]
macro_rules! impl_mul {
    ($($t:ty)*) => ($(
        impl Mul<$t> for $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: $t) -> $t {
                self.mul_uint(&other)
            }
        }

        impl<'a> Mul<&'a $t> for $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: &$t) -> $t {
                self.mul_uint(other)
            }
        }

        impl<'a, 'b> Mul<&'b $t> for &'a $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: &$t) -> $t {
                self.mul_uint(other)
            }
        }
    )*)
}

// ============== TODO: stack allocated cuint ===============

#[macro_export]
macro_rules! create_cuint {
    ($name:ident, $size:expr, $limb_type:ident) => {
        #[derive(Clone, Copy)]
        pub struct $name {
            pub(crate) digits: [$limb_type; $size],
        }
        impl Default for $name {
            fn default() -> Self {
                Self {
                    digits: [0 as $limb_type; $size],
                }
            }
        }
    };
}
