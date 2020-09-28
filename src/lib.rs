#![feature(min_specialization)]
#![feature(str_split_once)]

mod repl;

pub use repl::Repl;
pub use repl_derive::{repl, Interactive, InteractiveMethods};

use core::any::type_name;
use core::fmt::Debug;

pub type Result<'a, T> = core::result::Result<T, InteractiveError<'a>>;
pub type InteractiveClosure<'a, T> = core::result::Result<T, InteractiveError<'a>>;

#[derive(Debug, PartialEq, Eq)]
pub enum InteractiveError<'a> {
    MethodNotFound {
        struct_name: &'a str,
        method_name: &'a str,
    },
    FieldNotFound {
        struct_name: &'a str,
        field_name: &'a str,
    },
    InstanceNotFound {
        instance_name: &'a str,
    },
    WrongNumberOfArguments {
        expected: usize,
        found: usize,
    },
    SyntaxError,
}

pub trait Interactive<'a, F, R>: Debug + InteractiveMethods<'a, F, R> {
    fn __interactive_get_field(
        &'a mut self,
        field_name: &'a str,
    ) -> crate::Result<'a, &mut dyn crate::Interactive<'a, F, R>>;

    fn __interactive_eval_field(&'a self, field_name: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R;
}

pub trait InteractiveMethods<'a, F, R>: Debug {
    fn __interactive_eval_method(&'a mut self, method_name: &'a str, args: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn Debug>) -> R;
}

impl<'a, F, R, T: Debug + InteractiveMethods<'a, F, R>> Interactive<'a, F, R> for T {
    default fn __interactive_get_field(
        &'a mut self,
        field_name: &'a str,
    ) -> Result<'a, &mut dyn Interactive<'a, F, R>> {
        Err(InteractiveError::FieldNotFound {
            struct_name: type_name::<T>(),
            field_name,
        })
    }

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

impl<'a, F, R, T: Debug> InteractiveMethods<'a, F, R> for T {
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
