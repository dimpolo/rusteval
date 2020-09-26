extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Visibility};

#[proc_macro_derive(Interactive)]
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
            let name_string = format!("{}", name.as_ref().unwrap());
            quote! {
                #name_string => Ok(&self.#name as &dyn core::fmt::Debug),
            }
        });

    let _interactive_attr_matches = fields
        .iter()
        .filter(|field| matches!(field.vis, Visibility::Public(_)))
        .map(|field| {
            let name = &field.ident;
            let name_string = format!("{}", name.as_ref().unwrap());
            quote! {
                #name_string => Ok(&mut self.#name as &mut dyn repl::Interactive),
            }
        });

    let expanded = quote! {
        impl<'a> repl::Interactive<'a> for #name {
            fn __interactive_get_attribute(&'a self, attribute_name: &'a str) -> repl::Result<'a, &dyn core::fmt::Debug>{
                match attribute_name {
                    #(#attr_matches)*
                    _ => Err(repl::InteractiveError::AttributeNotFound{struct_name: stringify!(#name), attribute_name}),
                }
            }
            fn __interactive_get_interactive_attribute(&'a mut self, attribute_name: &'a str) -> repl::Result<&'a mut dyn repl::Interactive>{
                match attribute_name {
                    _ => Err(repl::InteractiveError::AttributeNotFound{struct_name: stringify!(#name), attribute_name}),
                }
            }
            fn __interactive_call_method(&'a mut self, method_name: &'a str, _args: &'a str) -> repl::Result<'a, Option<&mut dyn core::fmt::Debug>>{
                Err(repl::InteractiveError::MethodNotFound{struct_name: stringify!(#name), method_name})
            }
        }
    };

    expanded.into()
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
