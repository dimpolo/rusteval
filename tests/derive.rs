#![feature(min_specialization)]

use repl::Interactive;
use std::fmt::Debug;

#[derive(Default, Debug)]
struct Inner(bool, Option<String>);

#[derive(Interactive, Default, Debug)]
struct TestStruct {
    pub field1: u32,
    pub field2: Inner,
    private_field: u32,
}

/* TODO
#[derive(Interactive, Default)]
struct TestStruct {}
*/

#[test]
fn test_primitive_field() {
    let test_struct = TestStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            test_struct.__interactive_get_field("field1").unwrap()
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
            test_struct.__interactive_get_field("field2").unwrap()
        ),
        "Inner(false, None)"
    );
}

#[test]
fn test_private_field() {
    // TODO custom private field error
    use repl::InteractiveError;

    let test_struct = TestStruct::default();

    assert_eq!(
        test_struct
            .__interactive_get_field("private_field")
            .unwrap_err(),
        InteractiveError::AttributeNotFound {
            struct_name: "TestStruct",
            field_name: "private_field"
        }
    );
}
