use proc_macro::TokenStream;

use quote::quote;
use syn::export::{Span, TokenStream2};
use syn::*;

pub fn derive_interactive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    interactive_impl(&ast).into()
}

pub fn derive_interactive_root(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    let struct_name = &ast.ident;

    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();
    let mut new_generics = ast.generics.clone();
    add_new_impl_with_extra_lifetime_bounds(&mut new_generics);
    let (new_impl_generics, _, _) = new_generics.split_for_impl();

    let interactive_impl = interactive_impl(&ast);

    let expanded = quote! {
        #interactive_impl

        impl #new_impl_generics InteractiveRoot<'a, F, R> for #struct_name #ty_generics {}

        #[cfg(feature = "std")]
        impl #impl_generics #struct_name #ty_generics {
            fn eval_to_debug_string(&mut self, expression: &str) -> String {
                self.try_eval(expression, |result| format!("{:?}", result))
            }
        }
    };

    expanded.into()
}

fn interactive_impl(ast: &ItemStruct) -> TokenStream2 {
    let struct_name = &ast.ident;

    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();
    let mut new_generics = ast.generics.clone();
    add_new_impl_generics(&mut new_generics);
    let (new_impl_generics, _, _) = new_generics.split_for_impl();

    let get_interactive_fields = || ast.fields.iter().filter(is_interactive_field);

    let eval_field_matches = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name) => f(Ok(&self.#name.as_debug())),
        }
    });

    let get_field_matches = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name) => Ok(&self.#name),
        }
    });

    let get_field_mut_matches = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name) => Ok(&mut self.#name),
        }
    });

    let all_field_names = get_interactive_fields().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name),
        }
    });

    quote! {
        impl #new_impl_generics minus_i::Interactive<'a, F, R> for #struct_name #ty_generics {
            fn __interactive_get_field(&'a self, field_name: &'a str) -> minus_i::Result<'a, &dyn minus_i::Interactive<'a, F, R>>{
                match field_name {
                    #(#get_field_matches)*
                    _ => Err(minus_i::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name}),
                }
            }
            fn __interactive_get_field_mut(&'a mut self, field_name: &'a str) -> minus_i::Result<'a, &mut dyn minus_i::Interactive<'a, F, R>>{
                match field_name {
                    #(#get_field_mut_matches)*
                    _ => Err(minus_i::InteractiveError::FieldNotFound{type_name: stringify!(#struct_name), field_name}),
                }
            }

        }

        impl #new_impl_generics minus_i::InteractiveFields<'a, F, R> for #struct_name #ty_generics{
            fn __interactive_eval_field(&'a self, field_name: &'a str, f: F) -> R
            where
                F: Fn(minus_i::Result<'a, &dyn ::core::fmt::Debug>) -> R,
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

fn add_new_impl_generics(generics: &mut syn::Generics) {
    // TODO check for collisions and choose not colliding type params
    let lifetime = Lifetime::new("'a", Span::call_site());
    let f_type_param = TypeParam::from(Ident::new("F", Span::call_site()));
    let r_type_param = TypeParam::from(Ident::new("R", Span::call_site()));

    generics
        .params
        .push(GenericParam::Lifetime(LifetimeDef::new(lifetime)));
    generics.params.push(GenericParam::Type(f_type_param));
    generics.params.push(GenericParam::Type(r_type_param));
}

fn add_new_impl_with_extra_lifetime_bounds(generics: &mut syn::Generics) {
    // TODO check for collisions and choose not colliding type params
    let lifetime = Lifetime::new("'a", Span::call_site());

    let mut f_type_param = TypeParam::from(Ident::new("F", Span::call_site()));
    f_type_param
        .bounds
        .push(TypeParamBound::Lifetime(lifetime.clone()));

    let mut r_type_param = TypeParam::from(Ident::new("R", Span::call_site()));
    r_type_param
        .bounds
        .push(TypeParamBound::Lifetime(lifetime.clone()));

    generics
        .params
        .push(GenericParam::Lifetime(LifetimeDef::new(lifetime)));
    generics.params.push(GenericParam::Type(f_type_param));
    generics.params.push(GenericParam::Type(r_type_param));
}
