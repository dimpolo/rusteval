pub mod codegen;
pub mod collect;
pub use rustdoc_json;

use rustdoc_types::{Crate, Function, Id, Impl, Item, ItemEnum, Struct, StructKind, Type};

#[derive(Debug)]
pub struct TypedItemRef<'a, T> {
    pub item: &'a Item,
    pub inner: &'a T,
}

impl<T> Clone for TypedItemRef<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for TypedItemRef<'_, T> {}

#[derive(Clone, Debug)]
pub struct CrateWrapper {
    pub crate_: Crate,
}

impl CrateWrapper {
    pub fn new(crate_: Crate) -> Self {
        CrateWrapper { crate_ }
    }

    pub fn get_item(&self, id: &Id) -> Option<&Item> {
        self.crate_.index.get(id)
    }

    pub fn get_struct(&self, name: &str) -> Option<&Struct> {
        self.crate_
            .index
            .iter()
            .find_map(|(_, item)| match &item.inner {
                ItemEnum::Struct(struct_) => {
                    if item.name.as_deref() == Some(name) {
                        Some(struct_)
                    } else {
                        None
                    }
                }
                _ => None,
            })
    }

    pub fn get_fields_of<'a>(&'a self, struct_: &'a Struct) -> Vec<TypedItemRef<'a, Type>> {
        match &struct_.kind {
            StructKind::Unit => Vec::new(),
            StructKind::Tuple(fields) => fields
                .iter()
                .filter_map(|field_id| {
                    if let Some(field_id) = field_id {
                        let field = self.get_item(field_id).unwrap();
                        match &field.inner {
                            ItemEnum::StructField(inner) => {
                                Some(TypedItemRef { item: field, inner })
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        None //
                    }
                })
                .collect(),
            StructKind::Plain { fields, .. } => fields
                .iter()
                .map(|field_id| {
                    let field = self.get_item(field_id).unwrap();
                    match &field.inner {
                        ItemEnum::StructField(inner) => TypedItemRef { item: field, inner },
                        _ => unreachable!(),
                    }
                })
                .collect(),
        }
    }

    pub fn get_impls_for<'a>(
        &'a self,
        struct_: &'a Struct,
    ) -> impl Iterator<Item = TypedItemRef<'a, Impl>> {
        struct_.impls.iter().map(|impl_id| {
            let impl_ = self.get_item(impl_id).unwrap();
            match &impl_.inner {
                ItemEnum::Impl(inner) => TypedItemRef { item: impl_, inner },
                _ => unreachable!(),
            }
        })
    }

    pub fn get_functions_in<'a>(
        &'a self,
        impl_: &'a Impl,
    ) -> impl Iterator<Item = TypedItemRef<'a, Function>> {
        impl_.items.iter().filter_map(move |fn_id| {
            let fn_ = self.get_item(fn_id).unwrap();
            match &fn_.inner {
                ItemEnum::Function(inner) => Some(TypedItemRef { item: fn_, inner }),
                _ => None, // skip other items, like associated types
            }
        })
    }
}
