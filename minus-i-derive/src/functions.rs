use proc_macro::TokenStream;

use quote::quote;
use syn::export::{Span, TokenStream2};
use syn::spanned::Spanned;
use syn::*;

static SUPPORTED_FUNC_ARGS: &[&str] = &[
    "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32",
    "u64", "u128", "usize", "String",
];

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

    let expanded = quote! {
        #original_impl

        impl #impl_generics ::minus_i::InteractiveMethods for #struct_name #ty_generics #where_clause{
            fn interactive_eval_method(
                &self,
                method_name: &str,
                args: &str,
                f: &mut dyn FnMut(::minus_i::Result<'_, &dyn ::core::fmt::Debug>),
            )
            {
                match method_name {
                    #(#method_matches)*

                    _ => f(Err(::minus_i::InteractiveError::MethodNotFound {
                        type_name: stringify!(#struct_name),
                        method_name,
                    })),
                }
            }

            fn interactive_eval_method_mut(
                &mut self,
                method_name: &str,
                args: &str,
                f: &mut dyn FnMut(::minus_i::Result<'_, &dyn ::core::fmt::Debug>),
            )
            {
                match method_name {
                    #(#method_mut_matches)*

                    _ => f(Err(::minus_i::InteractiveError::MethodNotFound {
                        type_name: stringify!(#struct_name),
                        method_name,
                    })),
                }
            }
        }

        impl #impl_generics ::minus_i::InteractiveMethodNames for #struct_name #ty_generics #where_clause{
            fn get_all_interactive_method_names(&self) -> &'static [&'static str]{
                &[#(#all_method_names)*]
            }
        }
    };

    expanded.into()
}

pub fn interactive_function(input: TokenStream) -> TokenStream {
    let original_func = TokenStream2::from(input.clone());

    let struct_name = &Ident::new(
        &format!("InteractiveFunction{}", hash(&input)),
        Span::call_site(),
    );

    let ast = parse_macro_input!(input as ImplItemMethod);

    let function_name = &ast.sig.ident;

    let method_call = gen_method_call(&ast, &None);

    let expanded = quote! {
        #original_func

        struct #struct_name;

        impl ::minus_i::InteractiveFunction for #struct_name{
            fn function_name(&self) -> &'static str{
                stringify!(#function_name)
            }

            fn eval(&self, method_name: &str, args: &str, f: &mut dyn FnMut(::minus_i::Result<'_, &dyn ::core::fmt::Debug>)) {
                #method_call
            }
        }

        ::minus_i::inventory::submit! {
            &#struct_name as &dyn ::minus_i::InteractiveFunction
        }
    };

    expanded.into()
}

fn is_interactive_method(method: &&ImplItemMethod) -> bool {
    // skip associated functions
    // skip methods with argument types that are not supported

    matches!(method.sig.inputs.first(), Some(FnArg::Receiver(_)))
        && method.sig.inputs.iter().skip(1).all(is_supported_fn_arg)
}

fn gen_method_match_expr(method: &&ImplItemMethod) -> TokenStream2 {
    let method_ident = &method.sig.ident;
    let receiver = Some(quote! {self.});

    let method_call = gen_method_call(method, &receiver);

    quote! {
        stringify!(#method_ident) => {
            #method_call
        }
    }
}

fn get_expected_arg_len(method: &ImplItemMethod, receiver: &Option<TokenStream2>) -> usize {
    if receiver.is_some() {
        // don't count self
        method.sig.inputs.len() - 1
    } else {
        method.sig.inputs.len()
    }
}

fn gen_method_call(method: &ImplItemMethod, receiver: &Option<TokenStream2>) -> TokenStream2 {
    let method_ident = &method.sig.ident;

    let expected_arg_len = get_expected_arg_len(method, receiver);
    let args = if expected_arg_len == 1 { "arg" } else { "args" };
    let parse_func = Ident::new(
        &format!("parse_{}_{}", expected_arg_len, args),
        method.sig.inputs.span(),
    );

    let arg_names: Vec<_> = (0..expected_arg_len)
        .map(|arg_num| {
            let index = if receiver.is_some() {
                // skip self
                arg_num + 1
            } else {
                arg_num
            };

            let span = method.sig.inputs[index].span();
            Ident::new(&format!("arg{}", arg_num), span)
        })
        .collect();

    let args_punctuated: Vec<_> = arg_names
        .iter()
        .map(|arg_name| {
            quote! {
                #arg_name,
            }
        })
        .collect();

    quote! {
        match ::minus_i::arg_parse::#parse_func(method_name, args){
            Ok((#(#args_punctuated)*)) => f(Ok(& #receiver #method_ident(#(#args_punctuated)*))),
            Err(e) => f(Err(e)),
        }
    }
}

fn is_supported_fn_arg(arg: &FnArg) -> bool {
    if let FnArg::Typed(arg_type) = arg {
        if let Type::Path(type_path) = &*arg_type.ty {
            SUPPORTED_FUNC_ARGS
                .iter()
                .any(|supported_arg| type_path.path.is_ident(supported_arg))
        } else {
            false
        }
    } else {
        false
    }
}

fn hash(input: &TokenStream) -> u64 {
    use std::collections::hash_map;
    use std::hash::Hasher;

    let mut hasher = hash_map::DefaultHasher::new();
    hasher.write(input.to_string().as_bytes());
    hasher.finish()
}
