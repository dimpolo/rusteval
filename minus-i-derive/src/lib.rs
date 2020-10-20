//! Procedural macros for the [minus-i](../minus_i/index.html) crate.
//! Don't use this crate directly.
// TODO correct link

#![warn(trivial_casts, rust_2018_idioms)]

use proc_macro::TokenStream;

mod derive;
mod functions;

#[proc_macro_derive(Interactive)]
pub fn derive_interactive(input: TokenStream) -> TokenStream {
    derive::derive_interactive(input)
}

#[proc_macro_derive(InteractiveRoot)]
pub fn derive_interactive_root(input: TokenStream) -> TokenStream {
    derive::derive_root(input)
}

#[proc_macro_derive(PartialDebug)]
pub fn derive_partial_debug(input: TokenStream) -> TokenStream {
    derive::derive_partial_debug(input)
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    functions::methods(input)
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    functions::function(input)
}
