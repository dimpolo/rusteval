#![feature(min_specialization)]
#![feature(str_split_once)]

use core::fmt::Debug;
use minus_i::{Interactive, InteractiveError, InteractiveMethods, InteractiveRoot};

#[derive(Interactive, Debug, Default)]
struct TestStruct {
    pub a: bool,
}

#[InteractiveMethods]
impl TestStruct {
    pub fn try_ping(&self) -> core::result::Result<String, ()> {
        Ok("pong".into())
    }

    pub fn answer(&self) {
        println!("42");
    }

    pub fn add(&self, a: f32, b: f32) -> f32 {
        a + b
    }

    pub fn frob(&self, a: usize, b: f32, c: i32) -> (usize, f32, i32) {
        (a, b, c)
    }

    pub fn toggle(&mut self) {
        self.a = !self.a;
    }
}

#[derive(Interactive, Debug, Default)]
struct ParentStruct {
    pub child: TestStruct,
}

#[derive(InteractiveRoot, Default, Debug)]
struct Root {
    pub parent: ParentStruct,
}

#[test]
fn test_get_root_object() {
    let mut root = Root::default();
    assert_eq!(
        root.eval_to_string("parent"),
        "ParentStruct { child: TestStruct { a: false } }"
    );
}

#[test]
fn test_get_child() {
    let mut root = Root::default();
    assert_eq!(
        root.eval_to_string("parent.child"),
        "TestStruct { a: false }"
    );
}

#[test]
fn test_get_child_field() {
    let mut root = Root::default();
    assert_eq!(root.eval_to_string("parent.child.a"), "false");
}

#[test]
fn test_call_child_method() {
    let mut root = Root::default();
    assert_eq!(
        root.eval_to_string("parent.child.try_ping()"),
        "Ok(\"pong\")"
    );
}

#[test]
fn test_call_with_float() {
    let mut root = Root::default();
    assert_eq!(root.eval_to_string("parent.child.add(4.20, 6.9)"), "11.1");
}

#[test]
fn test_call_with_different_arg_types() {
    let mut root = Root::default();
    assert_eq!(
        root.eval_to_string("parent.child.frob(420, 6.9, -7)"),
        "(420, 6.9, -7)"
    );
}

#[test]
fn test_call_with_bad_args() {
    let mut root = Root::default();
    assert_eq!(
        root.eval_to_string("parent.child.add(nope, 1)"),
        format!(
            "{}",
            InteractiveError::ArgsError {
                given_args: "nope, 1",
            }
        )
    );
}

#[test]
fn test_shared_reference_field() {
    #[derive(InteractiveRoot)]
    struct RefStruct<'a> {
        pub child: &'a TestStruct,
    }

    let child = TestStruct::default();
    let mut root = RefStruct { child: &child };
    assert_eq!(root.eval_to_string("child.a"), "false");
}

#[test]
fn test_shared_reference_method() {
    #[derive(InteractiveRoot)]
    struct RefStruct<'a> {
        pub child: &'a TestStruct,
    }

    let child = TestStruct::default();
    let mut root = RefStruct { child: &child };
    assert_eq!(root.eval_to_string("child.add(1, 2)"), "3.0");
}

#[test]
fn test_shared_reference_mut_method() {
    #[derive(InteractiveRoot)]
    struct RefStruct<'a> {
        pub child: &'a TestStruct,
    }

    let child = TestStruct::default();
    let mut root = RefStruct { child: &child };
    assert_eq!(
        root.eval_to_string("child.toggle()"),
        format!(
            "{}",
            InteractiveError::MethodNotFound {
                type_name: "TestStruct",
                method_name: "toggle"
            }
        )
    );
    // TODO custom mutability error
}

#[test]
fn test_shared_dyn_reference_field() {
    #[derive(InteractiveRoot)]
    struct RefStruct<'a> {
        pub child: &'a dyn Interactive,
    }

    let child = TestStruct::default();
    let mut root = RefStruct { child: &child };
    assert_eq!(root.eval_to_string("child.a"), "false");
}

#[test]
fn test_mut_dyn_reference_field() {
    #[derive(InteractiveRoot)]
    struct RefStruct<'a> {
        pub child: &'a mut dyn Interactive,
    }

    let mut child = TestStruct::default();
    let mut root = RefStruct { child: &mut child };
    assert_eq!(root.eval_to_string("child.a"), "false");
}
