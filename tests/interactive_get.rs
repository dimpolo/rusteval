#![feature(min_specialization)]

use repl::{Interactive, InteractiveMethods};

#[derive(Interactive, Debug, Default)]
struct TestStruct {
    pub a: bool,
}

#[InteractiveMethods]
impl TestStruct {
    pub fn try_ping(&self) -> Result<String, ()> {
        Ok("pong".into())
    }
}

#[derive(Debug, Default)]
struct ParentStruct {
    pub child: TestStruct,
}

impl<'a> repl::Interactive<'a> for ParentStruct {
    fn __interactive_get_field(
        &'a self,
        field_name: &'a str,
    ) -> repl::Result<'a, &dyn ::core::fmt::Debug> {
        match field_name {
            "child" => Ok(&self.child as &dyn ::core::fmt::Debug),
            _ => Err(repl::InteractiveError::AttributeNotFound {
                struct_name: "ParentStruct",
                field_name,
            }),
        }
    }
    fn __interactive_get_interactive_field(
        &'a mut self,
        field_name: &'a str,
    ) -> repl::Result<&'a dyn repl::Interactive> {
        match field_name {
            "child" => Ok(&mut self.child as &mut dyn repl::Interactive),
            _ => Err(repl::InteractiveError::AttributeNotFound {
                struct_name: "ParentStruct",
                field_name,
            }),
        }
    }
}

#[test]
fn test_get_child() {
    let parent_struct = ParentStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            parent_struct.__interactive_get_field("child").unwrap()
        ),
        "TestStruct { a: false }"
    );
}

#[test]
fn test_get_child_field() {
    let mut parent_struct = ParentStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            parent_struct
                .__interactive_get_interactive_field("child")
                .unwrap()
                .__interactive_get_field("a")
                .unwrap()
        ),
        "false"
    );
}

/*
#[test]
fn test_call_child_method() {
    let mut parent_struct = ParentStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            parent_struct
                .__interactive_get_interactive_field("child")
                .unwrap()
                .__interactive_call_method("try_ping", "")
                .unwrap()
        ),
        "false"
    );
}
*/
