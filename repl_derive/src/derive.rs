use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Visibility};

pub fn derive_interactive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let attr_matches = fields
        .iter()
        .filter(|field| matches!(field.vis, Visibility::Public(_)))
        .map(|field| {
            let name = &field.ident;
            quote! {
                stringify!(#name) => Ok(&self.#name as &dyn ::core::fmt::Debug),
            }
        });

    let interactive_attr_matches = fields
        .iter()
        .filter(|field| matches!(field.vis, Visibility::Public(_)))
        .map(|field| {
            let name = &field.ident;
            quote! {
                stringify!(#name) => Ok(&mut self.#name as &dyn repl::Interactive),
            }
        });

    let expanded = quote! {
        impl<'a> repl::Interactive<'a> for #name {
            fn __interactive_get_field(&'a self, field_name: &'a str) -> repl::Result<'a, &dyn ::core::fmt::Debug>{
                match field_name {
                    #(#attr_matches)*
                    _ => Err(repl::InteractiveError::AttributeNotFound{struct_name: stringify!(#name), field_name}),
                }
            }
            fn __interactive_get_interactive_field(&'a mut self, field_name: &'a str) -> repl::Result<&'a dyn repl::Interactive>{
                match field_name {
                    #(#interactive_attr_matches)*
                    _ => Err(repl::InteractiveError::AttributeNotFound{struct_name: stringify!(#name), field_name}),
                }
            }
        }
    };

    expanded.into()
}
