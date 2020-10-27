use rusteval::{Interactive, InteractiveRoot, PartialDebug};

#[derive(Interactive, Default)]
struct NotDebug {
    field: u8,
}

#[derive(Interactive, PartialDebug, Default)]
struct Parent {
    not_debug: NotDebug,
}

#[derive(InteractiveRoot, Default)]
struct Root {
    parent: Parent,
}

#[test]
fn test_no_debug_field() {
    let mut root = Root::default();
    assert_eq!(root.eval_to_string("parent.not_debug.field"), "0");
}

#[test]
fn test_no_debug_access() {
    let mut root = Root::default();
    assert_eq!(
        root.eval_to_string("parent.not_debug"),
        "´partial_debug::NotDebug´ doesn't implement ´Debug´"
    );
}

#[test]
fn test_partial_debug() {
    let mut root = Root::default();
    assert_eq!(
        root.eval_to_string("parent"),
        "Parent { not_debug: Unknown }"
    );
}
