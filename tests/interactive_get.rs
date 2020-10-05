#![feature(min_specialization)]

use minus_i::{AsDebug, Interactive, InteractiveFields, InteractiveMethods};

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

#[derive(Interactive, Debug, Default)]
struct ParentStruct {
    pub child: TestStruct,
}

#[test]
fn test_get_child() {
    let parent_struct = ParentStruct::default();

    parent_struct.interactive_eval_field("child", &mut |result| {
        assert_eq!(format!("{:?}", result.unwrap()), "TestStruct { a: false }")
    });
}

#[test]
fn test_get_child_field() {
    let parent_struct = ParentStruct::default();

    let child = parent_struct.interactive_get_field("child").unwrap();

    (&*child).interactive_eval_field("a", &mut |result| {
        assert_eq!(format!("{:?}", result.unwrap()), "false")
    });
}

#[test]
fn test_call_child_method() {
    let mut parent_struct = ParentStruct::default();

    let child = parent_struct.interactive_get_field_mut("child").unwrap();

    child.interactive_eval_method_mut("try_ping", "", &mut |result| {
        assert_eq!(format!("{:?}", result.unwrap()), "Ok(\"pong\")")
    });
}
