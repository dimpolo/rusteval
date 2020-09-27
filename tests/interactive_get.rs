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

#[derive(Interactive, Debug, Default)]
struct ParentStruct {
    pub child: TestStruct,
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

    let child = parent_struct
        .__interactive_get_interactive_field("child")
        .unwrap();

    assert_eq!(
        format!("{:?}", (&*child).__interactive_get_field("a").unwrap()),
        "false"
    );
}

#[test]
fn test_call_child_method() {
    let mut parent_struct = ParentStruct::default();

    let child = parent_struct
        .__interactive_get_interactive_field("child")
        .unwrap();

    assert_eq!(
        format!(
            "{:?}",
            child.__interactive_call_method("try_ping", "").unwrap()
        ),
        "Some(Ok(\"pong\"))"
    );
}
