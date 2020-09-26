use repl::InteractiveMethods;
use repl_derive::Interactive;

#[derive(Interactive, Debug, Default)]
struct TestStruct {
    field: u32,
}

impl TestStruct {
    pub fn get_field(&self) -> u32 {
        self.field
    }

    pub fn answer(&self) {
        println!("42");
    }
}

impl<'a> InteractiveMethods<'a> for TestStruct {
    fn __interactive_call_method(
        &'a mut self,
        method_name: &'a str,
        _args: &'a str,
    ) -> repl::Result<'a, Option<Box<dyn core::fmt::Debug>>> {
        match method_name {
            "get_field" => Ok(Some(Box::new(self.get_field()) as Box<dyn core::fmt::Debug>)),
            "answer" => Ok({
                self.answer();
                None
            }),

            _ => Err(repl::InteractiveError::MethodNotFound {
                struct_name: stringify!(TestStruct),
                method_name,
            }),
        }
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
