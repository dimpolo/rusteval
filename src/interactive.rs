use core::any::type_name;
use core::fmt::Debug;

use auto_impl::auto_impl;

use crate::specialization::{AsDebug, AsMethods, AsMethodsMut};
use crate::{InteractiveError, Result};

/// A trait that gives interactive access to its fields either as `dyn Interactive` or `dyn Debug`.
///
/// This trait gets implemented automatically when you use derive it with [`Interactive`].
/// See the macros documentation for more information.
///
/// [`Interactive`]: macro@crate::Interactive
#[cfg_attr(feature = "std", auto_impl(&, &mut, Box, Rc, Arc))]
#[cfg_attr(not(feature = "std"), auto_impl(&, &mut))]
pub trait Interactive: AsDebug + AsMethods + AsMethodsMut {
    /// Looks for a field with the given name and on success return a shared reference to it.
    fn get_field<'a>(&'a self, field_name: &'a str) -> crate::Result<'a, &dyn crate::Interactive> {
        Err(InteractiveError::FieldNotFound {
            type_name: type_name::<Self>(),
            field_name,
        })
    }

    /// Looks for a field with the given name and on success return a mutable reference to it.
    #[auto_impl(keep_default_for(&, Rc, Arc))]
    fn get_field_mut<'a>(
        &'a mut self,
        field_name: &'a str,
    ) -> crate::Result<'a, &mut dyn crate::Interactive> {
        Err(InteractiveError::FieldNotFound {
            type_name: type_name::<Self>(),
            field_name,
        })
    }

    /// Looks for a field with the given name,
    /// and passes it as a `Ok(&dyn Debug)` to the given closure.
    ///
    /// On error the `Err(InteractiveError)` is passed to the closure instead.
    fn eval_field(&self, field_name: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>)) {
        f(Err(InteractiveError::FieldNotFound {
            type_name: type_name::<Self>(),
            field_name,
        }))
    }

    /// Returns all interactive field names of this type.
    ///
    /// Can be used to drive auto-completion in a CLI.
    fn get_all_field_names(&self) -> &'static [&'static str] {
        &[]
    }
}

/// A trait that allows to interactively evaluate a structs methods and pass their result to the given closure.
///
/// This trait gets implemented automatically when you use the [`Methods`] attribute.
/// See its documentation for more information.
///
/// [`Interactive`]: macro@crate::Interactive
/// [`Methods`]: macro@crate::Methods
#[cfg_attr(feature = "std", auto_impl(&, &mut, Box, Rc, Arc))]
#[cfg_attr(not(feature = "std"), auto_impl(&, &mut))]
pub trait Methods {
    /// Looks for a method with the given name,
    /// parses the args string into the expected arguments of the method,
    /// executes the method and
    /// passes the result as a `Ok(&dyn Debug)` to the given closure.
    ///
    /// On error the `Err(InteractiveError)` is passed to the closure instead.
    ///
    /// This method does not have access to methods that take `&mut self` as their receiver,
    /// use [`eval_method_mut`] instead.
    ///
    /// [`eval_method_mut`]: #method.eval_method_mut
    fn eval_method(
        &self,
        method_name: &str,
        args: &str,
        f: &mut dyn FnMut(Result<'_, &dyn Debug>),
    ) {
        {
            let _ = args;
            f(Err(InteractiveError::MethodNotFound {
                type_name: type_name::<Self>(),
                method_name,
            }));
        }
    }

    /// Looks for a method with the given name,
    /// parses the args string into the expected arguments of the method,
    /// executes the method and
    /// passes the result as a `Ok(&dyn Debug)` to the given closure.
    ///
    /// On error the `Err(InteractiveError)` is passed to the closure instead.
    #[auto_impl(keep_default_for(&, Rc, Arc))]
    fn eval_method_mut(
        &mut self,
        method_name: &str,
        args: &str,
        f: &mut dyn FnMut(Result<'_, &dyn Debug>),
    ) {
        let _ = args;
        f(Err(InteractiveError::MethodNotFound {
            type_name: type_name::<Self>(),
            method_name,
        }));
    }

    /// Returns all interactive method names of this type.
    ///
    /// Can be used to drive auto-completion in a CLI.
    fn get_all_method_names(&self) -> &'static [&'static str] {
        &[]
    }
}
