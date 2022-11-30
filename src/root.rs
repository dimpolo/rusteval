use core::fmt::Debug;

use crate::{Interactive, InteractiveError, Result};

enum AccessType<'a> {
    FieldAccess(&'a str),
    MethodAccess(&'a str, &'a str),
}

/// The main entry point to everything interactive.
///
/// The provided methods are not meant to be overridden.
/// This trait gets implemented automatically when you derive it with [`InteractiveRoot`].
///
/// It provides access to interactive fields, methods, and free functions by means of a string query.
///
/// [`InteractiveRoot`]: macro@crate::InteractiveRoot
///
/// A query looks just like normal Rust syntax. Possible queries are:
/// * `free_function()`
/// * `field_of_root`
/// * `field_of_root.child_field`
/// * `field_of_root.child_method()`
/// * etc.
///
/// Functions can be called with arguments just as you would in Rust:
/// * `takes_bool(true)`
/// * `takes_nums(1, 2)`
/// * `takes_char('C')`
/// * `takes_string_like("foo")`
///
/// Chars and string like types support escaping:
/// * `show_escaping('\x41', "\u{1f980} is \u{2764}")`
///
/// Currently supported argument types are:
///
/// `bool`, `char`, `f32`, `f64`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`,
/// `u64`, `u128`, `usize`, `String`, `str`
///
/// References to these types are also supported.
///
/// Generic argument types are not supported.
///
/// Both `String` and `str` are only available with default features on.
pub trait InteractiveRoot: Interactive + Sized {
    #[cfg(feature = "std")]
    /// Evaluates the query and returns the result as a String.
    /// Not available in no_std contexts.
    fn eval_to_string(&mut self, query: &str) -> String {
        let mut s = String::new();
        self.try_eval_mut(query, |result| {
            s = match result {
                Ok(r) => format!("{:?}", r),
                Err(e) => format!("{}", e),
            }
        });
        s
    }

    /// Evaluates the query and writes the result into the provided buffer.
    /// Useful in no_std contexts.
    fn eval_and_write<T>(&mut self, query: &str, buf: &mut T) -> core::fmt::Result
    where
        T: core::fmt::Write,
    {
        let mut r = Ok(());
        self.try_eval_mut(query, |result| {
            r = match result {
                Ok(r) => write!(buf, "{:?}", r),
                Err(e) => write!(buf, "{}", e),
            }
        });
        r
    }

