use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::*;

pub fn derive_interactive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    interactive_impl(&ast).into()
}

pub fn derive_root(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    let struct_name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let interactive_impl = interactive_impl(&ast);

    let expanded = quote! {
        #interactive_impl

        impl #impl_generics InteractiveRoot for #struct_name #ty_generics #where_clause{}

        #[cfg(feature = "std")]
        impl #impl_generics ::rusteval::Methods for #struct_name #ty_generics #where_clause{
            fn eval_method(
                &self,
                function_name: &str,
                args: &str,
                f: &mut dyn ::core::ops::FnMut(::rusteval::Result<'_, &dyn ::core::fmt::Debug>),
            ) {
                if let ::core::option::Option::Some(function) = ::core::iter::Iterator::find(
                    &mut ::core::iter::IntoIterator::into_iter(
                        ::rusteval::inventory::iter::<&dyn ::rusteval::Function>,
                    ),
                    |function| function.function_name() == function_name,
                ) {
                    function.eval(args, f)
                } else {
                    f(::core::result::Result::Err(::rusteval::InteractiveError::FunctionNotFound {
                        function_name,
                    }))
                }
            }

            fn eval_method_mut(
                &mut self,
                function_name: &str,
                args: &str,
                f: &mut dyn ::core::ops::FnMut(::rusteval::Result<'_, &dyn ::core::fmt::Debug>),
            ) {
                (&*self).eval_method(function_name, args, f)
            }

            fn get_all_method_names(&self) -> &'static [&'static str]{
                ::lazy_static::lazy_static! {
                    static ref NAMES: ::std::vec::Vec<&'static str> = ::core::iter::Iterator::collect(::core::iter::Iterator::map(
                        ::core::iter::IntoIterator::into_iter(
                            ::rusteval::inventory::iter::<&dyn ::rusteval::Function>,
                        ),
                        |function| function.function_name(),
                    ));
                }
                &*NAMES
            }
        }
    };

    expanded.into()
}

fn interactive_impl(ast: &ItemStruct) -> TokenStream2 {
    let struct_name = &ast.ident;

    let tick_a = get_unused_lifetime(ast);

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let interactive_fields: Vec<_> = ast.fields.iter().collect();

    let eval_field_matches = interactive_fields.iter().enumerate().map(|(i, field)| {
        let name = get_name(field, i);

        quote! {
            stringify!(#name) => f(::rusteval::specialization::AsDebug::try_as_debug(&self.#name)),
        }
    });

    let get_field_matches = interactive_fields.iter().enumerate().map(|(i, field)| {
        let name = get_name(field, i);

        quote! {
            stringify!(#name) => ::rusteval::specialization::AsInteractive::try_as_interactive(&self.#name),
        }
    });

    let get_field_mut_matches = interactive_fields
        .iter()
        .filter(is_owned_or_mut_reference)
        .enumerate()
        .map(|(i, field)| {
            let name = get_name(field, i);

            quote! {
                stringify!(#name) => ::rusteval::specialization::AsInteractiveMut::try_as_interactive_mut(&mut self.#name),
            }
        });

    let all_field_names = interactive_fields.iter().enumerate().map(|(i, field)| {
        let name = get_name(field, i);
        quote! {
            stringify!(#name),
        }
    });

    // TODO shorten impl when default impl would work as_well

    quote! {
        impl #impl_generics ::rusteval::Interactive for #struct_name #ty_generics #where_clause {
            fn get_field<#tick_a>(&#tick_a self, field_name: &#tick_a str) -> ::rusteval::Result<'_, &dyn ::rusteval::Interactive>{
                match field_name {
                    #(#get_field_matches)*
                    _ => ::core::result::Result::Err(::rusteval::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name}),
                }
            }
            fn get_field_mut<#tick_a>(&#tick_a mut self, field_name: &#tick_a str) -> ::rusteval::Result<'_, &mut dyn ::rusteval::Interactive>{
                match field_name {
                    #(#get_field_mut_matches)*
                    _ => ::core::result::Result::Err(::rusteval::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name}),
                }
            }

            fn eval_field(&self, field_name: &str, f: &mut dyn ::core::ops::FnMut(::rusteval::Result<'_, &dyn ::core::fmt::Debug>))
            {
                match field_name {
                    #(#eval_field_matches)*
                    _ => f(::core::result::Result::Err(::rusteval::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name})),
                }
            }

            fn get_all_field_names(&self) -> &'static [&'static str]{
                &[#(#all_field_names)*]
            }
        }
    }
}

pub fn derive_partial_debug(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    let struct_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let interactive_fields = ast.fields.iter();

    let as_debug_all_fields = interactive_fields.enumerate().map(|(i, field)| {
        let name = get_name(field, i);
        quote! {
            .field(
                stringify!(#name),
                match ::rusteval::specialization::AsDebug::try_as_debug(&self.#name){
                    ::core::result::Result::Ok(field) => field,
                    ::core::result::Result::Err(_) => &::rusteval::specialization::Unknown,
                },
            )
        }
    });

    let expanded = quote! {
        impl #impl_generics ::core::fmt::Debug for #struct_name #ty_generics #where_clause{
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#struct_name))
                    #(#as_debug_all_fields)*
                    .finish()
    }
        }
    };

    expanded.into()
}

fn get_unused_lifetime(ast: &ItemStruct) -> Lifetime {
    let mut lifetime_name = "'rusteval".to_owned();

    for possible_lifetime in ('a'..='z').map(|char| char.to_string()) {
        if ast
            .generics
            .lifetimes()
            .any(|lt| lt.lifetime.ident == possible_lifetime)
        {
            continue;
        } else {
            lifetime_name = "'".to_owned() + &possible_lifetime;
            break;
        }
    }

    Lifetime::new(&lifetime_name, ast.span())
}

fn get_name(field: &Field, field_index: usize) -> TokenStream2 {
    if let Some(ident) = field.ident.as_ref() {
        quote! {#ident}
    } else {
        let i = Index::from(field_index);
        quote_spanned! {field.span()=> #i}
    }
}

fn is_owned_or_mut_reference(field: &&&Field) -> bool {
    !matches!(field.ty, Type::Reference(_))
        || matches!(
            field.ty,
            Type::Reference(TypeReference {
                mutability: Some(_),
                ..
            })
        )
}
