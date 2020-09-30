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

    let interactive_methods: Vec<_> = methods.filter(is_interactive_method).collect();

    let method_matches = interactive_methods.iter().map(gen_method_match_expr);

    let all_method_names = interactive_methods.iter().map(|method| {
        let name = &method.sig.ident;
        quote! {
            stringify!(#name),
        }
    });

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

        impl repl::InteractiveMethodNames for #struct_name {
            fn get_all_interactive_method_names(&self) -> &'static [&'static str]{
                &[#(#all_method_names)*]
            }
        }
    };

    // eprintln!("{}", expanded);
    expanded.into()
}

fn is_interactive_method(method: &&ImplItemMethod) -> bool {
    // skip methods that are not pub and associated functions

    matches!(method.vis, Visibility::Public(_))
        && matches!(method.sig.inputs.first(), Some(FnArg::Receiver(_)))
}

fn gen_method_match_expr(method: &&ImplItemMethod) -> TokenStream2 {
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
    let method_call = if expected_arg_len > 1 {
        quote! { unimplemented!()}
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
