use core::any::type_name;
use core::fmt::Debug;

use crate::as_debug::AsDebug;
use crate::{InteractiveError, Result};

/// The main trait of this crate TODO
///
/// # Note:
/// It is currently not possible to check if a trait is implemented at runtime.
/// This means that all members of an [`Interactive`] struct need to also implement [`Interactive`], which is why
/// a default blanket implementation for all `T` is provided.
///
pub trait Interactive: AsDebug + Methods + Fields {
    /// Looks for a field with the given name and on success return a shared reference to it.
    fn get_field<'a>(&'a self, field_name: &'a str) -> crate::Result<'a, &dyn crate::Interactive>;

    /// Looks for a field with the given name and on success return a mutable reference to it.
    ///
    /// # Note:
    /// Be careful when calling methods on the returned trait object that require only a shared reference.
    /// Since there is a default implementation for all T for those methods
    /// Rust will use the trait object as a `& &mut dyn Interactive`
    /// and you'll get the default implementation instead of the concrete one.
    ///
    /// See the below example on how to circumvent this.
    /// ```
    /// # #![feature(min_specialization)]
    /// # use minus_i::Interactive;
    /// #
    /// #[derive(Interactive, Default)]
    /// struct Struct {
    ///     field: OtherStruct,
    /// }
    ///
    /// #[derive(Interactive, Default)]
    /// struct OtherStruct {
    ///     other_field: u8,
    /// }
    ///
    /// let mut obj = Struct::default();
    ///
    /// assert!(obj
    ///     .get_field_mut("field")
    ///     .unwrap()
    ///     .get_field("other_field")
    ///     .is_err());
    ///
    /// assert!((&*obj.get_field_mut("field").unwrap())
    ///     .get_field("other_field")
    ///     .is_ok());
    /// ```
    fn get_field_mut<'a>(
        &'a mut self,
        field_name: &'a str,
    ) -> crate::Result<'a, &mut dyn crate::Interactive>;
}

impl<T> Interactive for T {
    default fn get_field<'a>(&'a self, field_name: &'a str) -> Result<'a, &dyn Interactive> {
        Err(InteractiveError::FieldNotFound {
            type_name: type_name::<T>(),
            field_name,
        })
    }

    default fn get_field_mut<'a>(
        &'a mut self,
        field_name: &'a str,
    ) -> Result<'a, &mut dyn Interactive> {
        Err(InteractiveError::FieldNotFound {
            type_name: type_name::<T>(),
            field_name,
        })
    }
}

/// A trait that allows to interactively evaluate a field and pass its value to the given closure.
///
/// This trait gets implemented automatically when you derive [`Interactive`].
///
/// # Note:
/// It is currently not possible to check if a trait is implemented at runtime.
/// This means that all members of an [`Interactive`] struct need to implement this trait, which is why
/// a default blanket implementation for all `T` is provided.
///
/// [`Interactive`]: ./derive.Interactive.html
pub trait Fields {
    /// Looks for a field with the given name,
    /// and passes it as a `Ok(&dyn Debug)` to the given closure.
    ///
    /// On error the an `Err(InteractiveError)` is passed to the closure instead.
    fn eval_field(&self, field_name: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>));

    /// Returns all interactive field names of this type.
    ///
    /// Can be used to drive auto-completion in a CLI.
    fn get_all_field_names(&self) -> &'static [&'static str];
}

impl<T> Fields for T {
    default fn eval_field(&self, field_name: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>)) {
        f(Err(InteractiveError::FieldNotFound {
            type_name: type_name::<T>(),
            field_name,
        }));
    }

    default fn get_all_field_names(&self) -> &'static [&'static str] {
        &[]
    }
}

/// A trait that allows to interactively evaluate a structs methods and pass their result to the given closure.
///
/// This trait gets implemented automatically when you use the [`Methods`] attribute.
///
/// # Note:
/// It is currently not possible to check if a trait is implemented at runtime.
/// This means that all members of an [`Interactive`] struct need to implement this trait, which is why
/// a default blanket implementation for all `T` is provided.
///
/// [`Interactive`]: ./derive.Interactive.html
/// [`Methods`]: ./attr.Methods.html
pub trait Methods {
    /// Looks for a method with the given name,
    /// parses the args string into the expected arguments of the method,
    /// executes the method and
    /// passes the result as a `Ok(&dyn Debug)` to the given closure.
    ///
    /// On error the an `Err(InteractiveError)` is passed to the closure instead.
    ///
    /// TODO explain difference
    fn eval_method(&self, method_name: &str, args: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>));

    /// Looks for a method with the given name,
    /// parses the args string into the expected arguments of the method,
    /// executes the method and
    /// passes the result as a `Ok(&dyn Debug)` to the given closure.
    ///
    /// On error the an `Err(InteractiveError)` is passed to the closure instead.
    fn eval_method_mut(
        &mut self,
        method_name: &str,
        args: &str,
        f: &mut dyn FnMut(Result<'_, &dyn Debug>),
    );

    /// Returns all interactive field names of this type.
    ///
    /// Can be used to drive auto-completion in a CLI.
    fn get_all_method_names(&self) -> &'static [&'static str];
}

impl<T> Methods for T {
    default fn eval_method(
        &self,
        method_name: &str,
        _args: &str,
        f: &mut dyn FnMut(Result<'_, &dyn Debug>),
    ) {
        f(Err(InteractiveError::MethodNotFound {
            type_name: type_name::<T>(),
            method_name,
        }));
    }

    default fn eval_method_mut(
        &mut self,
        method_name: &str,
        _args: &str,
        f: &mut dyn FnMut(Result<'_, &dyn Debug>),
    ) {
        f(Err(InteractiveError::MethodNotFound {
            type_name: type_name::<T>(),
            method_name,
        }));
    }

    default fn get_all_method_names(&self) -> &'static [&'static str] {
        &[]
    }
}
