//! Docs and stuff TODO
//! Default not neccessary
//! # Known Limitations:
//! * derive not implemented for Enums
//! * interactive method args must implement parse
//!
//! # Example
//! ```no_run
//!
//! use minus_i::{Interactive, Methods, InteractiveRoot};
//!
//! #[derive(Interactive, Debug, Default)]
//! struct ChildStruct {
//!     last_sum: f32,
//! }
//!
//! #[Methods]
//! impl ChildStruct {
//!     fn add(&mut self, a: f32, b: f32) -> f32 {
//!         self.last_sum = a + b;
//!         self.last_sum
//!     }
//! }
//!
//! #[derive(Interactive, Debug, Default)]
//! struct ParentStruct {
//!     child1: ChildStruct,
//!     child2: ChildStruct,
//! }
//!
//! #[derive(InteractiveRoot, Debug, Default)]
//! struct Root {
//!     parent: ParentStruct,
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     use std::io;
//!     use std::io::Write;
//!
//!     let mut root = Root::default();
//!     let mut input = String::new();
//!
//!     loop {
//!         print!(">>> ");
//!         io::stdout().flush()?;
//!
//!         input.clear();
//!         io::stdin().read_line(&mut input)?;
//!         println!("{}", root.eval_to_string(&input));
//!     }
//! }
//! ```

#![allow(incomplete_features)] // TODO re-enable warning
#![feature(specialization)]
#![feature(str_split_once)]
#![feature(format_args_capture)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    rust_2018_idioms
)]
#![cfg_attr(not(feature = "std"), no_std)]

/// Derive this on a struct to make it an interactive access point for your application.
///
/// Same as `#[derive(Interactive)]` but with two additional impls:
/// * [`trait@InteractiveRoot`] with its default methods
/// * [`trait@Methods`] as a way to access free functions marked with the attribute [`macro@Function`] (only available with default features on).
///
///
/// ```
/// use minus_i::{Interactive, InteractiveRoot, Methods, Function};
///
/// #[derive(Interactive)]
/// struct SomethingInteractive;
///
/// #[Methods]
/// impl SomethingInteractive{
///     fn ping(&self) -> &str{
///         "pong"
///     }
/// }
///
/// #[Function]
/// fn add_one(a: u32) -> u32 {
///     a + 1
/// }
///
/// let something = SomethingInteractive;
///
/// #[derive(InteractiveRoot)]
/// struct Root {
///     field: SomethingInteractive,
/// }
///
/// let mut root = Root { field: something };
/// assert_eq!(root.eval_to_string("field.ping()"), "\"pong\"");
/// assert_eq!(root.eval_to_string("add_one(42)"), "43");
/// ```
pub use minus_i_derive::InteractiveRoot;

/// Gives interactive access to a structs fields.
/// ```
/// # use minus_i::Interactive;
/// #
/// #[derive(Interactive)]
/// struct Struct {
///     field1: u32,
///     field2: u32,
/// }
/// ```
/// Expands to something like:
/// ```
/// # use minus_i::*;
/// # use minus_i::InteractiveError::*;
/// # use minus_i::specialization::*;
/// # use core::fmt::Debug;
/// #
/// # struct Struct {
/// #     field1: u32,
/// #     field2: u32,
/// # }
/// impl Interactive for Struct {
///     fn get_field<'a>(&'a self, field_name: &'a str) -> Result<'_, &dyn Interactive> {
///         match field_name {
///             "field1" => self.field1.try_as_interactive(),
///             "field2" => self.field2.try_as_interactive(),
///             _ => Err(FieldNotFound {
///                 type_name: "Struct",
///                 field_name,
///             }),
///         }
///     }
///     fn get_field_mut<'a>(&'a mut self, field_name: &'a str) -> Result<'_, &mut dyn Interactive> {
///         /* ... */
///         # unimplemented!()
///     }
///     fn eval_field(&self, field_name: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>)) {
///         match field_name {
///             "field1" => f(self.field1.try_as_debug()),
///             /* ... */
///             # _ => unimplemented!(),
///         }
///     }
///     fn get_all_field_names(&self) -> &'static [&'static str] {
///         &["field1", "field2"]
///     }
/// }
/// ```
pub use minus_i_derive::Interactive;

/// Gives interactive access to a structs methods.
pub use minus_i_derive::Methods;

/// Implements [`Debug`] for a struct replacing all fields that do not implement `Debug` with a placeholder.
///
/// [`Debug`]: core::fmt::Debug
pub use minus_i_derive::PartialDebug;

/// Gives interactive access to a function.
#[cfg(feature = "std")]
pub use minus_i_derive::Function;

pub use error::{ArgParseError, InteractiveError, Result};
#[cfg(feature = "std")]
pub use function::Function;
pub use interactive::{Interactive, Methods};
pub use root::InteractiveRoot;

#[cfg(feature = "std")]
#[doc(hidden)]
pub use inventory;

pub mod arg_parse;
mod error;
mod function;
mod interactive;
mod root;
pub mod specialization;
