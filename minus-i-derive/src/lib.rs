//! Procedural macros for the [minus-i](../minus_i/index.html) crate.
//! Don't use this crate directly.
// TODO correct link

#![warn(missing_docs, trivial_casts, rust_2018_idioms)]

use proc_macro::TokenStream;

mod derive;
mod functions;

/// Gives interactive access to a structs fields.
#[proc_macro_derive(Interactive)]
pub fn derive_interactive(input: TokenStream) -> TokenStream {
    derive::derive_interactive(input)
}

/// Makes a struct an interactive access point for your application.
#[proc_macro_derive(InteractiveRoot)]
pub fn derive_interactive_root(input: TokenStream) -> TokenStream {
    derive::derive_root(input)
}

/// Implements [`Debug`] for a struct replacing all fields that do not implement `Debug` with a placeholder.
///
/// [`Debug`]: core::fmt::Debug
#[proc_macro_derive(PartialDebug)]
pub fn derive_partial_debug(input: TokenStream) -> TokenStream {
    derive::derive_partial_debug(input)
}

/// Gives interactive access to a structs methods.
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    functions::methods(input)
}

/// Gives interactive access to a function.
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    functions::function(input)
}
