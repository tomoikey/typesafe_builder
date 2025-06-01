mod derive_builder;
mod input;

use darling::FromDeriveInput;
use derive_builder::derive_builder_impl;
use input::Input;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let input = match Input::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    match derive_builder_impl(input) {
        Ok(expanded) => TokenStream::from(expanded),
        Err(e) => TokenStream::from(e.write_errors()),
    }
}
