use crate::{Interactive, Result};
use core::fmt::Debug;

pub trait Repl {
    fn get_root_object<'a, F, R>(
        &mut self,
        object_name: &'a str,
    ) -> Result<&mut dyn Interactive<'a, F, R>>;

    fn eval_root_object<'a, F, R>(&'a self, object_name: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R;

    fn eval_to_debug_string(&mut self, expression: &str) -> String {
        self.try_eval(expression, |result| format!("{:?}", result))
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
