use core::fmt::Debug;
use rusteval::Interactive;

#[derive(Default, Debug)]
struct Inner(bool, Option<String>);

#[derive(Interactive, Default, Debug)]
struct TestStruct {
    field1: u32,
    field2: Inner,
}

#[derive(Interactive)]
struct RefStruct<'a> {
    test_struct_ref: &'a TestStruct,
}

#[derive(Interactive)]
struct RefMutStruct<'a> {
    test_struct_ref: &'a mut TestStruct,
}

#[derive(Interactive)]
struct DynRefStruct<'a> {
    test_struct_ref: &'a dyn Interactive,
}

#[derive(Interactive)]
struct DynRefMutStruct<'a> {
    test_struct_ref: &'a mut dyn Interactive,
}

#[test]
fn test_primitive_field() {
    let test_struct = TestStruct::default();

    test_struct.eval_field("field1", &mut |field| {
        assert_eq!(format!("{:?}", field.unwrap()), "0")
    });
}

#[test]
fn test_complex_field() {
    let test_struct = TestStruct::default();

    test_struct.eval_field("field2", &mut |field| {
        assert_eq!(format!("{:?}", field.unwrap()), "Inner(false, None)")
    });
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
                .get_field("test_struct_ref")
                .unwrap()
                .try_as_debug()
                .unwrap()
        ),
        "TestStruct { field1: 0, field2: Inner(false, None) }"
    );

    ref_struct.eval_field("test_struct_ref", &mut |field| {
        assert_eq!(
            format!("{:?}", field.unwrap()),
            "TestStruct { field1: 0, field2: Inner(false, None) }"
        )
    });
}

#[test]
fn test_mut_references() {
    let mut test_struct = TestStruct::default();

    let mut ref_struct = RefMutStruct {
        test_struct_ref: &mut test_struct,
    };

    ref_struct.eval_field("test_struct_ref", &mut |field| {
        assert_eq!(
            format!("{:?}", field.unwrap()),
            "TestStruct { field1: 0, field2: Inner(false, None) }"
        )
    });

    assert_eq!(
        format!(
            "{:?}",
            ref_struct
                .get_field_mut("test_struct_ref")
                .unwrap()
                .try_as_debug()
                .unwrap()
        ),
        "TestStruct { field1: 0, field2: Inner(false, None) }"
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
                .get_field("test_struct_ref")
                .unwrap()
                .try_as_debug()
                .unwrap()
        ),
        "TestStruct { field1: 0, field2: Inner(false, None) }"
    );

    ref_struct.eval_field("test_struct_ref", &mut |field| {
        assert_eq!(
            format!("{:?}", field.unwrap()),
            "TestStruct { field1: 0, field2: Inner(false, None) }"
        )
    });
}

#[test]
fn test_shared_references_as_mut_references() {
    use rusteval::InteractiveError;

    let test_struct = TestStruct::default();

    let mut ref_struct = RefStruct {
        test_struct_ref: &test_struct,
    };

    // TODO custom mutability error

    assert_eq!(
        ref_struct
            .get_field_mut("test_struct_ref")
            .map(|_| ()) // unwrap_err requires that Ok value implements Debug
            .unwrap_err(),
        InteractiveError::FieldNotFound {
            type_name: "RefStruct",
            field_name: "test_struct_ref"
        }
    );
}

#[test]
fn test_dyn_references() {
    let test_struct = TestStruct::default();

    let ref_struct = DynRefStruct {
        test_struct_ref: &test_struct,
    };

    assert_eq!(
        format!(
            "{:?}",
            ref_struct
                .get_field("test_struct_ref")
                .unwrap()
                .try_as_debug()
                .unwrap()
        ),
        "TestStruct { field1: 0, field2: Inner(false, None) }"
    );

    ref_struct.eval_field("test_struct_ref", &mut |field| {
        assert_eq!(
            format!("{:?}", field.unwrap()),
            "TestStruct { field1: 0, field2: Inner(false, None) }"
        )
    });
}

#[test]
fn test_dyn_mut_references() {
    let mut test_struct = TestStruct::default();

    let mut ref_struct = DynRefMutStruct {
        test_struct_ref: &mut test_struct,
    };

    ref_struct.eval_field("test_struct_ref", &mut |field| {
        assert_eq!(
            format!("{:?}", field.unwrap()),
            "TestStruct { field1: 0, field2: Inner(false, None) }"
        )
    });

    assert_eq!(
        format!(
            "{:?}",
            ref_struct
                .get_field_mut("test_struct_ref")
                .unwrap()
                .try_as_debug()
                .unwrap()
        ),
        "TestStruct { field1: 0, field2: Inner(false, None) }"
    );
}

#[test]
fn test_dyn_mut_references_as_shared_references() {
    let mut test_struct = TestStruct::default();

    let ref_struct = DynRefMutStruct {
        test_struct_ref: &mut test_struct,
    };

    assert_eq!(
        format!(
            "{:?}",
            ref_struct
                .get_field("test_struct_ref")
                .unwrap()
                .try_as_debug()
                .unwrap()
        ),
        "TestStruct { field1: 0, field2: Inner(false, None) }"
    );

    ref_struct.eval_field("test_struct_ref", &mut |field| {
        assert_eq!(
            format!("{:?}", field.unwrap()),
            "TestStruct { field1: 0, field2: Inner(false, None) }"
        )
    });
}

#[test]
fn test_dyn_shared_references_as_mut_references() {
    use rusteval::InteractiveError;

    let test_struct = TestStruct::default();

    let mut ref_struct = DynRefStruct {
        test_struct_ref: &test_struct,
    };

    // TODO custom mutability error

    assert_eq!(
        ref_struct
            .get_field_mut("test_struct_ref")
            .map(|_| ()) // unwrap_err requires that Ok value implements Debug
            .unwrap_err(),
        InteractiveError::FieldNotFound {
            type_name: "DynRefStruct",
            field_name: "test_struct_ref"
        }
    );
}

#[test]
fn test_tuple_struct() {
    #[derive(Interactive)]
    struct TupleStruct(u32, u32);

    let tuple_struct = TupleStruct(42, 43);

    assert_eq!(tuple_struct.get_all_field_names(), ["0", "1"]);
    tuple_struct.eval_field("1", &mut |field| {
        assert_eq!(format!("{:?}", field.unwrap()), "43")
    });
}
