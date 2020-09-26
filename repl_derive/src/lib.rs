extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, ImplItem, ItemImpl, ReturnType, Visibility};

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
            quote! {
                stringify!(#name) => Ok(&self.#name as &dyn ::core::fmt::Debug),
            }
        });

    let _interactive_attr_matches = fields
        .iter()
        .filter(|field| matches!(field.vis, Visibility::Public(_)))
        .map(|field| {
            let name = &field.ident;
            quote! {
                stringify!(#name) => Ok(&mut self.#name as &mut dyn repl::Interactive),
            }
        });

    let expanded = quote! {
        impl<'a> repl::Interactive<'a> for #name {
            fn __interactive_get_attribute(&'a self, attribute_name: &'a str) -> repl::Result<'a, &dyn ::core::fmt::Debug>{
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
            /*
            fn __interactive_call_method(&'a mut self, method_name: &'a str, _args: &'a str) -> repl::Result<'a, ::core::option::Option<&dyn ::core::fmt::Debug>>{
                Err(repl::InteractiveError::MethodNotFound{struct_name: stringify!(#name), method_name})
            }
            */
        }
    };

    expanded.into()
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn InteractiveMethods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let output = proc_macro2::TokenStream::from(input.clone());
    let ast = parse_macro_input!(input as ItemImpl);
    /*
    let name = if let Type::Path(TypePath {
        path: Path { segments: seg, .. },
        ..
    }) = *ast.self_ty
    {
        seg
    } else {
        unimplemented!()
    };*/

    let name = &ast.self_ty;

    let methods = ast.items.iter().filter_map(|item| match item {
        ImplItem::Method(method) => Some(method),
        _ => None,
    });

    let public_methods = methods.filter(|method| matches!(method.vis, Visibility::Public(_)));
    let method_matches = public_methods.map(|method| {
        let method_ident = &method.sig.ident;
        match &method.sig.output{
            ReturnType::Default => quote! {
                stringify!(#method_ident) => Ok({
                    self.#method_ident();
                    None
                }),
            },
            ReturnType::Type(_, _) => {
                quote! {
                    stringify!(#method_ident) => Ok(Some(Box::new(self.#method_ident()) as Box<dyn ::core::fmt::Debug>)),}
            }
        }
    });

    let expanded = quote! {
        #output

        impl<'a> repl::InteractiveMethods<'a> for #name {
            fn __interactive_call_method(
                &'a mut self,
                method_name: &'a str,
                _args: &'a str,
            ) -> repl::Result<'a, ::core::option::Option<Box<dyn ::core::fmt::Debug>>> {
                match method_name {
                    #(#method_matches)*

                    _ => Err(repl::InteractiveError::MethodNotFound {
                        struct_name: stringify!(#name),
                        method_name,
                    }),
                }
            }
        }
    };

    // eprintln!("{:#?}", public_methods.collect::<Vec<_>>());
    // ast.into_token_stream().into()

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
