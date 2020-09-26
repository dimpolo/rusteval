use repl::{Interactive, InteractiveMethods};

#[derive(Debug, Default)]
struct TestStruct {}

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
        "TestStruct"
    );
}

#[test]
fn test_call_child_method() {
    let parent_struct = ParentStruct::default();

    assert_eq!(
        format!(
            "{:?}",
            parent_struct.__interactive_get_field("child").unwrap()
        ),
        "TestStruct"
    );
}
