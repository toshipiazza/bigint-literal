use bigint_literal::bigint;
use num::{BigInt, bigint::Sign};

#[test]
fn test_base10_implicit_radix_plus() {
    let a: BigInt = 10000000i32.into();
    let b = bigint!("10000000");
    assert_eq!(b.sign(), Sign::Plus);
    assert_eq!(a, b);
}

#[test]
fn test_base10_implicit_radix_minus() {
    let a: BigInt = (-10000000i32).into();
    let b = bigint!("-10000000");
    assert_eq!(b.sign(), Sign::Minus);
    assert_eq!(a, b);
}

#[test]
fn test_base10_explicit_radix_plus() {
    let a: BigInt = 10000000i32.into();
    let b = bigint!("10000000", 10);
    assert_eq!(b.sign(), Sign::Plus);
    assert_eq!(a, b);
}


#[test]
fn test_base10_explicit_radix_minus() {
    let a: BigInt = (-10000000i32).into();
    let b = bigint!("-10000000", 10);
    assert_eq!(b.sign(), Sign::Minus);
    assert_eq!(a, b);
}

#[test]
fn test_base16_explicit_radix_plus() {
    let a: BigInt = 0xabcdefi32.into();
    let b = bigint!("abcdef", 16);
    assert_eq!(b.sign(), Sign::Plus);
    assert_eq!(a, b);
}

#[test]
fn test_base16_explicit_radix_minus() {
    let a: BigInt = (-0xabcdefi32).into();
    let b = bigint!("-abcdef", 16);
    assert_eq!(b.sign(), Sign::Minus);
    assert_eq!(a, b);
}

#[test]
fn test_base16_explicit_radix_large() {
    let b = bigint!("-ffffffffffffffffffffffffffffffff", 16);
    let (sign, digs) = b.to_u32_digits();
    assert_eq!(digs.len(), 4); // b should have 4 u32 limbs
    assert_eq!(digs[0], 0xffffffffu32);
    assert_eq!(digs[1], 0xffffffffu32);
    assert_eq!(digs[2], 0xffffffffu32);
    assert_eq!(digs[3], 0xffffffffu32);
    assert_eq!(sign, Sign::Minus);
}

#[test]
fn test_base10_integer_literal_i8_plus() {
    let a: BigInt = 50i8.into();
    let b = bigint!(50i8);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Plus);
}

#[test]
fn test_base10_integer_literal_i8_minus() {
    let a: BigInt = (-50i8).into();
    let b = bigint!(-50i8);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Minus);
}

#[test]
fn test_base10_integer_literal_i16_plus() {
    let a: BigInt = 25530i16.into();
    let b = bigint!(25530i16);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Plus);
}

#[test]
fn test_base10_integer_literal_i16_minus() {
    let a: BigInt = (-25530i16).into();
    let b = bigint!(-25530i16);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Minus);
}

#[test]
fn test_base10_integer_literal_i32_plus() {
    let a: BigInt = 1405691582i32.into();
    let b = bigint!(1405691582i32);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Plus);
}

#[test]
fn test_base10_integer_literal_i32_minus() {
    let a: BigInt = (-1405691582i32).into();
    let b = bigint!(-1405691582i32);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Minus);
}


#[test]
fn test_base10_integer_literal_i64_plus() {
    let a: BigInt = 8446744073709551615i64.into();
    let b = bigint!(8446744073709551615i64);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Plus);
}

#[test]
fn test_base10_integer_literal_i64_minus() {
    let a: BigInt = (-8446744073709551615i64).into();
    let b = bigint!(-8446744073709551615i64);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Minus);
}

#[test]
fn test_base16_integer_literal_i8_plus() {
    let a: BigInt = 0x1fi8.into();
    let b = bigint!(0x1fi8);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Plus);
}

#[test]
fn test_base16_integer_literal_i8_minus() {
    let a: BigInt = (-0x1fi8).into();
    let b = bigint!(-0x1fi8);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Minus);
}

#[test]
fn test_base16_integer_literal_i16_plus() {
    let a: BigInt = 0x1fffi16.into();
    let b = bigint!(0x1fffi16);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Plus);
}

#[test]
fn test_base16_integer_literal_i16_minus() {
    let a: BigInt = (-0x1fffi16).into();
    let b = bigint!(-0x1fffi16);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Minus);
}

#[test]
fn test_base16_integer_literal_i32_plus() {
    let a: BigInt = 0x1fffffffi32.into();
    let b = bigint!(0x1fffffffi32);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Plus);
}

#[test]
fn test_base16_integer_literal_i32_minus() {
    let a: BigInt = (-0x1fffffffi32).into();
    let b = bigint!(-0x1fffffffi32);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Minus);
}

#[test]
fn test_base16_integer_literal_i64_plus() {
    let a: BigInt = 0x1fffffffffffffffi64.into();
    let b = bigint!(0x1fffffffffffffffi64);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Plus);
}

#[test]
fn test_base16_integer_literal_i64_minus() {
    let a: BigInt = (-0x1fffffffffffffffi64).into();
    let b = bigint!(-0x1fffffffffffffffi64);
    assert_eq!(a, b);
    assert_eq!(b.sign(), Sign::Minus);
}
