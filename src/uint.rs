use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BigNum {
    digits: Vec<u64>,
}

pub trait Digits {
    fn capacity(&self) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl Digits for BigNum {
    fn capacity(&self) -> usize {
        self.digits.capacity()
    }
    fn len(&self) -> usize {
        self.digits.len()
    }
    fn is_empty(&self) -> bool {
        self.digits.len() == 0
    }
}

impl<'a> Add<&'a BigNum> for BigNum {
    type Output = BigNum;

    fn add(self, other: &BigNum) -> BigNum {
        BigNum {
            digits: vec![self.digits[0] + other.digits[0]],
        }
        // self += other;
        // self
    }
}

macro_rules! forward_val_val_binop_commutative {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl $imp<$res> for $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                // forward to val-ref, with the larger capacity as val
                if self.capacity() >= other.capacity() {
                    $imp::$method(self, &other)
                } else {
                    $imp::$method(other, &self)
                }
            }
        }
    };
}

macro_rules! forward_ref_val_binop_commutative {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl<'a> $imp<$res> for &'a $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                // reverse, forward to val-ref
                $imp::$method(other, self)
            }
        }
    };
}

macro_rules! forward_ref_ref_binop_commutative {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl<'a, 'b> $imp<&'b $res> for &'a $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: &$res) -> $res {
                // forward to val-ref, choosing the larger to clone
                if self.len() >= other.len() {
                    $imp::$method(self.clone(), other)
                } else {
                    $imp::$method(other.clone(), self)
                }
            }
        }
    };
}

forward_val_val_binop_commutative!(impl Add for BigNum, add);
forward_ref_ref_binop_commutative!(impl Add for BigNum, add);
forward_ref_val_binop_commutative!(impl Add for BigNum, add);

// impl Add for BigNum {
//     type Output = BigNum;

//     #[inline(always)]
//     fn add(self, other: BigNum) -> BigNum {
//         BigNum {
//             digits: vec![self.digits[0] + other.digits[0]],
//         }
//     }
// }

impl BigNum {
    pub fn new(num: u64) -> BigNum {
        BigNum { digits: vec![num] }
    }
}
