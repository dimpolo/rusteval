use core::fmt::Debug;

use crate::{Interactive, Result};

pub trait InteractiveRoot<'a, F: 'a, R: 'a>: Interactive<'a, F, R> {
    fn try_eval(&'a mut self, expression: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R,
    {
        // split off the root object name
        if let Some((root_object_name, expression_remainder)) = expression.split_once('.') {
            // get the root object
            let root_object = match self.__interactive_get_field(root_object_name.trim()) {
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

                get_or_call(current_object, command, f)
            } else {
                get_or_call(root_object, expression_remainder, f)
            }
        } else {
            // eval a root object
            (&*self).__interactive_eval_field(expression.trim(), f)
        }
    }

    /* TODO make this work instead of inside proc macro
    #[cfg(feature = "std")]
    fn eval_to_debug_string(&mut self, expression: &str) -> String {
        self.try_eval(expression, |result| format!("{:?}", result))
    }
    */
}

fn get_or_call<'a, F, R>(object: &'a mut dyn Interactive<'a, F, R>, command: &'a str, f: F) -> R
where
    F: Fn(Result<&dyn Debug>) -> R,
{
    if let Some(method_name) = command.trim().strip_suffix("()") {
        object.__interactive_eval_method(method_name.trim(), "", f)
    } else {
        (&*object).__interactive_eval_field(command.trim(), f)
    }
}
