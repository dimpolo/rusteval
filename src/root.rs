use core::fmt::Debug;

use crate::{Interactive, InteractiveError, Result};

enum AccessType<'a> {
    FieldAccess(&'a str),
    MethodAccess(&'a str, &'a str),
}

/// Docs and stuff TODO
pub trait InteractiveRoot: Interactive + Sized {
    /// Evaluates the given expression and calls the given closure with a [`Result`]`<&dyn `[`Debug`]`>`.
    /// # Example
    ///
    /// ```
    /// # #![feature(min_specialization)]
    /// # use minus_i::{Interactive, InteractiveMethods, InteractiveRoot, AsDebug};
    /// # use core::fmt::Debug;
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
    /// root.try_eval("child.add(1, 2)", &mut |result| assert_eq!(format!("{:?}", result), "Ok(3)"));
    /// root.try_eval("child.field1", &mut |result| assert_eq!(format!("{:?}", result), "Ok(false)"));
    /// ```
    fn try_eval(&mut self, expression: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>)) {
        match self.get_queried_object_mut(expression) {
            Ok((object, rest_expression)) => {
                let access_type = parse_access_type(rest_expression);
                match access_type {
                    Ok(AccessType::FieldAccess(field_name)) => {
                        (&*object).interactive_eval_field(field_name, f)
                    }
                    Ok(AccessType::MethodAccess(method_name, args)) => {
                        object.interactive_eval_method_mut(method_name, args, f)
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

    /// # Example
    ///
    /// ```
    /// # #![feature(min_specialization)]
    /// # use minus_i::{Interactive, InteractiveRoot, AsDebug};
    /// # use core::fmt::Debug;
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
    /// let (child, rest_expression) = root.get_queried_object("child.rest").unwrap();
    /// assert_eq!(child.get_all_interactive_field_names(), &["field1"]);
    /// assert_eq!(rest_expression, "rest");
    /// ```
    fn get_queried_object<'a>(
        &'a self,
        expression: &'a str,
    ) -> Result<'_, (&dyn Interactive, &str)> {
        let (mut object_path, rest_expression) = parse_object_path(expression);

        let mut current: &dyn Interactive = self;

        while !object_path.is_empty() {
            let (field_name, object_path_remainder) = object_path
                .rsplit_once('.')
                .unwrap_or((object_path.trim(), ""));
            object_path = object_path_remainder;

            current = current.interactive_get_field(field_name.trim())?
        }
        Ok((current, rest_expression))
    }

    /// Same as [`get_queried_object`] but returning a mutable reference.
    ///
    /// [`get_queried_object`]: ./trait.InteractiveRoot.html#method.get_queried_object
    fn get_queried_object_mut<'a>(
        &'a mut self,
        expression: &'a str,
    ) -> Result<'_, (&mut dyn Interactive, &str)> {
        let (mut object_path, rest_expression) = parse_object_path(expression);

        let mut current: &mut dyn Interactive = self;

        while !object_path.is_empty() {
            let (field_name, object_path_remainder) = object_path
                .rsplit_once('.')
                .unwrap_or((object_path.trim(), ""));
            object_path = object_path_remainder;

            current = current.interactive_get_field_mut(field_name.trim())?
        }
        Ok((current, rest_expression))
    }
     
    /// Docs and Stuff TODO
    fn eval_and_write<T>(&mut self, expression: &str, buf: &mut T) -> core::fmt::Result
    where
        T: core::fmt::Write,
    {
        let mut r = Ok(());
        self.try_eval(expression, &mut |result| r = write!(buf, "{:?}", result));
        r
    }

    #[cfg(feature = "std")]
    /// Docs and Stuff TODO
    fn eval_to_string(&mut self, expression: &str) -> String {
        let mut s = String::new();
        self.try_eval(expression, &mut |result| s = format!("{:?}", result));
        s
    }
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
