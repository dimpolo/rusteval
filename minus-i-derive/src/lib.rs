extern crate proc_macro;

use proc_macro::TokenStream;

mod derive;
mod methods;

#[proc_macro_derive(Interactive)]
pub fn derive_interactive(input: TokenStream) -> TokenStream {
    derive::derive_interactive(input)
}

#[proc_macro_derive(InteractiveRoot)]
pub fn derive_interactive_root(input: TokenStream) -> TokenStream {
    derive::derive_interactive_root(input)
}

#[proc_macro_derive(PartialDebug)]
pub fn derive_partial_debug(input: TokenStream) -> TokenStream {
    derive::derive_partial_debug(input)
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn InteractiveMethods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    methods::interactive_methods(input)
}
