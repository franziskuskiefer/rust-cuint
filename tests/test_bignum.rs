extern crate bignum;
use bignum::uint::BigNum;

#[test]
fn test_add() {
    let a = BigNum::new(1);
    let b = BigNum::new(2);
    println!("{:?}", &a + &b);
    assert_eq!(&a + &b, BigNum::new(3));
}
