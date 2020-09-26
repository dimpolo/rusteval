mod derive;
mod methods;

extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro_derive(Interactive)]
pub fn derive_interactive(input: TokenStream) -> TokenStream {
    derive::derive_interactive(input)
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn InteractiveMethods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    methods::interactive_methods(input)
}

#[proc_macro]
pub fn repl(_input: TokenStream) -> TokenStream {
    let expanded = quote! {{
        struct Repl{
            field: u32,
        }

        impl Repl{
            fn new(field: u32)->Self{
                Self{field}
            }
        }

        Repl::new(42)
    }};
    expanded.into()
}
