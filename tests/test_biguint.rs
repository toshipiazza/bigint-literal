use bigint_literal::biguint;
use num::{BigUint, FromPrimitive};

#[test]
fn test_base10_implicit_radix() {
    let b = biguint!("10000000");
    assert_eq!(b, BigUint::from_u32(10000000).unwrap());
}

#[test]
fn test_base10_explicit_radix() {
    let b = biguint!("10000000", 10);
    assert_eq!(b, BigUint::from_u32(10000000).unwrap());
}

#[test]
fn test_base16_explicit_radix() {
    let b = biguint!("abcdef", 16);
    assert_eq!(b, BigUint::from_u32(0xabcdef).unwrap());
}

#[test]
fn test_base10_integer_literal_u8() {
    let b = biguint!(250u8);
    assert_eq!(b, BigUint::from_u8(250).unwrap());
}

#[test]
fn test_base10_integer_literal_u16() {
    let b = biguint!(65530u16);
    assert_eq!(b, BigUint::from_u16(65530).unwrap());
}

#[test]
fn test_base10_integer_literal_u32() {
    let b = biguint!(3405691582u32);
    assert_eq!(b, BigUint::from_u32(3405691582u32).unwrap());
}

#[test]
fn test_base16_integer_literal_u8() {
    let b = biguint!(0xffu8);
    assert_eq!(b, BigUint::from_u8(0xff).unwrap());
}

#[test]
fn test_base16_integer_literal_u16() {
    let b = biguint!(0xffffu16);
    assert_eq!(b, BigUint::from_u16(0xffff).unwrap());
}

#[test]
fn test_base16_integer_literal_u32() {
    let b = biguint!(0xffffffffu32);
    assert_eq!(b, BigUint::from_u32(0xffffffff).unwrap());
}
