//! This crate provides traits and macros that make your application's structs and functions interactive.
//!
//! Annotating a struct with `#[derive(Interactive)]`, a struct's methods with `#[Methods]`
//! and a free function with `#[Function]` will implement a set of traits,
//! that will allow you to access them as if Rust had a REPL.
//!
//! Use this crate as an alternative for "print debugging" or
//! as an ergonomic testing API.
//!
//! This crate is `no_std` compatible so you can use it to interact with embedded devices
//! and blink those LEDs from a USB or UART connection.
//!
//! # Usage
//! * Annotate everything you want to access with [`macro@Interactive`], [`macro@Methods`] and [`macro@Function`]
//! * Define a new struct that owns or holds references to the objects you want to access
//! * Derive [`macro@InteractiveRoot`] for it
//! * Use the trait's methods to evaluate a string
//! (the simplest one is [`eval_to_string`](InteractiveRoot::eval_to_string) but others allow for more custom behaviour)
//! * Accessing a field will give you its Debug representation
//! * Calling a function or a method will parse its arguments and give you the Debug representation of its return value
//!
//!
//! Since this crate makes a lot of use of the [`Debug`] trait the helper macro [`PartialDebug`] is provided.
//! It implements `Debug` for a struct replacing all fields that do not implement `Debug` with a placeholder.
//!
//! [`Debug`]: core::fmt::Debug
//!
//! ### CLI Usage
//! Functions like [`get_all_field_names`](Interactive::get_all_field_names) are provided.
//! This makes it possible to implement things like auto-completion.
//!
//! Have a look at the autocomplete example for how this might be done using the [rustyline](https://docs.rs/crate/rustyline) crate.
//!
//! # Example
//! ```
//! use minus_i::{Interactive, Methods, InteractiveRoot, Function, PartialDebug};
//!
//! #[derive(Default)]
//! struct NoDebug;
//!
//! #[derive(Interactive, PartialDebug, Default)]
//! struct ChildStruct {
//!     last_sum: f32,
//!     no_debug: NoDebug,
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
//!     child: ChildStruct,
//! }
//!
//! #[derive(InteractiveRoot, Debug, Default)]
//! struct Root {
//!     parent: ParentStruct,
//! }
//!
//! #[Function]
//! fn split_str_at(s: &str, mid: usize) -> (&str, &str) {
//!     s.split_at(mid)
//! }
//!
//! let mut root = Root::default();
//! assert_eq!(root.eval_to_string("parent.child.add(4.2, 6.9)"), "11.1");
//! assert_eq!(root.eval_to_string("parent.child"), "ChildStruct { last_sum: 11.1, no_debug: Unknown }");
//! // split_str_at("foobar", 3) => ("foo", "bar")
//! assert_eq!(root.eval_to_string("split_str_at(\"foobar\", 3)"), "(\"foo\", \"bar\")");
//! ```
//!
//! # How it works
//! This crate makes use of the unstable `specialization` feature, so it is only available on nightly.
//!
//! Methods like `try_as_interactive` are implemented on all types.
//! The method normally returns an error but in the specialized case
//! a trait object (`&dyn Interactive` in this case) is returned.
//!
//! The macros then implement getters that look something like this:
//! ```
//! # use minus_i::*;
//! # use minus_i::specialization::*;
//! # struct Stub {
//! #     field1: (),
//! #     field2: (),
//! # }
//! # impl Stub {
//! fn get_field<'a>(&'a self, field_name: &'a str) -> Result<'_, &dyn Interactive> {
//!     match field_name {
//!         "field1" => self.field1.try_as_interactive(),
//!         "field2" => self.field2.try_as_interactive(),
//!         _ => Err(InteractiveError::FieldNotFound {
//!             type_name: "Struct",
//!             field_name,
//!         }),
//!     }
//! }
//! # }
//! ```
//!
//! See the macro's documentation for more details.
//!
//! # Current limitations:
//! * Methods and functions can only be made interactive if their argument types are supported
//! * Enums are not supported

