#![feature(min_specialization)]
#![feature(str_split_once)]

use repl::{Interactive, InteractiveMethods, Result};
use std::fmt::Debug;

#[derive(Interactive, Debug, Default)]
struct TestStruct {
    pub a: bool,
}

#[InteractiveMethods]
impl TestStruct {
    pub fn try_ping(&self) -> std::result::Result<String, ()> {
        Ok("pong".into())
    }

    pub fn answer(&self) {
        println!("42");
    }
}

#[derive(Interactive, Debug, Default)]
struct ParentStruct {
    pub child: TestStruct,
}

#[test]
fn test_get_root_object() {
    let mut repl = Repl::default();
    assert_eq!(
        repl.eval_to_debug_string("parent"),
        "Ok(ParentStruct { child: TestStruct { a: false } })"
    );
}

#[test]
fn test_get_child() {
    let mut repl = Repl::default();
    assert_eq!(
        repl.eval_to_debug_string("parent.child"),
        "Ok(TestStruct { a: false })"
    );
}

#[test]
fn test_get_child_field() {
    let mut repl = Repl::default();
    assert_eq!(repl.eval_to_debug_string("parent.child.a"), "Ok(false)");
}

#[test]
fn test_call_child_method() {
    let mut repl = Repl::default();
    assert_eq!(
        repl.eval_to_debug_string("parent.child.try_ping()"),
        "Ok(Ok(\"pong\"))"
    );
}

#[test]
fn test_call_with_float() {
    let mut repl = Repl::default();
    assert_eq!(
        repl.eval_to_debug_string("parent.child.add(4.20, 6.9)"),
        "Ok(false)"
    );
}

#[derive(Default)]
struct Repl {
    parent: ParentStruct,
}

impl Repl {
    fn eval_to_debug_string(&mut self, expression: &str) -> String {
        self.try_eval(expression, |result| format!("{:?}", result))
    }

    fn get_root_object<'a, F, R>(
        &mut self,
        object_name: &str,
    ) -> Result<&mut dyn Interactive<'a, F, R>> {
        match object_name {
            "parent" => Ok(&mut self.parent),
            _ => unimplemented!("{}", object_name),
        }
    }

    fn eval_root_object<F, R>(&self, object_name: &str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R,
    {
        match object_name {
            "parent" => f(Ok(&self.parent)),
            _ => unimplemented!("{}", object_name),
        }
    }

    fn try_eval<'a, F, R>(&'a mut self, expression: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R,
    {
        // split off the root object name
        if let Some((root_object_name, expression_remainder)) = expression.split_once('.') {
            // get the root object
            let root_object = match self.get_root_object(root_object_name.trim()) {
                Ok(root_object) => root_object,
                Err(e) => {
                    return f(Err(e));
                }
            };

            // split off field access or method call
            if let Some((object_path, command)) = expression_remainder.rsplit_once('.') {
                // walk the object_path to find the target object
                let mut current_object = root_object;
                for field_name in object_path.split('.') {
                    current_object = match current_object.__interactive_get_field(field_name.trim())
                    {
                        Ok(current_object) => current_object,
                        Err(e) => {
                            return f(Err(e));
                        }
                    };
                }

                call_or_get(current_object, command, f)
            } else {
                call_or_get(root_object, expression_remainder, f)
            }
        } else {
            // eval a root object
            self.eval_root_object(expression.trim(), f)
        }
    }
}

fn call_or_get<'a, F, R>(object: &'a mut dyn Interactive<'a, F, R>, command: &'a str, f: F) -> R
where
    F: Fn(Result<&dyn Debug>) -> R,
{
    if let Some(method_name) = command.trim().strip_suffix("()") {
        object.__interactive_eval_method(method_name.trim(), "", f)
    } else {
        (&*object).__interactive_eval_field(command.trim(), f)
    }
}
