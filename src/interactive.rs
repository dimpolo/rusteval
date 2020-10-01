use core::any::type_name;
use core::fmt::Debug;

/// The result type of most interactive methods
pub type Result<'a, T> = core::result::Result<T, InteractiveError<'a>>;

/// The main error type of this crate
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum InteractiveError<'a> {
    #[allow(missing_docs)]
    MethodNotFound {
        struct_name: &'a str,
        method_name: &'a str,
    },
    #[allow(missing_docs)]
    FieldNotFound {
        struct_name: &'a str,
        field_name: &'a str,
    },
    #[allow(missing_docs)]
    WrongNumberOfArguments { expected: usize, found: usize },
    #[allow(missing_docs)]
    ArgsError { given_args: &'a str },
    #[allow(missing_docs)]
    SyntaxError,
}

/// The main trait of this crate TODO
///
/// Note:
/// It is currently not possible to check if a trait is implemented at runtime.
/// This means that all members of an [`Interactive`] struct need to also implement [`Interactive`], which is why
/// a default blanket implementation for all `T: Debug` is provided.
///
pub trait Interactive<'a, F, R>:
    Debug
    + InteractiveMethods<'a, F, R>
    + InteractiveFields<'a, F, R>
    + InteractiveFieldNames
    + InteractiveMethodNames
{
    /// Looks for a field with the given name and on success return a shared reference to it.
    fn __interactive_get_field(
        &'a self,
        field_name: &'a str,
    ) -> crate::Result<'a, &dyn crate::Interactive<'a, F, R>>;

    /// Looks for a field with the given name and on success return a mutable reference to it.
    fn __interactive_get_field_mut(
        &'a mut self,
        field_name: &'a str,
    ) -> crate::Result<'a, &mut dyn crate::Interactive<'a, F, R>>;
}

impl<'a, F, R, T> Interactive<'a, F, R> for T
where
    T: Debug,
{
    default fn __interactive_get_field(
        &'a self,
        field_name: &'a str,
    ) -> Result<'a, &dyn Interactive<'a, F, R>> {
        Err(InteractiveError::FieldNotFound {
            struct_name: type_name::<T>(),
            field_name,
        })
    }

    default fn __interactive_get_field_mut(
        &'a mut self,
        field_name: &'a str,
    ) -> Result<'a, &mut dyn Interactive<'a, F, R>> {
        Err(InteractiveError::FieldNotFound {
            struct_name: type_name::<T>(),
            field_name,
        })
    }
}

/// A trait that allows to interactively evaluate a field and pass its value to a given closure.
///
/// This trait gets implemented automatically when you derive [`Interactive`].
///
/// Note:
/// It is currently not possible to check if a trait is implemented at runtime.
/// This means that all members of an [`Interactive`] struct need to implement this trait, which is why
/// a default blanket implementation for all `T: Debug` is provided.
///
/// [`Interactive`]: ./derive.Interactive.html
pub trait InteractiveFields<'a, F, R>: Debug {
    /// Looks for a field with the given name,
    /// and passes it as a `Ok(&dyn Debug)` to the given closure.
    ///
    /// On error the an `Err(InteractiveError)` is passed to the closure instead.
    ///
    /// The return value of the closure is also returned by this method.
    fn __interactive_eval_field(&'a self, field_name: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R;
}

impl<'a, F, R, T> InteractiveFields<'a, F, R> for T
where
    T: Debug,
{
    default fn __interactive_eval_field(&'a self, field_name: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R,
    {
        f(Err(InteractiveError::FieldNotFound {
            struct_name: type_name::<T>(),
            field_name,
        }))
    }
}

/// A trait that allows to interactively evaluate a function and pass its result to a given closure.
///
/// This trait gets implemented automatically when you use the [`InteractiveMethods`] attribute.
///
/// Note:
/// It is currently not possible to check if a trait is implemented at runtime.
/// This means that all members of an [`Interactive`] struct need to implement this trait, which is why
/// a default blanket implementation for all `T: Debug` is provided.
///
/// [`Interactive`]: ./derive.Interactive.html
/// [`InteractiveMethods`]: ./attr.InteractiveMethods.html
pub trait InteractiveMethods<'a, F, R>: Debug {
    /// Looks for a method with the given name,
    /// parses the args string into the expected arguments of the method,
    /// executes the method and
    /// passes the result as a `Ok(&dyn Debug)` to the given closure.
    ///
    /// On error the an `Err(InteractiveError)` is passed to the closure instead.
    ///
    /// The return value of the closure is also returned by this method.
    fn __interactive_eval_method(&'a mut self, method_name: &'a str, args: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R;
}

impl<'a, F, R, T> InteractiveMethods<'a, F, R> for T
where
    T: Debug,
{
    default fn __interactive_eval_method(
        &'a mut self,
        method_name: &'a str,
        _args: &'a str,
        f: F,
    ) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R,
    {
        f(Err(InteractiveError::MethodNotFound {
            struct_name: type_name::<T>(),
            method_name,
        }))
    }
}

/// A trait that allows a CLI to query all interactive field names.
///
/// This trait gets implemented automatically when you derive [`Interactive`].
///
/// Note:
/// It is currently not possible to check if a trait is implemented at runtime.
/// This means that all members of an [`Interactive`] struct need to implement this trait, which is why
/// a default blanket implementation for all `T: Debug` is provided.
///
/// [`Interactive`]: ./derive.Interactive.html
pub trait InteractiveFieldNames: Debug {
    /// Returns all interactive field names of this type.
    fn get_all_interactive_field_names(&self) -> &'static [&'static str];
}

impl<T> InteractiveFieldNames for T
where
    T: Debug,
{
    default fn get_all_interactive_field_names(&self) -> &'static [&'static str] {
        &[]
    }
}

/// A trait that allows a CLI to query all interactive method names.
///
/// This trait gets implemented automatically when you use the [`InteractiveMethods`] attribute.
///
/// Note:
/// It is currently not possible to check if a trait is implemented at runtime.
/// This means that all members of an [`Interactive`] struct need to implement this trait, which is why
/// a default blanket implementation for all `T: Debug` is provided.
///
/// [`Interactive`]: ./derive.Interactive.html
/// [`InteractiveMethods`]: ./attr.InteractiveMethods.html
pub trait InteractiveMethodNames: Debug {
    /// Returns all interactive field names of this type.
    fn get_all_interactive_method_names(&self) -> &'static [&'static str];
}

impl<T> InteractiveMethodNames for T
where
    T: Debug,
{
    default fn get_all_interactive_method_names(&self) -> &'static [&'static str] {
        &[]
    }
}