#![allow(incomplete_features)] // TODO re-enable warning
#![feature(specialization)]
#![feature(str_split_once)]
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
///
/// # What it does:
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
/// # use minus_i::specialization::*;
/// # use minus_i::InteractiveError::*;
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
///
/// Only methods with supported argument types will be made interactive.
///
/// Currently supported argument types are:
///
/// `bool`, `char`, `f32`, `f64`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`,
/// `u64`, `u128`, `usize`, `String`, `str`
///
/// References to these types are also supported.
///
/// Both `String` and `str` are only available with default features on.
///
/// # What it does:
/// ```
/// # use minus_i::Methods;
/// #
/// # struct Struct;
/// #
/// #[Methods]
/// impl Struct {
///     fn ping(&self) -> &str {
///         "pong"
///     }
///     fn frob(&mut self, arg: u32){
///         unimplemented!()
///     }
/// }
/// ```
/// Expands to something like:
/// (notice how `frob` is only available inside `eval_method_mut`)
/// ```
/// # use core::fmt::Debug;
/// # use minus_i::*;
/// # use minus_i::arg_parse::*;
/// # use minus_i::InteractiveError::*;
/// #
/// # struct Struct;
/// #
/// # impl Struct {
/// #     fn ping(&self) -> &str {
/// #         "pong"
/// #     }
/// #     fn frob(&mut self, arg: u32){
/// #         unimplemented!()
/// #     }
/// # }
/// #
/// impl Methods for Struct {
///     fn eval_method(&self, method_name: &str, args: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>)) {
///         match method_name {
///             "ping" => match parse_0_args(method_name, args) {
///                 Ok(()) => f(Ok(&self.ping())),
///                 Err(e) => f(Err(e)),
///             },
///             _ => f(Err(MethodNotFound {
///                 type_name: "Struct",
///                 method_name,
///             })),
///         }
///     }
///     fn eval_method_mut(&mut self, method_name: &str, args: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>)) {
///         match method_name {
///             "ping" => match parse_0_args(method_name, args) {
///                 Ok(()) => f(Ok(&self.ping())),
///                 Err(e) => f(Err(e)),
///             },
///             "frob" => match parse_1_arg(method_name, args) {
///                 Ok((arg0,)) => f(Ok(&self.frob(arg0))),
///                 Err(e) => f(Err(e)),
///             },
///             _ => f(Err(MethodNotFound {
///                 type_name: "Struct",
///                 method_name,
///             })),
///         }
///     }
///     fn get_all_method_names(&self) -> &'static [&'static str] {
///         &["ping", "frob"]
///     }
/// }
/// ```
pub use minus_i_derive::Methods;

/// Implements [`Debug`] for a struct replacing all fields that do not implement `Debug` with a placeholder.
///
/// [`Debug`]: core::fmt::Debug
///
/// # What it does:
/// ```
/// # use minus_i::PartialDebug;
///
/// struct NoDebug;
///
/// #[derive(PartialDebug)]
/// struct Struct {
///     field: NoDebug,
/// }
/// ```
///
/// Expands to something like:
/// ```
/// # use std::fmt::Debug;
/// # use core::fmt;
/// # use minus_i::specialization::AsDebug;
/// #
/// # struct NoDebug;
/// # struct Struct {
/// #     field: NoDebug,
/// # }
/// #
/// impl Debug for Struct {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         f.debug_struct("Struct")
///             .field(
///                 "field",
///                 match &self.field.try_as_debug() {
///                     Ok(field) => field,
///                     Err(_) => &minus_i::specialization::Unknown,
///                 },
///             )
///             .finish()
///     }
/// }
///
/// ```
pub use minus_i_derive::PartialDebug;

/// Gives interactive access to a function.
///
/// Can be used in different modules.
///
/// This makes use of the [inventory](https://docs.rs/inventory/*/inventory/) crate
/// to submit a wrapper struct to a global registry.
///
/// You can gain access to the wrapped function by using `#[derive(InteractiveRoot)]`. ([link])
///
/// Since the inventory crate requires std this macro is only available with default features on.
///
/// [link]: macro@crate::InteractiveRoot
/// # What it does:
/// ```
/// # use minus_i::Function;
/// #
/// #[Function]
/// fn add_one(a: u32) -> u32 {
///     a + 1
/// }
/// ```
/// Expands to something like:
/// ```
/// # use core::fmt::Debug;
/// # use minus_i::*;
/// # use minus_i::arg_parse::*;
/// # use minus_i::inventory;
///
/// # fn add_one(a: u32) -> u32 {
/// #     a + 1
/// # }
/// #
/// struct FunctionXYZ;
/// impl Function for FunctionXYZ {
///     fn eval(&self, args: &str, f: &mut dyn FnMut(Result<'_, &dyn Debug>)) {
///         match parse_1_arg(self.function_name(), args) {
///             Ok((arg0,)) => f(Ok(&add_one(arg0))),
///             Err(e) => f(Err(e)),
///         }
///     }
///     fn function_name(&self) -> &'static str {
///         "add_one"
///     }
/// }
/// inventory::submit! {
///     &FunctionXYZ as &dyn ::minus_i::Function
/// }
///
/// ```
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
