use proc_macro::TokenStream;

use quote::quote;
use syn::export::TokenStream2;
use syn::*;

pub fn derive_interactive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    interactive_impl(&ast).into()
}

pub fn derive_interactive_root(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    let struct_name = &ast.ident;

    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();

    let interactive_impl = interactive_impl(&ast);

    let expanded = quote! {
        #interactive_impl

        impl #impl_generics InteractiveRoot for #struct_name #ty_generics {}
    };

    expanded.into()
}

fn interactive_impl(ast: &ItemStruct) -> TokenStream2 {
    let struct_name = &ast.ident;

    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();

    let tick_a = quote! {'unused}; // TODO check that unused

    let get_interactive_fields = || ast.fields.iter().filter(is_interactive_field);

    let eval_field_matches = get_interactive_fields().map(|field| {
        let name = &field.ident;

        if is_reference(&field) {
            quote! {
                stringify!(#name) => f(Ok(&(&*self.#name).as_debug())),
            }
        } else {
            quote! {
                stringify!(#name) => f(Ok(&self.#name.as_debug())),
            }
        }
    });

    let get_field_matches = get_interactive_fields().map(|field| {
        let name = &field.ident;
        if is_reference(&field) {
            quote! {
                stringify!(#name) => Ok(&*self.#name),
            }
        } else {
            quote! {
                stringify!(#name) => Ok(&self.#name),
            }
        }
    });

    let get_field_mut_matches = get_interactive_fields()
        .filter(is_owned_or_mut_referece)
        .map(|field| {
            let name = &field.ident;

            if is_reference(&field) {
                quote! {
                    stringify!(#name) => Ok(&mut *self.#name),
                }
            } else {
                quote! {
                    stringify!(#name) => Ok(&mut self.#name),
                }
            }
        });

    let all_field_names = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name),
        }
    });

    quote! {
        impl #impl_generics minus_i::Interactive for #struct_name #ty_generics {
            fn __interactive_get_field<#tick_a>(&self, field_name: &#tick_a str) -> minus_i::Result<#tick_a, &dyn minus_i::Interactive>{
                match field_name {
                    #(#get_field_matches)*
                    _ => Err(minus_i::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name}),
                }
            }
            fn __interactive_get_field_mut<#tick_a>(&mut self, field_name: &#tick_a str) -> minus_i::Result<#tick_a, &mut dyn minus_i::Interactive>{
                match field_name {
                    #(#get_field_mut_matches)*
                    _ => Err(minus_i::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name}),
                }
            }

        }

        impl #impl_generics minus_i::InteractiveFields for #struct_name #ty_generics{
            fn __interactive_eval_field(&self, field_name: &str, f: &mut dyn FnMut(minus_i::Result<'_, &dyn ::core::fmt::Debug>))
            {
                match field_name {
                    #(#eval_field_matches)*
                    _ => f(Err(minus_i::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name})),
                }
            }
        }

        impl #impl_generics minus_i::InteractiveFieldNames for #struct_name #ty_generics{
            fn get_all_interactive_field_names(&self) -> &'static [&'static str]{
                &[#(#all_field_names)*]
            }
        }
    }
}

pub fn derive_partial_debug(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    let struct_name = &ast.ident;
    let (impl_generics, ty_generics, _where_clause) = ast.generics.split_for_impl();
    // TODO where clause, generics in InteractiveMethods

    let get_interactive_fields = || ast.fields.iter().filter(is_interactive_field);

    let as_debug_all_fields = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            .field(stringify!(#name), self.#name.as_debug())
        }
    });

    let expanded = quote! {
        impl #impl_generics ::core::fmt::Debug for #struct_name #ty_generics{
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#struct_name))
                    #(#as_debug_all_fields)*
                    .finish()
    }
        }
    };

    expanded.into()
}

fn is_interactive_field(field: &&Field) -> bool {
    matches!(field.vis, Visibility::Public(_))
}

fn is_reference(field: &Field) -> bool {
    matches!(field.ty, Type::Reference(_))
}

fn is_owned_or_mut_referece(field: &&Field) -> bool {
    !matches!(field.ty, Type::Reference(_))
        || matches!(
            field.ty,
            Type::Reference(TypeReference {
                mutability: Some(_),
                ..
            })
        )
}
