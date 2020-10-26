#![no_implicit_prelude]
use ::minus_i::{Function, Interactive, InteractiveRoot, Methods, PartialDebug};

// These are required for now
use ::core::ops::*;
use ::core::option::Option::*;
use ::core::result::Result::*;
use ::minus_i::inventory;

struct NoDebug;

#[derive(Interactive, PartialDebug)]
struct ChildStruct {
    last_sum: f32,
    no_debug: NoDebug,
}

#[Methods]
impl ChildStruct {
    fn yes(&mut self, _a: f32, _b: &str, _c: &mut str) -> bool {
        true
    }
}

#[derive(InteractiveRoot)]
struct Root {
    child: ChildStruct,
}

#[Function]
fn split_str_at(s: &str, mid: usize) -> (&str, &str) {
    s.split_at(mid)
}
