#![doc(hidden)]

use core::any::type_name;
use core::fmt::Debug;

use crate::{Interactive, InteractiveError, Methods, Result};

#[doc(hidden)]
pub trait AsDebug {
    fn try_as_debug(&self) -> Result<'_, &dyn Debug>;
}

impl<T> AsDebug for T {
    default fn try_as_debug(&self) -> Result<'_, &dyn Debug> {
        Err(InteractiveError::DebugNotImplemented {
            type_name: type_name::<T>(),
        })
    }
}

impl<T> AsDebug for T
where
    T: Debug,
{
    fn try_as_debug(&self) -> Result<'_, &dyn Debug> {
        Ok(self)
    }
}

impl AsDebug for &dyn Interactive {
    fn try_as_debug(&self) -> Result<'_, &dyn Debug> {
        (&**self).try_as_debug()
    }
}

impl AsDebug for &mut dyn Interactive {
    fn try_as_debug(&self) -> Result<'_, &dyn Debug> {
        (&**self).try_as_debug()
    }
}

impl AsDebug for Box<dyn Interactive> {
    fn try_as_debug(&self) -> Result<'_, &dyn Debug> {
        (&**self).try_as_debug()
    }
}

#[doc(hidden)]
#[allow(missing_copy_implementations)]
#[derive(Debug)]
/// Used as a dummy value for types that don't implement Debug inside #[derive(PartialDebug)].
pub struct Unknown;

#[doc(hidden)]
pub trait AsInteractive {
    fn try_as_interactive(&self) -> Result<'_, &dyn Interactive>;
    fn try_as_interactive_mut(&mut self) -> Result<'_, &mut dyn Interactive>;
}

impl<T> AsInteractive for T {
    default fn try_as_interactive(&self) -> Result<'_, &dyn Interactive> {
        Err(InteractiveError::InteractiveNotImplemented {
            type_name: type_name::<T>(),
        })
    }
    default fn try_as_interactive_mut(&mut self) -> Result<'_, &mut dyn Interactive> {
        Err(InteractiveError::InteractiveNotImplemented {
            type_name: type_name::<T>(),
        })
    }
}

impl<T> AsInteractive for T
where
    T: Interactive,
{
    fn try_as_interactive(&self) -> Result<'_, &dyn Interactive> {
        Ok(self)
    }

    fn try_as_interactive_mut(&mut self) -> Result<'_, &mut dyn Interactive> {
        Ok(self)
    }
}

#[doc(hidden)]
pub trait AsMethods {
    fn try_as_methods(&self) -> Result<'_, &dyn Methods>;
    fn try_as_methods_mut(&mut self) -> Result<'_, &mut dyn Methods> {
        Err(InteractiveError::MethodsNotImplemented {
            type_name: type_name::<Self>(),
        })
    }
}

impl<T> AsMethods for T {
    default fn try_as_methods(&self) -> Result<'_, &dyn Methods> {
        Err(InteractiveError::MethodsNotImplemented {
            type_name: type_name::<T>(),
        })
    }

    default fn try_as_methods_mut(&mut self) -> Result<'_, &mut dyn Methods> {
        Err(InteractiveError::MethodsNotImplemented {
            type_name: type_name::<T>(),
        })
    }
}

impl<T> AsMethods for T
where
    T: Methods,
{
    fn try_as_methods(&self) -> Result<'_, &dyn Methods> {
        Ok(self)
    }

    fn try_as_methods_mut(&mut self) -> Result<'_, &mut dyn Methods> {
        Ok(self)
    }
}
