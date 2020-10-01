use core::any::type_name;
use core::fmt::Debug;

pub type Result<'a, T> = core::result::Result<T, InteractiveError<'a>>;

#[non_exhaustive]
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
    ArgsError {
        given_args: &'a str,
    },
}

pub trait Interactive<'a, F, R>:
    Debug
    + InteractiveMethods<'a, F, R>
    + InteractiveFields<'a, F, R>
    + InteractiveFieldNames
    + InteractiveMethodNames
{
    fn __interactive_get_field(
        &'a self,
        field_name: &'a str,
    ) -> crate::Result<'a, &dyn crate::Interactive<'a, F, R>>;

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

pub trait InteractiveFields<'a, F, R>: Debug {
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

pub trait InteractiveMethods<'a, F, R>: Debug {
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

pub trait InteractiveFieldNames: Debug {
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

pub trait InteractiveMethodNames: Debug {
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
