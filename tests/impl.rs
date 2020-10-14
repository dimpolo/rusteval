#![feature(min_specialization)]

use minus_i::{InteractiveError, Methods};

#[derive(Debug, Default)]
struct TestStruct {
    field: u32,
}

#[Methods]
impl TestStruct {
    fn _new() -> Self {
        Self::default()
    }

    fn clone(&self) -> Self {
        Self { field: self.field }
    }

    fn get_field(&self) -> u32 {
        self.field
    }

    fn answer(&self) {
        println!("42");
    }
}

#[test]
fn test_call_no_args_primitive_return() {
    let mut test_struct = TestStruct::default();

    test_struct.eval_method_mut("get_field", "", &mut |result| {
        assert_eq!(format!("{:?}", result.unwrap()), "0")
    });
}

#[test]
fn test_call_no_args_no_return() {
    let mut test_struct = TestStruct::default();

    test_struct.eval_method_mut("answer", "", &mut |result| {
        assert_eq!(format!("{:?}", result.unwrap()), "()")
    });
}

#[test]
fn test_method_not_found() {
    use minus_i::InteractiveError;

    let mut test_struct = TestStruct::default();

    test_struct.eval_method_mut("yeet", "", &mut |result| {
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

    test_struct.eval_method_mut("clone", "", &mut |result| {
        assert_eq!(format!("{:?}", result.unwrap()), "TestStruct { field: 0 }")
    });
}

#[test]
fn test_associated_function() {
    let mut test_struct = TestStruct::default();

    test_struct.eval_method_mut("_new", "", &mut |result| {
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

    test_struct.eval_method_mut("answer", "43", &mut |result| {
        assert_eq!(
            result.unwrap_err(),
            InteractiveError::WrongNumberOfArguments {
                method_name: "answer",
                expected: 0,
                found: 1
            }
        )
    });
}
