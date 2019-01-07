#![recursion_limit="128"]

extern crate proc_macro;
use crate::proc_macro::TokenStream;

mod utils;

#[proc_macro]
pub fn make_compiletime_h256(item: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(item as syn::Ident);
    let hash = utils::keccak(name.to_string().as_bytes());
    let e0 = hash.as_ref()[0];
    let e1 = hash.as_ref()[1];
    let e2 = hash.as_ref()[2];
    let e3 = hash.as_ref()[3];
    let e4 = hash.as_ref()[4];
    let e5 = hash.as_ref()[5];
    let e6 = hash.as_ref()[6];
    let e7 = hash.as_ref()[7];
    let e8 = hash.as_ref()[8];
    let e9 = hash.as_ref()[9];
    let e10 = hash.as_ref()[10];
    let e11 = hash.as_ref()[11];
    let e12 = hash.as_ref()[12];
    let e13 = hash.as_ref()[13];
    let e14 = hash.as_ref()[14];
    let e15 = hash.as_ref()[15];
    let e16 = hash.as_ref()[16];
    let e17 = hash.as_ref()[17];
    let e18 = hash.as_ref()[18];
    let e19 = hash.as_ref()[19];
    let e20 = hash.as_ref()[20];
    let e21 = hash.as_ref()[21];
    let e22 = hash.as_ref()[22];
    let e23 = hash.as_ref()[23];
    let e24 = hash.as_ref()[24];
    let e25 = hash.as_ref()[25];
    let e26 = hash.as_ref()[26];
    let e27 = hash.as_ref()[27];
    let e28 = hash.as_ref()[28];
    let e29 = hash.as_ref()[29];
    let e30 = hash.as_ref()[30];
    let e31 = hash.as_ref()[31];

    let result = quote::quote! {
        fn #name() -> H256 { H256::from([#e0,  #e1,  #e2,  #e3,  #e4,  #e5,  #e6,  #e7,
                                         #e8,  #e9,  #e10, #e11, #e12, #e13, #e14, #e15,
                                         #e16, #e17, #e18, #e19, #e20, #e21, #e22, #e23,
                                         #e24, #e25, #e26, #e27, #e28, #e29, #e30, #e31]) }
    };
    result.into()
}
