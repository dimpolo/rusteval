#![feature(min_specialization)]

use minus_i::{InteractiveError, InteractiveMethods};

#[derive(Debug, Default)]
struct TestStruct {
    field: u32,
}

#[InteractiveMethods]
impl TestStruct {
    fn _private_method(&self) {}

    pub fn _new() -> Self {
        Self::default()
    }

    pub fn clone(&self) -> Self {
        Self { field: self.field }
    }

    pub fn get_field(&self) -> u32 {
        self.field
    }

    pub fn answer(&self) {
        println!("42");
    }
}

#[test]
fn test_call_no_args_primitive_return() {
    let mut test_struct = TestStruct::default();

    test_struct.interactive_eval_method("get_field", "", &mut |result| {
        assert_eq!(format!("{:?}", result.unwrap()), "0")
    });
}

#[test]
fn test_call_no_args_no_return() {
    let mut test_struct = TestStruct::default();

    test_struct.interactive_eval_method("answer", "", &mut |result| {
        assert_eq!(format!("{:?}", result.unwrap()), "()")
    });
}

#[test]
fn test_method_not_found() {
    use minus_i::InteractiveError;

    let mut test_struct = TestStruct::default();

    test_struct.interactive_eval_method("yeet", "", &mut |result| {
        assert_eq!(
            result.unwrap_err(),
            InteractiveError::MethodNotFound {
                type_name: "TestStruct",
                method_name: "yeet"
            }
        )
    });
}

#[test]
fn test_clone_method() {
    let mut test_struct = TestStruct::default();

    test_struct.interactive_eval_method("clone", "", &mut |result| {
        assert_eq!(format!("{:?}", result.unwrap()), "TestStruct { field: 0 }")
    });
}

#[test]
fn test_private_method() {
    let mut test_struct = TestStruct::default();

    test_struct.interactive_eval_method("_private_method", "", &mut |result| {
        assert_eq!(
            result.unwrap_err(),
            InteractiveError::MethodNotFound {
                type_name: "TestStruct",
                method_name: "_private_method"
            }
        )
    });
}

#[test]
fn test_associated_function() {
    let mut test_struct = TestStruct::default();

    test_struct.interactive_eval_method("_new", "", &mut |result| {
        assert_eq!(
            result.unwrap_err(),
            InteractiveError::MethodNotFound {
                type_name: "TestStruct",
                method_name: "_new"
            }
        )
    });
}

#[test]
fn test_too_many_args() {
    let mut test_struct = TestStruct::default();

    test_struct.interactive_eval_method("answer", "43", &mut |result| {
        assert_eq!(
            result.unwrap_err(),
            InteractiveError::WrongNumberOfArguments {
                expected: 0,
                found: 1
            }
        )
    });
}
