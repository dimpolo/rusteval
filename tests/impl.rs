#![feature(min_specialization)]

use repl::InteractiveMethods;

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

    assert_eq!(
        format!(
            "{:?}",
            test_struct
                .__interactive_call_method("get_field", "")
                .unwrap()
        ),
        "Some(0)"
    );
}

#[test]
fn test_call_no_args_no_return() {
    let mut test_struct = TestStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            test_struct.__interactive_call_method("answer", "").unwrap()
        ),
        "None"
    );
}

#[test]
fn test_method_not_found() {
    use repl::InteractiveError;

    let mut test_struct = TestStruct::default();

    assert_eq!(
        test_struct
            .__interactive_call_method("yeet", "")
            .unwrap_err(),
        InteractiveError::MethodNotFound {
            struct_name: "TestStruct",
            method_name: "yeet"
        }
    );
}

#[test]
fn test_clone_method() {
    let mut test_struct = TestStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            test_struct.__interactive_call_method("clone", "").unwrap()
        ),
        "Some(TestStruct { field: 0 })"
    );
}

#[test]
fn test_private_method() {
    let mut test_struct = TestStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            test_struct
                .__interactive_call_method("_private_method", "")
                .unwrap_err()
        ),
        "MethodNotFound { struct_name: \"TestStruct\", method_name: \"_private_method\" }"
    );
}

#[test]
fn test_associated_function() {
    let mut test_struct = TestStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            test_struct
                .__interactive_call_method("_new", "")
                .unwrap_err()
        ),
        "MethodNotFound { struct_name: \"TestStruct\", method_name: \"_new\" }"
    );
}

#[test]
fn test_too_many_args() {
    let mut test_struct = TestStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            test_struct
                .__interactive_call_method("answer", "43")
                .unwrap_err()
        ),
        "WrongNumberOfArguments { expected: 0, found: 1 }"
    );
}
