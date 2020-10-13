#![doc(hidden)]

use core::fmt::Debug;

use crate::{InteractiveError, Result};
use core::any::type_name;

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

#[doc(hidden)]
#[allow(missing_copy_implementations)]
#[derive(Debug)]
/// Used as a dummy value for types that don't implement Debug inside #[derive(PartialDebug)].
pub struct Unknown;
