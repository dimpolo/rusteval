//! Docs and stuff TODO
//! Default not neccessary
//! # Known Limitations:
//! * Enums
//!
//! # Example
//! ```no_run
//! #![feature(min_specialization)]
//!
//! use repl::{Interactive, InteractiveMethods, InteractiveRoot, AsDebug};
//!
//! #[derive(Interactive, Debug, Default)]
//! struct ChildStruct {
//!     pub last_sum: f32,
//! }
//!
//! #[InteractiveMethods]
//! impl ChildStruct {
//!     pub fn add(&mut self, a: f32, b: f32) -> f32 {
//!         self.last_sum = a + b;
//!         self.last_sum
//!     }
//! }
//!
//! #[derive(Interactive, Debug, Default)]
//! struct ParentStruct {
//!     pub child1: ChildStruct,
//!     pub child2: ChildStruct,
//! }
//!
//! #[derive(InteractiveRoot, Debug, Default)]
//! struct Root {
//!     pub parent: ParentStruct,
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
//!         io::stdin().read_line(&mut input)?;
//!         println!("{}", root.eval_to_debug_string(&input));
//!     }
//! }
//! ```

#![allow(incomplete_features)] // TODO re-enable warning
#![feature(specialization)]
#![feature(str_split_once)]
#![warn(missing_docs, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use as_debug::AsDebug;
pub use interactive::{
    Interactive, InteractiveError, InteractiveFieldNames, InteractiveFields,
    InteractiveMethodNames, InteractiveMethods, Result,
};
pub use repl_derive::{Interactive, InteractiveMethods, InteractiveRoot, PartialDebug};
pub use root::InteractiveRoot;

mod as_debug;
mod interactive;
mod root;
