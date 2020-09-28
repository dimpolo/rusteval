use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input, Visibility};
use syn::export::TokenStream2;

pub fn derive_interactive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    interactive_impl(&ast).into()
}

pub fn derive_interactive_root(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;

    let interactive_impl = interactive_impl(&ast);

    let expanded = quote! {
        #interactive_impl

        impl<'a, F: 'a, R: 'a> InteractiveRoot<'a, F, R> for #struct_name {}
    };

    expanded.into()
}

fn interactive_impl(ast: &DeriveInput) -> TokenStream2 {
    let struct_name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let eval_field_matches = fields
        .iter()
        .filter(|field| matches!(field.vis, Visibility::Public(_)))
        .map(|field| {
            let name = &field.ident;
            quote! {
                stringify!(#name) => f(Ok(&self.#name)),
            }
        });

    let get_field_matches = fields
        .iter()
        .filter(|field| matches!(field.vis, Visibility::Public(_)))
        .map(|field| {
            let name = &field.ident;
            quote! {
                stringify!(#name) => Ok(&mut self.#name),
            }
        });

    quote! {
        impl<'a, F, R> repl::Interactive<'a, F, R> for #struct_name {
            fn __interactive_get_field(&'a mut self, field_name: &'a str) -> repl::Result<'a, &mut dyn repl::Interactive<'a, F, R>>{
                match field_name {
                    #(#get_field_matches)*
                    _ => Err(repl::InteractiveError::FieldNotFound{struct_name: stringify!(#struct_name), field_name}),
                }
            }
            fn __interactive_eval_field(&'a self, field_name: &'a str, f: F) -> R
            where
                F: Fn(repl::Result<'a, &dyn ::core::fmt::Debug>) -> R,
            {
                match field_name {
                    #(#eval_field_matches)*
                    _ => f(Err(repl::InteractiveError::FieldNotFound{struct_name: stringify!(#struct_name), field_name})),
                }
            }
        }
    }
}
