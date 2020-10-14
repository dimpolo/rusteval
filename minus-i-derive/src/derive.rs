use proc_macro::TokenStream;

use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::*;

pub fn derive_interactive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    interactive_impl(&ast).into()
}

pub fn derive_interactive_root(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    let struct_name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let interactive_impl = interactive_impl(&ast);

    let expanded = quote! {
        #interactive_impl

        impl #impl_generics InteractiveRoot for #struct_name #ty_generics #where_clause{}

        #[cfg(feature = "std")]
        impl #impl_generics ::minus_i::InteractiveMethods for #struct_name #ty_generics #where_clause{
            fn interactive_eval_method(
                &self,
                function_name: &str,
                args: &str,
                f: &mut dyn FnMut(::minus_i::Result<'_, &dyn ::core::fmt::Debug>),
            ) {
                if let Some(function) = ::minus_i::inventory::iter::<&dyn ::minus_i::InteractiveFunction>.into_iter()
                    .find(|function| function.function_name() == function_name)
                {
                    function.eval(args, f)
                } else {
                    f(Err(::minus_i::InteractiveError::FunctionNotFound {
                        function_name,
                    }))
                }
            }

            fn interactive_eval_method_mut(
                &mut self,
                function_name: &str,
                args: &str,
                f: &mut dyn FnMut(::minus_i::Result<'_, &dyn ::core::fmt::Debug>),
            ) {
                (&*self).interactive_eval_method(function_name, args, f)
            }

            fn get_all_interactive_method_names(&self) -> &'static [&'static str]{
                ::lazy_static::lazy_static! {
                    static ref NAMES: ::std::vec::Vec<&'static str> = ::minus_i::inventory::iter::<&dyn ::minus_i::InteractiveFunction>
                    .into_iter()
                    .map(|function| function.function_name())
                    .collect();
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

        if needs_dereference(&field) {
            quote! {
                stringify!(#name) => f(::minus_i::as_debug::AsDebug::try_as_debug(&*self.#name)),
            }
        } else {
            quote! {
                stringify!(#name) => f(::minus_i::as_debug::AsDebug::try_as_debug(&self.#name)),
            }
        }
    });

    let get_field_matches = interactive_fields.iter().enumerate().map(|(i, field)| {
        let name = get_name(field, i);
        if needs_dereference(&field) {
            quote! {
                stringify!(#name) => Ok(&*self.#name),
            }
        } else {
            quote! {
                stringify!(#name) => Ok(&self.#name),
            }
        }
    });

    let get_field_mut_matches = interactive_fields
        .iter()
        .filter(is_owned_or_mut_reference)
        .enumerate()
        .map(|(i, field)| {
            let name = get_name(field, i);

            if needs_dereference(&field) {
                quote! {
                    stringify!(#name) => Ok(&mut *self.#name),
                }
            } else {
                quote! {
                    stringify!(#name) => Ok(&mut self.#name),
                }
            }
        });

    let all_field_names = interactive_fields.iter().enumerate().map(|(i, field)| {
        let name = get_name(field, i);
        quote! {
            stringify!(#name),
        }
    });

    quote! {
        impl #impl_generics ::minus_i::Interactive for #struct_name #ty_generics #where_clause {
            fn interactive_get_field<#tick_a>(&#tick_a self, field_name: &#tick_a str) -> ::minus_i::Result<#tick_a, &dyn ::minus_i::Interactive>{
                match field_name {
                    #(#get_field_matches)*
                    _ => Err(::minus_i::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name}),
                }
            }
            fn interactive_get_field_mut<#tick_a>(&#tick_a mut self, field_name: &#tick_a str) -> ::minus_i::Result<#tick_a, &mut dyn ::minus_i::Interactive>{
                match field_name {
                    #(#get_field_mut_matches)*
                    _ => Err(::minus_i::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name}),
                }
            }

        }

        impl #impl_generics ::minus_i::InteractiveFields for #struct_name #ty_generics #where_clause{
            fn interactive_eval_field(&self, field_name: &str, f: &mut dyn FnMut(::minus_i::Result<'_, &dyn ::core::fmt::Debug>))
            {
                match field_name {
                    #(#eval_field_matches)*
                    _ => f(Err(::minus_i::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name})),
                }
            }

            fn get_all_interactive_field_names(&self) -> &'static [&'static str]{
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
                match ::minus_i::as_debug::AsDebug::try_as_debug(&self.#name){
                    Ok(field) => field,
                    Err(_) => &::minus_i::as_debug::Unknown,
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
    let mut lifetime_name = "'minus_i".to_owned();

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

    Lifetime::new(&lifetime_name, Span::call_site())
}

fn get_name(field: &Field, field_index: usize) -> TokenStream2 {
    if let Some(ident) = field.ident.as_ref() {
        quote! {#ident}
    } else {
        let i = Index::from(field_index);
        quote_spanned! {field.span()=> #i}
    }
}

fn needs_dereference(field: &Field) -> bool {
    // TODO check who needs this
    matches!(field.ty, Type::Reference(_))
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
