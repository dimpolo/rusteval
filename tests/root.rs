#![feature(min_specialization)]
#![feature(str_split_once)]

use core::fmt::Debug;
use repl::{Interactive, InteractiveMethods, InteractiveRoot};

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
}

#[derive(Interactive, Debug, Default)]
struct ParentStruct {
    pub child: TestStruct,
}

#[derive(InteractiveRoot, Default, Debug)]
struct GenRepl {
    pub parent: ParentStruct,
}

#[test]
fn test_get_root_object() {
    let mut repl = GenRepl::default();
    assert_eq!(
        repl.eval_to_debug_string("parent"),
        "Ok(ParentStruct { child: TestStruct { a: false } })"
    );
}

#[test]
fn test_get_child() {
    let mut repl = GenRepl::default();
    assert_eq!(
        repl.eval_to_debug_string("parent.child"),
        "Ok(TestStruct { a: false })"
    );
}

#[test]
fn test_get_child_field() {
    let mut repl = GenRepl::default();
    assert_eq!(repl.eval_to_debug_string("parent.child.a"), "Ok(false)");
}

#[test]
fn test_call_child_method() {
    let mut repl = GenRepl::default();
    assert_eq!(
        repl.eval_to_debug_string("parent.child.try_ping()"),
        "Ok(Ok(\"pong\"))"
    );
}

#[test]
fn test_call_with_float() {
    let mut repl = GenRepl::default();
    assert_eq!(
        repl.eval_to_debug_string("parent.child.add(4.20, 6.9)"),
        "Ok(false)"
    );
}
