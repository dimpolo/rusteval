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

    let eval_attr_matches = fields
        .iter()
        .filter(|field| matches!(field.vis, Visibility::Public(_)))
        .map(|field| {
            let name = &field.ident;
            quote! {
                stringify!(#name) => f(Ok(&self.#name)),
            }
        });

    let attr_matches = fields
        .iter()
        .filter(|field| matches!(field.vis, Visibility::Public(_)))
        .map(|field| {
            let name = &field.ident;
            quote! {
                stringify!(#name) => Ok(&mut self.#name),
            }
        });

    let expanded = quote! {
        impl<'a, F, R> repl::Interactive<'a, F, R> for #name {
            fn __interactive_get_field(&'a mut self, field_name: &'a str) -> repl::Result<'a, &mut dyn repl::Interactive<'a, F, R>>{
                match field_name {
                    #(#attr_matches)*
                    _ => Err(repl::InteractiveError::AttributeNotFound{struct_name: stringify!(#name), field_name}),
                }
            }
            fn __interactive_eval_field(&'a self, field_name: &'a str, f: F) -> R
            where
                F: Fn(repl::Result<'a, &dyn ::core::fmt::Debug>) -> R,
            {
                match field_name {
                    #(#eval_attr_matches)*
                    _ => f(Err(repl::InteractiveError::AttributeNotFound{struct_name: stringify!(#name), field_name})),
                }
            }
        }
    };

    expanded.into()
}
