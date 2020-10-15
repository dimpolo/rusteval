#![feature(min_specialization)]

use core::fmt::Debug;
use minus_i::{Interactive, InteractiveRoot};
use std::rc::Rc;

#[derive(Interactive, Debug, Default)]
struct Inner {
    field: u32,
}

#[derive(Interactive, Debug, Default)]
struct Outer {
    inner: Inner,
}

#[derive(InteractiveRoot, Debug, Default)]
struct Root {
    outer_box: Box<Outer>,
    outer_rc: Rc<Outer>,
}

#[test]
fn test_box_eval() {
    let mut root = Root::default();

    assert_eq!(root.eval_to_string("outer_box.inner"), "Inner { field: 0 }")
}

#[test]
fn test_box_access() {
    let mut root = Root::default();

    assert_eq!(root.eval_to_string("outer_box.inner.field"), "0")
}

#[test]
fn test_rc_eval() {
    let mut root = Root::default();

    assert_eq!(root.eval_to_string("outer_rc.inner"), "Inner { field: 0 }")
}

#[test]
fn test_rc_access() {
    let mut root = Root::default();

    assert_eq!(root.eval_to_string("outer_rc.inner.field"), "0")
}
