extern crate proc_macro;
use crate::proc_macro::{ TokenStream };
use quote::quote;

mod utils;

#[proc_macro]
pub fn compiletime_keccak(item: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(item as syn::Ident);
    let hash = utils::keccak(name.to_string().as_bytes());

    let hash_values: Vec<proc_macro2::TokenStream> = hash.as_ref().iter().map(|val| {
        quote! { #val }
    }).collect();

    let result = quote! {
        fn #name() -> H256 { H256::from([ #(#hash_values),* ]) }
    };
    result.into()
}
