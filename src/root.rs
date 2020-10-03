use core::fmt::Debug;

use crate::{Interactive, InteractiveError, Result};

enum AccessType<'a> {
    FieldAccess(&'a str),
    MethodAccess(&'a str, &'a str),
}

/// Docs and stuff TODO
pub trait InteractiveRoot<'a, F: 'a, R: 'a>: Interactive<'a, F, R> + Sized {
    /// Evaluates the given expression, calls the given closure with a [`Result`]`<&dyn `[`Debug`]`>` and returns what the closure returned.
    /// # Example
    ///
    /// ```
    /// # #![feature(min_specialization)]
    /// # use minus_i::{Interactive, InteractiveMethods, InteractiveRoot, AsDebug};
    /// # use std::fmt::Debug;
    /// #
    /// #[derive(Interactive, Debug, Default)]
    /// struct Child {
    ///     pub field1: bool
    /// }
    /// #[InteractiveMethods]
    /// impl Child {
    ///     pub fn add(&mut self, a: u8, b: u8) -> u8 {
    ///         a + b
    ///     }
    /// }
    ///
    /// #[derive(InteractiveRoot, Debug, Default)]
    /// struct Root {
    ///     pub child: Child,
    /// }
    ///
    /// let mut root = Root::default();
    /// assert_eq!(root.try_eval("child.add(1, 2)", |result| format!("{:?}", result)), "Ok(3)");
    /// assert_eq!(root.try_eval("child.field1", |result| format!("{:?}", result)), "Ok(false)");
    /// ```
    fn try_eval(&'a mut self, expression: &'a str, f: F) -> R
    where
        F: Fn(Result<'a, &dyn Debug>) -> R,
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

    /// Splits the given expression into an object path and a rest expression.
    /// The object path is the part of the given expression before the last `.`
    /// Then recursively looks for an object matching the given object path
    /// and if successful returns a shared reference to it together with the rest expression.
    ///
    /// # Note:
    /// Currently you might have to use the associated function syntax
    /// [`InteractiveRoot`]`::<(), ()>::`[`get_queried_object`]`(&instance, expression)`
    /// if rust complains about not being able to infer a type.
    ///
    /// [`get_queried_object`]: ./trait.InteractiveRoot.html#method.get_queried_object

    /// # Example
    ///
    /// ```
    /// # #![feature(min_specialization)]
    /// # use minus_i::{Interactive, InteractiveRoot, AsDebug};
    /// # use std::fmt::Debug;
    /// #
    /// #[derive(Interactive, Debug, Default)]
    /// struct Child {
    ///     pub field1: bool
    /// }
    ///
    /// #[derive(InteractiveRoot, Debug, Default)]
    /// struct Root {
    ///     pub child: Child,
    /// }
    ///
    /// let root = Root::default();
    /// let (child, rest_expression) = InteractiveRoot::<(), ()>::get_queried_object(&root, "child.rest").unwrap();
    /// assert_eq!(child.get_all_interactive_field_names(), &["field1"]);
    /// assert_eq!(rest_expression, "rest");
    /// ```
    fn get_queried_object(
        &'a self,
        expression: &'a str,
    ) -> Result<'a, (&dyn Interactive<'a, F, R>, &str)> {
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

    /// Same as [`get_queried_object`] but returning a mutable reference.
    ///
    /// [`get_queried_object`]: ./trait.InteractiveRoot.html#method.get_queried_object
    fn get_queried_object_mut(
        &'a mut self,
        expression: &'a str,
    ) -> Result<'a, (&mut dyn Interactive<'a, F, R>, &str)> {
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

fn parse_access_type(expression: &str) -> Result<'_, AccessType<'_>> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
