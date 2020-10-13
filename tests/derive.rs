#![feature(min_specialization)]

use core::fmt::Debug;
use minus_i::{Interactive, InteractiveFields};

#[derive(Default, Debug)]
struct Inner(bool, Option<String>);

#[derive(Interactive, Default, Debug)]
struct TestStruct {
    pub field1: u32,
    pub field2: Inner,
    private_field: u32,
}

#[derive(Interactive)]
struct RefStruct<'a> {
    pub test_struct_ref: &'a TestStruct,
}

#[derive(Interactive)]
struct RefMutStruct<'a> {
    pub test_struct_ref: &'a mut TestStruct,
}

#[test]
fn test_primitive_field() {
    let test_struct = TestStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            test_struct
                .interactive_get_field("field1")
                .unwrap()
                .try_as_debug()
                .unwrap()
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
                .interactive_get_field("field2")
                .unwrap()
                .try_as_debug()
                .unwrap()
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
            .interactive_get_field("private_field")
            .map(|_| ()) // unwrap_err requires that Ok value implements Debug
            .unwrap_err(),
        InteractiveError::FieldNotFound {
            type_name: "TestStruct",
            field_name: "private_field"
        }
    );
}

#[test]
fn test_references() {
    let test_struct = TestStruct::default();

    let ref_struct = RefStruct {
        test_struct_ref: &test_struct,
    };

    assert_eq!(
        format!(
            "{:?}",
            ref_struct
                .interactive_get_field("test_struct_ref")
                .unwrap()
                .try_as_debug()
                .unwrap()
        ),
        "TestStruct { field1: 0, field2: Inner(false, None), private_field: 0 }"
    );

    ref_struct.interactive_eval_field("test_struct_ref", &mut |field| {
        assert_eq!(
            format!("{:?}", field.unwrap()),
            "TestStruct { field1: 0, field2: Inner(false, None), private_field: 0 }"
        )
    });
}

#[test]
fn test_mut_references() {
    let mut test_struct = TestStruct::default();

    let mut ref_struct = RefMutStruct {
        test_struct_ref: &mut test_struct,
    };

    ref_struct.interactive_eval_field("test_struct_ref", &mut |field| {
        assert_eq!(
            format!("{:?}", field.unwrap()),
            "TestStruct { field1: 0, field2: Inner(false, None), private_field: 0 }"
        )
    });

    assert_eq!(
        format!(
            "{:?}",
            (&*ref_struct
                .interactive_get_field_mut("test_struct_ref")
                .unwrap())
                .try_as_debug()
                .unwrap()
        ),
        "TestStruct { field1: 0, field2: Inner(false, None), private_field: 0 }"
    );
}

#[test]
fn test_mut_references_as_shared_references() {
    let mut test_struct = TestStruct::default();

    let ref_struct = RefMutStruct {
        test_struct_ref: &mut test_struct,
    };

    assert_eq!(
        format!(
            "{:?}",
            ref_struct
                .interactive_get_field("test_struct_ref")
                .unwrap()
                .try_as_debug()
                .unwrap()
        ),
        "TestStruct { field1: 0, field2: Inner(false, None), private_field: 0 }"
    );

    ref_struct.interactive_eval_field("test_struct_ref", &mut |field| {
        assert_eq!(
            format!("{:?}", field.unwrap()),
            "TestStruct { field1: 0, field2: Inner(false, None), private_field: 0 }"
        )
    });
}

#[test]
fn test_shared_references_as_mut_references() {
    use minus_i::InteractiveError;

    let test_struct = TestStruct::default();

    let mut ref_struct = RefStruct {
        test_struct_ref: &test_struct,
    };

    // TODO custom mutability error

    assert_eq!(
        ref_struct
            .interactive_get_field_mut("test_struct_ref")
            .map(|_| ()) // unwrap_err requires that Ok value implements Debug
            .unwrap_err(),
        InteractiveError::FieldNotFound {
            type_name: "RefStruct",
            field_name: "test_struct_ref"
        }
    );
}
