use proc_macro::TokenStream;

use quote::quote;
use syn::export::{Span, TokenStream2};
use syn::*;

pub fn interactive_methods(input: TokenStream) -> TokenStream {
    let original_impl = TokenStream2::from(input.clone());
    let ast = parse_macro_input!(input as ItemImpl);

    let struct_name = &ast.self_ty;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let methods = ast.items.iter().filter_map(|item| match item {
        ImplItem::Method(method) => Some(method),
        _ => None,
    });

    let interactive_methods: Vec<_> = methods.filter(is_interactive_method).collect();

    let method_matches = interactive_methods
        .iter()
        .filter(|method| {
            matches!(
                method.sig.receiver(),
                Some(FnArg::Receiver(Receiver {
                    mutability: None, ..
                }))
            )
        })
        .map(gen_method_match_expr);

    let method_mut_matches = interactive_methods.iter().map(gen_method_match_expr);
    // TODO don't duplicate &self methods

    let all_method_names = interactive_methods.iter().map(|method| {
        let name = &method.sig.ident;
        quote! {
            stringify!(#name),
        }
    });

    // TODO add generics
    let expanded = quote! {
        #original_impl

        impl #impl_generics minus_i::InteractiveMethods for #struct_name #ty_generics #where_clause{
            fn interactive_eval_method(
                &self,
                method_name: &str,
                args: &str,
                f: &mut dyn FnMut(minus_i::Result<'_, &dyn ::core::fmt::Debug>),
            )
            {
                let args_count = args.split_terminator(',').count();
                match method_name {
                    #(#method_matches)*

                    _ => f(Err(minus_i::InteractiveError::MethodNotFound {
                        type_name: stringify!(#struct_name),
                        method_name,
                    })),
                }
            }

            fn interactive_eval_method_mut(
                &mut self,
                method_name: &str,
                args: &str,
                f: &mut dyn FnMut(minus_i::Result<'_, &dyn ::core::fmt::Debug>),
            )
            {
                let args_count = args.split_terminator(',').count();
                match method_name {
                    #(#method_mut_matches)*

                    _ => f(Err(minus_i::InteractiveError::MethodNotFound {
                        type_name: stringify!(#struct_name),
                        method_name,
                    })),
                }
            }
        }

        impl #impl_generics minus_i::InteractiveMethodNames for #struct_name #ty_generics #where_clause{
            fn get_all_interactive_method_names(&self) -> &'static [&'static str]{
                &[#(#all_method_names)*]
            }
        }
    };

    expanded.into()
}

fn is_interactive_method(method: &&ImplItemMethod) -> bool {
    // skip methods that are not pub and associated functions
    // TODO check if args are parseable

    matches!(method.vis, Visibility::Public(_))
        && matches!(method.sig.inputs.first(), Some(FnArg::Receiver(_)))
}

fn gen_method_match_expr(method: &&ImplItemMethod) -> TokenStream2 {
    let method_ident = &method.sig.ident;

    // don't count self
    let expected_arg_len = method.sig.inputs.len() - 1;

    let args_len_check = quote! {
        if args_count != #expected_arg_len{
            return f(Err(minus_i::InteractiveError::WrongNumberOfArguments{
                expected: #expected_arg_len,
                found: args_count,
            }));
        }
    };

    let args_error = quote! {
        minus_i::InteractiveError::ArgsError { given_args: args }
    };

    let get_arg_names =
        || (0..expected_arg_len).map(|num| Ident::new(&format!("arg{}", num), Span::call_site()));

    let args_parse = get_arg_names().map(|arg_name| {
        quote! {
            let #arg_name = args_iterator
                .next()
                .ok_or(#args_error)?
                .trim()
                .parse()
                .map_err(|_|#args_error)?;
        }
    });

    let args_call: Vec<_> = get_arg_names()
        .map(|arg_name| {
            quote! {
                #arg_name,
            }
        })
        .collect();

    let method_call = if expected_arg_len > 0 {
        quote! {
            let parse = || {
                let mut args_iterator = args.split_terminator(','); // TODO doesn't work on types like str
                #(#args_parse)*
                Ok((#(#args_call)*))
            };

            match parse(){
                Ok((#(#args_call)*)) => f(Ok(&self.#method_ident(#(#args_call)*))),
                Err(e) => f(Err(e)),
            }
        }
    } else {
        quote! {
            f(Ok(&self.#method_ident()))
        }
    };

    quote! {
        stringify!(#method_ident) => {
            #args_len_check
            #method_call
        }
    }
}
