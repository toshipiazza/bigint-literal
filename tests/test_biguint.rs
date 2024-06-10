use bigint_literal::biguint;
use num::BigUint;

#[test]
fn test_base10_implicit_radix() {
    let a: BigUint = 10000000u32.into();
    let b = biguint!("10000000");
    assert_eq!(a, b);
}

#[test]
fn test_base10_explicit_radix() {
    let a: BigUint = 10000000u32.into();
    let b = biguint!("10000000", 10);
    assert_eq!(a, b);
}

#[test]
fn test_base16_explicit_radix() {
    let a: BigUint = 0xabcdefu32.into();
    let b = biguint!("abcdef", 16);
    assert_eq!(a, b);
}

#[test]
fn test_base16_explicit_radix_large() {
    let b = biguint!("ffffffffffffffffffffffffffffffff", 16);
    let digs = b.to_u32_digits();
    assert_eq!(digs.len(), 4); // b should have 4 u32 limbs
    assert_eq!(digs[0], 0xffffffffu32);
    assert_eq!(digs[1], 0xffffffffu32);
    assert_eq!(digs[2], 0xffffffffu32);
    assert_eq!(digs[3], 0xffffffffu32);
}

#[test]
fn test_base10_integer_literal_u8() {
    let a: BigUint = 250u8.into();
    let b = biguint!(250u8);
    assert_eq!(a, b);
}

#[test]
fn test_base10_integer_literal_u16() {
    let a: BigUint = 65530u16.into();
    let b = biguint!(65530u16);
    assert_eq!(a, b);
}

#[test]
fn test_base10_integer_literal_u32() {
    let a: BigUint = 3405691582u32.into();
    let b = biguint!(3405691582u32);
    assert_eq!(a, b);
}

#[test]
fn test_base10_integer_literal_u64() {
    let a: BigUint = 18446744073709551615u64.into();
    let b = biguint!(18446744073709551615u64);
    assert_eq!(a, b);
}

#[test]
fn test_base16_integer_literal_u8() {
    let a: BigUint = 0xffu8.into();
    let b = biguint!(0xffu8);
    assert_eq!(a, b);
}

#[test]
fn test_base16_integer_literal_u16() {
    let a: BigUint = 0xffffu16.into();
    let b = biguint!(0xffffu16);
    assert_eq!(a, b);
}

#[test]
fn test_base16_integer_literal_u32() {
    let a: BigUint = 0xffffffffu32.into();
    let b = biguint!(0xffffffffu32);
    assert_eq!(a, b);
}

#[test]
fn test_base16_integer_literal_u64() {
    let a: BigUint = 0xffffffffffffffffu64.into();
    let b = biguint!(0xffffffffffffffffu64);
    assert_eq!(a, b);
}
