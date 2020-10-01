//! Docs and stuff TODO

#![feature(min_specialization)]
#![feature(str_split_once)]
#![warn(missing_docs, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use interactive::{
    Interactive, InteractiveError, InteractiveFieldNames, InteractiveFields,
    InteractiveMethodNames, InteractiveMethods, Result,
};
pub use repl_derive::{Interactive, InteractiveMethods, InteractiveRoot};
pub use root::InteractiveRoot;

mod interactive;
mod root;
