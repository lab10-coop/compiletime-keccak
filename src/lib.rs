#![recursion_limit="128"]

extern crate proc_macro;
use crate::proc_macro::{ TokenStream };

mod utils;

#[proc_macro]
pub fn make_compiletime_h256(item: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(item as syn::Ident);
    let hash = utils::keccak(name.to_string().as_bytes());

    let all_vals: Vec<proc_macro2::TokenStream> = hash.as_ref().iter().map(|val| {
        quote::quote! { #val, }
    }).collect();

    let result = quote::quote! {
        fn #name() -> H256 { H256::from([ #(#all_vals)* ]) }
    };
    result.into()
}
