#![feature(min_specialization)]

use core::fmt::Debug;
use minus_i::{AsDebug, Interactive};

#[derive(Default, Debug)]
struct Inner(bool, Option<String>);

#[derive(Interactive, Default, Debug)]
struct TestStruct {
    pub field1: u32,
    pub field2: Inner,
    private_field: u32,
}

#[test]
fn test_primitive_field() {
    let test_struct = TestStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            test_struct
                .__interactive_get_field("field1")
                .unwrap()
                .as_debug()
        ),
        "0"
    );
}

#[test]
fn test_complex_field() {
    let test_struct = TestStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            test_struct
                .__interactive_get_field("field2")
                .unwrap()
                .as_debug()
        ),
        "Inner(false, None)"
    );
}

#[test]
fn test_private_field() {
    // TODO custom private field error
    use minus_i::InteractiveError;

    let test_struct = TestStruct::default();

    assert_eq!(
        test_struct
            .__interactive_get_field("private_field")
            .map(|_| ()) // unwrap_err requires that Ok value implements Debug
            .unwrap_err(),
        InteractiveError::FieldNotFound {
            type_name: "TestStruct",
            field_name: "private_field"
        }
    );
}
