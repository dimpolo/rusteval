use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemImpl, ReturnType, Visibility};

pub fn interactive_methods(input: TokenStream) -> TokenStream {
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
