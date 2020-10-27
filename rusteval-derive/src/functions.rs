use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::*;

#[cfg(feature = "std")]
static SUPPORTED_FUNC_ARGS: &[&str] = &[
    "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32",
    "u64", "u128", "usize", "String", "str",
];

#[cfg(not(feature = "std"))]
static SUPPORTED_FUNC_ARGS: &[&str] = &[
    "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32",
    "u64", "u128", "usize",
];

pub fn methods(input: TokenStream) -> TokenStream {
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

        impl #impl_generics ::rusteval::Methods for #struct_name #ty_generics #where_clause{
            fn eval_method(
                &self,
                method_name: &str,
                args: &str,
                f: &mut dyn FnMut(::rusteval::Result<'_, &dyn ::core::fmt::Debug>),
            )
            {
                match method_name {
                    #(#method_matches)*

                    _ => f(Err(::rusteval::InteractiveError::MethodNotFound {
                        type_name: stringify!(#struct_name),
                        method_name,
                    })),
                }
            }

            fn eval_method_mut(
                &mut self,
                method_name: &str,
                args: &str,
                f: &mut dyn FnMut(::rusteval::Result<'_, &dyn ::core::fmt::Debug>),
            )
            {
                match method_name {
                    #(#method_mut_matches)*

                    _ => f(Err(::rusteval::InteractiveError::MethodNotFound {
                        type_name: stringify!(#struct_name),
                        method_name,
                    })),
                }
            }

            fn get_all_method_names(&self) -> &'static [&'static str]{
                &[#(#all_method_names)*]
            }
        }
    };

    expanded.into()
}

pub fn function(input: TokenStream) -> TokenStream {
    let original_func = TokenStream2::from(input.clone());

    let struct_name = &Ident::new(&format!("Function{}", hash(&input)), original_func.span());

    let ast = parse_macro_input!(input as ImplItemMethod);

    let function_name = &ast.sig.ident;

    let method_call = gen_method_call(&ast, &None);

    let expanded = quote! {
        #original_func

        struct #struct_name;

        impl ::rusteval::Function for #struct_name{
            fn eval(&self, args: &str, f: &mut dyn FnMut(::rusteval::Result<'_, &dyn ::core::fmt::Debug>)) {
                let method_name = self.function_name();
                #method_call
            }
            fn function_name(&self) -> &'static str{
                stringify!(#function_name)
            }
        }

        ::rusteval::inventory::submit! {
            &#struct_name as &dyn ::rusteval::Function
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

/// Generate something like this:
/// ```ignore
/// "func" => match ::rusteval::arg_parse::parse_2_args(method_name, args) {
///     Ok((arg0, arg1, mut arg2)) => f(Ok(&self.add(arg0, &arg1, &mut arg2))),
///     Err(e) => f(Err(e)),
/// },
/// ```
///
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

            let arg = &method.sig.inputs[index];
            (
                Ident::new(&format!("arg{}", arg_num), arg.span()),
                reference_tokens(arg),
            )
        })
        .collect();

    // arg0, arg1, mut arg2
    let tuple_args = arg_names.iter().map(|(arg_name, ref_tokens)| {
        let mut_token = match ref_tokens {
            ReferenceTokens::NotStr { mut_token, .. } => mut_token,
            ReferenceTokens::Str { mut_token } => mut_token,
        };
        quote! {
            #mut_token #arg_name,
        }
    });
    // arg0, &arg1, &mut arg2
    let call_args = arg_names
        .iter()
        .map(|(arg_name, ref_tokens)| match ref_tokens {
            ReferenceTokens::NotStr {
                and_token,
                mut_token,
            } => {
                quote! {
                    #and_token #mut_token #arg_name,
                }
            }
            ReferenceTokens::Str { mut_token: None } => quote! {
                <::std::string::String as ::core::ops::Deref>::deref(& #arg_name),
            },
            ReferenceTokens::Str { mut_token: Some(_) } => quote! {
                <::std::string::String as ::core::ops::DerefMut>::deref_mut(&mut #arg_name),
            },
        });

    quote! {
        match ::rusteval::arg_parse::#parse_func(method_name, args){
            Ok((#(#tuple_args)*)) => f(Ok(& #receiver #method_ident(#(#call_args)*))),
            Err(e) => f(Err(e)),
        }
    }
}

/// true for bool, &bool, &mut bool, etc
/// false for &&bool or more complicated types like arrays, slices or generic types
fn is_supported_fn_arg(arg: &FnArg) -> bool {
    if let FnArg::Typed(PatType { ty: box ty, .. }) = arg {
        let type_path = match ty {
            Type::Path(type_path) => type_path,
            Type::Reference(TypeReference {
                elem: box Type::Path(type_path),
                ..
            }) => type_path,
            _ => return false,
        };

        SUPPORTED_FUNC_ARGS
            .iter()
            .any(|supported_arg| type_path.path.is_ident(supported_arg))
    } else {
        false
    }
}

enum ReferenceTokens<'a> {
    NotStr {
        and_token: Option<&'a Token!(&)>,
        mut_token: Option<&'a Token!(mut)>,
    },
    Str {
        mut_token: Option<&'a Token!(mut)>,
    },
}

/// u32 -> NotStr{ and_token: None, mut_token: None }
/// &u32 -> NotStr{ and_token: Some(&), mut_token: None }
/// &mut u32 -> NotStr{ and_token: Some(&), mut_token: Some(mut) }
///
/// special case:
/// &str -> Str{ None }
/// &mut str -> Str{ mut_token: Some(mut) }
fn reference_tokens(arg: &FnArg) -> ReferenceTokens<'_> {
    match arg {
        FnArg::Typed(PatType {
            ty:
                box Type::Reference(TypeReference {
                    elem: box Type::Path(type_path),
                    and_token,
                    mutability,
                    ..
                }),
            ..
        }) => {
            if type_path.path.is_ident("str") {
                ReferenceTokens::Str {
                    mut_token: mutability.as_ref(),
                }
            } else {
                ReferenceTokens::NotStr {
                    and_token: Some(and_token),
                    mut_token: mutability.as_ref(),
                }
            }
        }

        _ => ReferenceTokens::NotStr {
            and_token: None,
            mut_token: None,
        },
    }
}

fn hash(input: &TokenStream) -> u64 {
    use std::collections::hash_map;
    use std::hash::Hasher;

    let mut hasher = hash_map::DefaultHasher::new();
    hasher.write(input.to_string().as_bytes());
    hasher.finish()
}
