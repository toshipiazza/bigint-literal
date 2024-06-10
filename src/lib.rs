use num::{BigUint, Num};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ExprLit, Lit, LitInt, Result, Token,
};

struct Item(BigUint);

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = input.parse::<ExprLit>()?;
        match expr.lit {
            Lit::Str(s) => {
                let lookahead = input.lookahead1();
                let b = if lookahead.peek(Token![,]) {
                    let _comma = input.parse::<Token![,]>()?;
                    let radix = input.parse::<LitInt>()?.base10_parse::<u32>()?;
                    BigUint::from_str_radix(&s.value(), radix)
                } else {
                    BigUint::from_str_radix(&s.value(), 10)
                };
                b.map_err(|x| input.error(x)).map(Item)
            }
            Lit::Int(i) => {
                let b = BigUint::from_str_radix(i.base10_digits(), 10);
                b.map_err(|x| input.error(x)).map(Item)
            }
            _ => Err(input.error("Expected integer or string literal")),
        }
    }
}

#[proc_macro]
pub fn biguint(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as Item);
    let dig = &lit.0.to_u32_digits();
    let gen = quote!(num::BigUint::from_slice(&[ #(#dig,)* ]));
    TokenStream::from(gen)
}
