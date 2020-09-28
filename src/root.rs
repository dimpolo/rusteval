use core::fmt::Debug;

use crate::{Interactive, Result};

#[derive(Debug)]
enum AccessType<'a> {
    FieldAccess(&'a str),
    MethodAccess(&'a str, &'a str),
}

pub trait InteractiveRoot<'a, F: 'a, R: 'a>: Interactive<'a, F, R> {
    fn try_eval(&'a mut self, expression: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R,
        Self: Sized,
    {
        let (mut object_path, access_type) = parse_expression(expression);

        dbg!(&object_path, &access_type);

        let mut current = self as &mut dyn Interactive<'a, F, R>;

        while !object_path.is_empty() {
            let (field_name, object_path_remainder) = object_path
                .rsplit_once('.')
                .unwrap_or((object_path.trim(), ""));
            object_path = object_path_remainder;

            current = match current.__interactive_get_field(field_name.trim()) {
                Ok(current) => current,
                Err(e) => {
                    return f(Err(e));
                }
            };
        }

        match access_type {
            AccessType::FieldAccess(field_name) => {
                (&*current).__interactive_eval_field(field_name, f)
            }
            AccessType::MethodAccess(method_name, args) => {
                current.__interactive_eval_method(method_name, args, f)
            }
        }
    }

    /* TODO make this work instead of inside proc macro
    #[cfg(feature = "std")]
    fn eval_to_debug_string(&mut self, expression: &str) -> String {
        self.try_eval(expression, |result| format!("{:?}", result))
    }
    */
}

fn parse_expression(expression: &str) -> (&str, AccessType) {
    let expression = expression.trim();

    if let Some(Some(((object_path, method_name), args))) = expression.strip_suffix(')').map(|s| {
        s.split_once('(')
            .map(|(path, args)| (path.rsplit_once('.').unwrap_or(("", path)), args))
    }) {
        (
            object_path.trim(),
            AccessType::MethodAccess(method_name.trim(), args),
        )
    } else {
        let (object_path, field_name) = expression.rsplit_once('.').unwrap_or(("", expression));
        (
            object_path.trim(),
            AccessType::FieldAccess(field_name.trim()),
        )
    }
}
