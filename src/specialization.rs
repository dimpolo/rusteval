#![doc(hidden)]

use core::any::type_name;
use core::fmt::Debug;

use crate::{Interactive, InteractiveError, Methods, Result};

macro_rules! duck_type {
    ($vis:vis $AsTrait:ident ($method:ident) : $Trait:path | $Error:ident) => {
        $vis trait $AsTrait {
            fn $method(&self) -> Result<'_, &dyn $Trait>;
        }

        impl<T> $AsTrait for T {
            default fn $method(&self) -> Result<'_, &dyn $Trait> {
                Err(InteractiveError::$Error {
                    type_name: type_name::<T>(),
                })
            }
        }

        impl<T> $AsTrait for T
        where
            T: $Trait,
        {
            fn $method(&self) -> Result<'_, &dyn $Trait> {
                Ok(self)
            }
        }
    };
}

macro_rules! duck_type_mut {
    ($vis:vis $AsTrait:ident ($method:ident) : $Trait:path | $Error:ident) => {
        $vis trait $AsTrait {
            fn $method(&mut self) -> Result<'_, &mut dyn $Trait>;
        }

        impl<T> $AsTrait for T {
            default fn $method(&mut self) -> Result<'_, &mut dyn $Trait> {
                Err(InteractiveError::$Error {
                    type_name: type_name::<T>(),
                })
            }
        }

        impl<T> $AsTrait for T
        where
            T: $Trait,
        {
            fn $method(&mut self) -> Result<'_, &mut dyn $Trait> {
                Ok(self)
            }
        }
    };
}

duck_type!(pub AsInteractive(try_as_interactive): Interactive | InteractiveNotImplemented);

duck_type_mut!(pub AsInteractiveMut(try_as_interactive_mut): Interactive | InteractiveNotImplemented);
duck_type!(pub AsMethods(try_as_methods): Methods | MethodsNotImplemented);

duck_type_mut!(pub AsMethodsMut(try_as_methods_mut): Methods | MethodsNotImplemented);
duck_type!(pub AsDebug(try_as_debug): Debug | DebugNotImplemented);

macro_rules! deref_for_interactive {
    ($AsTrait:ident ($method:ident) : $Trait:path) => {
        impl $AsTrait for &dyn Interactive {
            fn $method(&self) -> Result<'_, &dyn $Trait> {
                (&**self).$method()
            }
        }

        impl $AsTrait for &mut dyn Interactive {
            fn $method(&self) -> Result<'_, &dyn $Trait> {
                (&**self).$method()
            }
        }
    };
}

macro_rules! deref_for_interactive_mut {
    ($AsTrait:ident ($method:ident) : $Trait:path) => {
        impl $AsTrait for &mut dyn Interactive {
            fn $method(&mut self) -> Result<'_, &mut dyn $Trait> {
                (&mut **self).$method()
            }
        }
    };
}

deref_for_interactive!(AsMethods(try_as_methods): Methods);
deref_for_interactive_mut!(AsMethodsMut(try_as_methods_mut): Methods);
deref_for_interactive!(AsDebug(try_as_debug): Debug);

// TODO add AsIndex and maybe AsDeref for custom smart pointers
// use std::ops::{Index, IndexMut};
// duck_type!(pub AsIndex(try_as_index): Index<usize, Output = dyn AsInteractive> | InteractiveNotImplemented);
// duck_type_mut!(pub AsIndexMut(try_as_index_mut): IndexMut<usize, Output = dyn AsInteractiveMut> | InteractiveNotImplemented);

// deref_for_interactive!(AsIndex(try_as_index): Index<usize, Output = dyn AsInteractive>);
// deref_for_interactive_mut!(AsIndexMut(try_as_index_mut): IndexMut<usize, Output = dyn AsInteractiveMut>);

#[allow(missing_copy_implementations)]
#[derive(Debug)]
/// Used as a dummy value for types that don't implement Debug inside #[derive(PartialDebug)].
pub struct Unknown;
