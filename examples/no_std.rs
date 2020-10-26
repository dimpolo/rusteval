use arrayvec::ArrayString;
use minus_i::{Interactive, InteractiveRoot, Methods};

#[derive(Interactive, Default)]
struct ChildStruct {}

#[Methods]
impl ChildStruct {
    fn add(&self, a: f32, b: f32) -> f32 {
        a + b
    }

    fn _not_interactive(&self, _: &str) {}
}

#[derive(Interactive, Default)]
struct ParentStruct {
    child: ChildStruct,
}

#[derive(InteractiveRoot, Default)]
struct Root {
    parent: ParentStruct,
}

fn main() -> core::fmt::Result {
    let mut root = Root::default();
    let mut buf = ArrayString::<[u8; 100]>::new();

    root.eval_and_write("parent.child.add(1, 2)", &mut buf)?;
    assert_eq!(buf.as_str(), "3.0");
    Ok(())
}
