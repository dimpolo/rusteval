#![feature(min_specialization)]

pub use repl_derive::{repl, Interactive, InteractiveMethods};

use core::fmt::Debug;
use std::any::type_name;

pub type Result<'a, T> = core::result::Result<T, InteractiveError<'a>>;

#[derive(Debug, PartialEq, Eq)]
pub enum InteractiveError<'a> {
    MethodNotFound {
        struct_name: &'a str,
        method_name: &'a str,
    },
    AttributeNotFound {
        struct_name: &'a str,
        field_name: &'a str,
    },
    InstanceNotFound {
        instance_name: &'a str,
    },
}

pub trait Interactive<'a>: Debug {
    fn __interactive_get_field(
        &'a self,
        field_name: &'a str,
    ) -> crate::Result<'a, &dyn core::fmt::Debug>;
    fn __interactive_get_interactive_field(
        &'a mut self,
        field_name: &'a str,
    ) -> crate::Result<'a, &mut dyn crate::Interactive>;
}

pub trait InteractiveMethods<'a>: Debug {
    fn __interactive_call_method(
        &'a mut self,
        method_name: &'a str,
        args: &'a str,
    ) -> crate::Result<'a, core::option::Option<Box<dyn core::fmt::Debug>>>;
}

impl<'a, T: Debug> Interactive<'a> for T {
    default fn __interactive_get_field(&'a self, field_name: &'a str) -> Result<'a, &dyn Debug> {
        Err(InteractiveError::AttributeNotFound {
            struct_name: type_name::<T>(),
            field_name,
        })
    }

    default fn __interactive_get_interactive_field(
        &'a mut self,
        field_name: &'a str,
    ) -> Result<'a, &mut dyn Interactive<'a>> {
        Err(InteractiveError::AttributeNotFound {
            struct_name: type_name::<T>(),
            field_name,
        })
    }
}
