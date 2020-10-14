#![feature(min_specialization)]

use core::fmt::Debug;
use minus_i::{Interactive, InteractiveRoot};

#[derive(Interactive, Debug, Default)]
struct Inner {
    field: u32,
}

#[derive(InteractiveRoot, Debug, Default)]
struct Root {
    inner: Box<Inner>,
}

#[test]
fn test_box_access() {
    let mut root = Root::default();

    assert_eq!(root.eval_to_string("inner.field"), "0")
}

impl ::minus_i::Fields for Box<Inner> {
    fn eval_field(
        &self,
        field_name: &str,
        f: &mut dyn FnMut(::minus_i::Result<'_, &dyn ::core::fmt::Debug>),
    ) {
        match field_name {
            "field" => f(::minus_i::as_debug::AsDebug::try_as_debug(&self.field)),
            _ => f(Err(::minus_i::InteractiveError::FieldNotFound {
                type_name: "Inner",
                field_name,
            })),
        }
    }
    fn get_all_field_names(&self) -> &'static [&'static str] {
        &["field"]
    }
}
