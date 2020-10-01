use proc_macro::TokenStream;

use quote::quote;
use syn::export::TokenStream2;
use syn::{parse_macro_input, DeriveInput, Field, Visibility};

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

        #[cfg(feature = "std")]
        impl #struct_name {
            fn eval_to_debug_string(&mut self, expression: &str) -> String {
                self.try_eval(expression, |result| format!("{:?}", result))
            }
        }
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

    let get_interactive_fields = || fields.iter().filter(is_interactive_field);

    let eval_field_matches = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name) => f(Ok(&self.#name)),
        }
    });

    let get_field_matches = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name) => Ok(&self.#name),
        }
    });

    let get_field_mut_matches = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name) => Ok(&mut self.#name),
        }
    });

    let all_field_names = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name),
        }
    });

    quote! {
        impl<'a, F, R> repl::Interactive<'a, F, R> for #struct_name {
            fn __interactive_get_field(&'a self, field_name: &'a str) -> repl::Result<'a, &dyn repl::Interactive<'a, F, R>>{
                match field_name {
                    #(#get_field_matches)*
                    _ => Err(repl::InteractiveError::FieldNotFound{struct_name: stringify!(#struct_name), field_name}),
                }
            }
            fn __interactive_get_field_mut(&'a mut self, field_name: &'a str) -> repl::Result<'a, &mut dyn repl::Interactive<'a, F, R>>{
                match field_name {
                    #(#get_field_mut_matches)*
                    _ => Err(repl::InteractiveError::FieldNotFound{struct_name: stringify!(#struct_name), field_name}),
                }
            }

        }

        impl<'a, F, R> repl::InteractiveFields<'a, F, R> for #struct_name {
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

        impl repl::InteractiveFieldNames for #struct_name {
            fn get_all_interactive_field_names(&self) -> &'static [&'static str]{
                &[#(#all_field_names)*]
            }
        }
    }
}

fn is_interactive_field(field: &&Field) -> bool {
    matches!(field.vis, Visibility::Public(_))
}
