use core::fmt::Debug;

use crate::{Interactive, InteractiveError, Result};

#[derive(Debug)]
enum AccessType<'a> {
    FieldAccess(&'a str),
    MethodAccess(&'a str, &'a str),
}

pub trait InteractiveRoot<'a, F: 'a, R: 'a>: Interactive<'a, F, R> + Sized {
    fn try_eval(&'a mut self, expression: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R,
    {
        match self.get_queried_object_mut(expression) {
            Ok((object, rest_expression)) => {
                let access_type = parse_access_type(rest_expression);
                match access_type {
                    Ok(AccessType::FieldAccess(field_name)) => {
                        (&*object).__interactive_eval_field(field_name, f)
                    }
                    Ok(AccessType::MethodAccess(method_name, args)) => {
                        object.__interactive_eval_method(method_name, args, f)
                    }
                    Err(e) => f(Err(e)),
                }
            }
            Err(e) => f(Err(e)),
        }
    }

    fn get_queried_object(
        &'a self,
        expression: &'a str,
    ) -> Result<(&dyn Interactive<'a, F, R>, &str)> {
        let (mut object_path, rest_expression) = parse_object_path(expression);

        let mut current = self as &dyn Interactive<'a, F, R>;

        while !object_path.is_empty() {
            let (field_name, object_path_remainder) = object_path
                .rsplit_once('.')
                .unwrap_or((object_path.trim(), ""));
            object_path = object_path_remainder;

            current = current.__interactive_get_field(field_name.trim())?
        }
        Ok((current, rest_expression))
    }

    fn get_queried_object_mut(
        &'a mut self,
        expression: &'a str,
    ) -> Result<(&mut dyn Interactive<'a, F, R>, &str)> {
        let (mut object_path, rest_expression) = parse_object_path(expression);

        let mut current = self as &mut dyn Interactive<'a, F, R>;

        while !object_path.is_empty() {
            let (field_name, object_path_remainder) = object_path
                .rsplit_once('.')
                .unwrap_or((object_path.trim(), ""));
            object_path = object_path_remainder;

            current = current.__interactive_get_field_mut(field_name.trim())?
        }
        Ok((current, rest_expression))
    }
    /* TODO make this work
    #[cfg(feature = "std")]
    fn eval_to_debug_string(&mut self, expression: &str) -> String {
        self.try_eval(expression, |result| format!("{:?}", result))
    }
    */
}

fn parse_access_type(expression: &str) -> Result<AccessType> {
    let expression = expression.trim();
    match expression.strip_suffix(')').map(|s| s.split_once('(')) {
        Some(Some((method_name, args))) => Ok(AccessType::MethodAccess(method_name.trim(), args)),
        Some(None) => Err(InteractiveError::SyntaxError),
        None => Ok(AccessType::FieldAccess(expression)),
    }
}

fn parse_object_path(expression: &str) -> (&str, &str) {
    let args_start_index = expression.find('(').unwrap_or_else(|| expression.len());
    match expression[..args_start_index].rfind('.') {
        Some(last_dot_index) => {
            let (object_path, rest_expression) = expression.split_at(last_dot_index);
            (object_path.trim(), &rest_expression.get(1..).unwrap_or(""))
        }
        None => ("", expression),
    }
}

#[test]
fn test_parse_object_path0() {
    assert_eq!(parse_object_path(""), ("", ""));
}
#[test]
fn test_parse_object_path1() {
    assert_eq!(parse_object_path("foo"), ("", "foo"));
}
#[test]
fn test_parse_object_path2() {
    assert_eq!(parse_object_path("foo."), ("foo", ""));
}
#[test]
fn test_parse_object_path3() {
    assert_eq!(parse_object_path("foo.bar"), ("foo", "bar"));
}
#[test]
fn test_parse_object_path4() {
    assert_eq!(parse_object_path("foo.frob(1.5)"), ("foo", "frob(1.5)"));
}