    /// Evaluates the given query and calls the given closure with a [`Result`]`<&dyn `[`Debug`]`>`.
    ///
    /// This method does not have access to methods that take `&mut self` as their receiver,
    /// use [`try_eval_mut`] instead.
    ///
    /// [`try_eval_mut`]: #method.try_eval_mut
    ///
    /// # Example
    ///
    /// ```
    /// # use rusteval::{Interactive, Methods, InteractiveRoot};
    /// # use core::fmt::Debug;
    /// #
    /// #[derive(Interactive, Debug, Default)]
    /// struct Child {
    ///     field1: bool
    /// }
    /// #[Methods]
    /// impl Child {
    ///     fn add(&self, a: u8, b: u8) -> u8 {
    ///         a + b
    ///     }
    ///     fn toggle(&mut self){
    ///         self.field1 = !self.field1;
    ///     }
    /// }
    ///
    /// #[derive(InteractiveRoot, Debug, Default)]
    /// struct Root {
    ///     child: Child,
    /// }
    ///
    /// let mut root = Root::default();
    /// root.try_eval("child.add(1, 2)", |result| assert_eq!(format!("{:?}", result.unwrap()), "3"));
    /// root.try_eval("child.field1", |result| assert_eq!(format!("{:?}", result.unwrap()), "false"));
    /// root.try_eval("child.toggle()", |result| assert_eq!(format!("{}", result.unwrap_err()), "No method named `toggle` found for type `Child`"));
    /// ```
    fn try_eval<F>(&self, query: &str, mut f: F)
    where
        F: FnMut(Result<'_, &dyn Debug>),
    {
        match self.get_queried_object(query) {
            Ok((object, rest_expression)) => {
                let access_type = parse_access_type(rest_expression);
                match access_type {
                    Ok(AccessType::FieldAccess(field_name)) => {
                        object.eval_field(field_name, &mut f)
                    }
                    Ok(AccessType::MethodAccess(method_name, args)) => {
                        match object.try_as_methods() {
                            Ok(obj) => obj.eval_method(method_name, args, &mut f),
                            Err(e) => f(Err(e)),
                        }
                    }
                    Err(e) => f(Err(e)),
                }
            }
            Err(e) => f(Err(e)),
        }
    }

    /// Evaluates the given query and calls the given closure with a [`Result`]`<&dyn `[`Debug`]`>`.
    ///
    /// If mutability is required access will only succeed for owned fields or fields behind a `&mut`.
    /// # Example
    ///
    /// ```
    /// # use rusteval::{Interactive, Methods, InteractiveRoot};
    /// # use core::fmt::Debug;
    /// #
    /// #[derive(Interactive, Debug, Default)]
    /// struct Child {
    ///     field1: bool
    /// }
    /// #[Methods]
    /// impl Child {
    ///     fn toggle(&mut self){
    ///         self.field1 = !self.field1;
    ///     }
    /// }
    ///
    /// #[derive(InteractiveRoot, Debug)]
    /// struct Root<'a> {
    ///     owned: Child,
    ///     borrowed: &'a Child,
    /// }
    ///
    /// let mut root = Root{ owned: Child::default(), borrowed: &Child::default()};
    /// root.try_eval_mut("owned.toggle()", |result| assert!(result.is_ok()));
    /// root.try_eval_mut("owned.field1", |result| assert_eq!(format!("{:?}", result.unwrap()), "true"));
    /// root.try_eval_mut("borrowed.toggle()", |result| assert!(result.is_err()));
    /// root.try_eval_mut("borrowed.field1", |result| assert_eq!(format!("{:?}", result.unwrap()), "false"));
    /// ```
    fn try_eval_mut<F>(&mut self, query: &str, mut f: F)
    where
        F: FnMut(Result<'_, &dyn Debug>),
    {
        match self.get_queried_object_mut(query) {
            Ok((object, rest_expression)) => {
                let access_type = parse_access_type(rest_expression);
                match access_type {
                    Ok(AccessType::FieldAccess(field_name)) => {
                        object.eval_field(field_name, &mut f)
                    }
                    Ok(AccessType::MethodAccess(method_name, args)) => {
                        match object.try_as_methods_mut() {
                            Ok(obj) => obj.eval_method_mut(method_name, args, &mut f),
                            Err(e) => f(Err(e)),
                        }
                    }
                    Err(e) => f(Err(e)),
                }
            }
            Err(InteractiveError::FieldNotFound { .. }) => self.try_eval(query, f), // field might be behind shared reference
            Err(e) => f(Err(e)),
        }
    }

    /// Splits the given query into an object path and a rest expression.
    ///
    /// Then recursively looks for an object matching the given object path
    /// and if successful returns a shared reference to it together with the rest expression.
    ///
    /// The object path is the part of the given query before the last `.`
    ///
    /// E.g. `"path.to.obj.foo"` will split into the object path `"path.to.obj"` and the rest expression `"foo"`.
    ///

    /// # Example
    ///
    /// ```
    /// # use rusteval::{Interactive, InteractiveRoot};
    /// # use core::fmt::Debug;
    /// #
    /// #[derive(Interactive, Debug, Default)]
    /// struct Child {
    ///     field1: bool
    /// }
    ///
    /// #[derive(InteractiveRoot, Debug, Default)]
    /// struct Root {
    ///     child: Child,
    /// }
    ///
    /// let root = Root::default();
    /// let (child, rest_expression) = root.get_queried_object("child.rest").unwrap();
    /// assert_eq!(child.get_all_field_names(), &["field1"]);
    /// assert_eq!(rest_expression, "rest");
    /// ```
    fn get_queried_object<'a>(&'a self, query: &'a str) -> Result<'_, (&dyn Interactive, &str)> {
        let (mut object_path, rest_expression) = parse_object_path(query);

        let mut current: &dyn Interactive = self;

        while !object_path.is_empty() {
            let (field_name, object_path_remainder) = object_path
                .split_once('.')
                .unwrap_or((object_path.trim(), ""));
            object_path = object_path_remainder;

            current = current.get_field(field_name.trim())?
        }
        Ok((current, rest_expression))
    }

    /// Same as [`get_queried_object`] but returning a mutable reference.
    ///
    /// [`get_queried_object`]: #method.get_queried_object
    fn get_queried_object_mut<'a>(
        &'a mut self,
        query: &'a str,
    ) -> Result<'_, (&mut dyn Interactive, &str)> {
        let (mut object_path, rest_expression) = parse_object_path(query);

        let mut current: &mut dyn Interactive = self;

        while !object_path.is_empty() {
            let (field_name, object_path_remainder) = object_path
                .split_once('.')
                .unwrap_or((object_path.trim(), ""));
            object_path = object_path_remainder;

            current = current.get_field_mut(field_name.trim())?
        }
        Ok((current, rest_expression))
    }
}

fn parse_access_type(expression: &str) -> Result<'_, AccessType<'_>> {
    let expression = expression.trim();
    match expression.strip_suffix(')').map(|s| s.split_once('(')) {
        Some(Some((method_name, args))) => Ok(AccessType::MethodAccess(method_name.trim(), args)),
        Some(None) => Err(InteractiveError::SyntaxError), // closing parenthesis but no opening parenthesis
        None => Ok(AccessType::FieldAccess(expression)),
    }
}

/// splits query into object_path and rest_expression
fn parse_object_path(query: &str) -> (&str, &str) {
    let args_start_index = query.find('(').unwrap_or(query.len());
    match query[..args_start_index].rfind('.') {
        Some(last_dot_index) => {
            let (object_path, rest_expression) = query.split_at(last_dot_index);
            (object_path.trim(), rest_expression.get(1..).unwrap_or(""))
        }
        None => ("", query),
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
