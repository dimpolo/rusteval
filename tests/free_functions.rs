#![feature(min_specialization)]

use minus_i::{InteractiveError, InteractiveFunction, InteractiveMethods, InteractiveRoot};

#[InteractiveFunction]
fn add_one(a: u32) -> u32 {
    a + 1
}

#[derive(InteractiveRoot)]
struct Root;

#[test]
fn test_free_function_names() {
    let root = Root;

    assert_eq!(root.get_all_interactive_method_names(), ["add_one"]);
}

#[test]
fn test_free_function_eval() {
    let mut root = Root;

    assert_eq!(root.eval_to_string("add_one(2)"), "3");
}

#[test]
fn test_free_function_not_there() {
    let mut root = Root;

    assert_eq!(
        root.eval_to_string("add_two(2)"),
        format!(
            "{}",
            InteractiveError::FunctionNotFound {
                function_name: "add_two"
            }
        )
    );
}
