use num::{bigint::Sign, BigInt, BigUint, Num};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ExprLit, Lit, LitInt, Result, Token,
};

struct DigitsAndRadix(String, u32);

impl Parse for DigitsAndRadix {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = input.parse::<ExprLit>()?;
        match expr.lit {
            Lit::Int(i) => Ok(DigitsAndRadix(i.base10_digits().to_string(), 10)),
            Lit::Str(s) => {
                let lookahead = input.lookahead1();
                if lookahead.peek(Token![,]) {
                    let _comma = input.parse::<Token![,]>()?;
                    let radix = input.parse::<LitInt>()?.base10_parse::<u32>()?;
                    Ok(DigitsAndRadix(s.value(), radix))
                } else {
                    Ok(DigitsAndRadix(s.value(), 10))
                }
            }
            _ => Err(input.error("Expected integer or string literal")),
        }
    }
}

struct BigU(BigUint);

impl Parse for BigU {
    fn parse(input: ParseStream) -> Result<Self> {
        let digits = DigitsAndRadix::parse(input)?;
        BigUint::from_str_radix(&digits.0, digits.1)
            .map_err(|x| input.error(x))
            .map(Self)
    }
}

struct BigI(BigInt);

impl Parse for BigI {
    fn parse(input: ParseStream) -> Result<Self> {
        let digits = DigitsAndRadix::parse(input)?;
        BigInt::from_str_radix(&digits.0, digits.1)
            .map_err(|x| input.error(x))
            .map(Self)
    }
}

#[proc_macro]
pub fn biguint(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as BigU);
    let dig = &lit.0.to_u32_digits();
    let gen = quote!(num::BigUint::from_slice(&[ #(#dig,)* ]));
    TokenStream::from(gen)
}

#[proc_macro]
pub fn bigint(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as BigI);
    let (sign, dig) = &lit.0.to_u32_digits();
    let sign = match sign {
        Sign::Plus => quote!(num::bigint::Sign::Plus),
        Sign::Minus => quote!(num::bigint::Sign::Minus),
        Sign::NoSign => panic!("Internal error: BigInt has no sign (invalid representation)"),
    };
    let gen = quote!(num::BigInt::from_slice(#sign, &[ #(#dig,)* ]));
    TokenStream::from(gen)
}
