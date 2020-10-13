#![feature(min_specialization)]

use arrayvec::ArrayString;
use minus_i::{Interactive, InteractiveMethods, InteractiveRoot};

#[derive(Default)]
struct ChildStruct {}

#[InteractiveMethods]
impl ChildStruct {
    pub fn add(&mut self, a: f32, b: f32) -> f32 {
        a + b
    }
}

#[derive(Interactive, Default)]
struct ParentStruct {
    pub child: ChildStruct,
}

#[derive(InteractiveRoot, Default)]
struct Root {
    pub parent: ParentStruct,
}

fn main() -> core::fmt::Result {
    let mut root = Root::default();
    let mut buf = ArrayString::<[u8; 10]>::new();

    root.eval_and_write("parent.child.add(1, 2)", &mut buf)?;
    assert_eq!(buf.as_str(), "3.0");
    Ok(())
}
