use rustdoc_types::{Crate, Function, Impl, Item, ItemEnum, Struct};

pub struct CrateWrapper {
    pub crate_: Crate,
}

impl CrateWrapper {
    pub fn new(crate_: Crate) -> Self {
        CrateWrapper { crate_ }
    }

    fn get_impls_for<'a>(&'a self, struct_: &'a Struct) -> impl Iterator<Item = FoundImpl<'a>> {
        struct_.impls.iter().filter_map(|impl_id| {
            let impl_ = self.crate_.index.get(impl_id).unwrap();
            match &impl_.inner {
                ItemEnum::Impl(inner) => Some(FoundImpl { impl_, inner }),
                _ => None,
            }
        })
    }

    fn get_methods_of<'a>(
        &'a self,
        impl_: FoundImpl<'a>,
    ) -> impl Iterator<Item = FoundFunction<'a>> {
        impl_.inner.items.iter().filter_map(move |fn_id| {
            let fn_ = self.crate_.index.get(fn_id).unwrap();
            match &fn_.inner {
                ItemEnum::Function(inner) => Some(FoundFunction { fn_, inner, impl_ }),
                _ => None,
            }
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FoundImpl<'a> {
    pub impl_: &'a Item,
    pub inner: &'a Impl,
}

#[derive(Copy, Clone, Debug)]
struct FoundFunction<'a> {
    pub fn_: &'a Item,
    pub inner: &'a Function,
    pub impl_: FoundImpl<'a>,
}
