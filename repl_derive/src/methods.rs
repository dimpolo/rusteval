use proc_macro::TokenStream;

use quote::quote;
use syn::export::TokenStream2;
use syn::{parse_macro_input, FnArg, ImplItem, ImplItemMethod, ItemImpl, Visibility};

pub fn interactive_methods(input: TokenStream) -> TokenStream {
    let original_impl = TokenStream2::from(input.clone());
    let ast = parse_macro_input!(input as ItemImpl);

    let struct_name = &ast.self_ty;

    let methods = ast.items.iter().filter_map(|item| match item {
        ImplItem::Method(method) => Some(method),
        _ => None,
    });

    let method_matches = methods.filter_map(gen_method_match_expr);

    let expanded = quote! {
        #original_impl

        impl<'a, F, R> repl::InteractiveMethods<'a, F, R> for #struct_name {
            fn __interactive_eval_method(
                &'a mut self,
                method_name: &'a str,
                args: &'a str,
                f: F,
            ) -> R
            where
                F: Fn(repl::Result<'a, &dyn ::core::fmt::Debug>) -> R,
            {
                let args = args.split_terminator(',');
                let args_count = args.count();
                match method_name {
                    #(#method_matches)*

                    _ => f(Err(repl::InteractiveError::MethodNotFound {
                        struct_name: stringify!(#struct_name),
                        method_name,
                    })),
                }
            }
        }
    };

    // eprintln!("{}", expanded);
    expanded.into()
}

fn gen_method_match_expr(method: &ImplItemMethod) -> Option<TokenStream2> {
    // skip methods that are not pub
    if !matches!(method.vis, Visibility::Public(_)) {
        return None;
    }

    // skip associated functions
    if !matches!(method.sig.inputs.first(), Some(FnArg::Receiver(_))) {
        return None;
    }

    let method_ident = &method.sig.ident;

    // don't count self
    let expected_arg_len = method.sig.inputs.len() - 1;

    let args_len_check = quote! {
        if args_count != #expected_arg_len{
            return f(Err(repl::InteractiveError::WrongNumberOfArguments{
                expected: #expected_arg_len,
                found: args_count,
            }));
        }
    };

    let method_call = quote! {
        f(Ok(&self.#method_ident()))
    };

    Some(quote! {
        stringify!(#method_ident) => {
            #args_len_check
            #method_call
        }
    })
}
