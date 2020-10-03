use crate::InteractiveError;
use std::fmt::Debug;

/// Docs and Stuff TODO
pub trait AsDebug {
    /// Docs and Stuff TODO
    fn as_debug(&self) -> &dyn Debug;
}

impl<T> AsDebug for T {
    default fn as_debug(&self) -> &dyn Debug {
        &InteractiveError::DebugNotImplemented
    }
}

impl<T> AsDebug for T
where
    T: Debug,
{
    fn as_debug(&self) -> &dyn Debug {
        self
    }
}
