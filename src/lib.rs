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

/// Derive this on a struct to make it an interactive access point to your application.
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

pub use error::{ArgParseError, InteractiveError, Result};
pub use interactive::{Interactive, Methods};
pub use minus_i_derive::{Interactive, Methods, PartialDebug};
pub use root::InteractiveRoot;

#[cfg(feature = "std")]
#[doc(hidden)]
pub use inventory;

#[cfg(feature = "std")]
pub use function::Function;

#[cfg(feature = "std")]
pub use minus_i_derive::Function;

pub mod arg_parse;
mod error;
mod function;
mod interactive;
mod root;
pub mod specialization;
