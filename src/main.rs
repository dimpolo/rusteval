#![feature(min_specialization)]

use repl::{Interactive, InteractiveMethods, InteractiveRoot};

#[derive(Interactive, Debug, Default)]
struct ChildStruct {
    pub last_sum: f32,
}

#[InteractiveMethods]
impl ChildStruct {
    pub fn add(&mut self, a: f32, b: f32) -> f32 {
        self.last_sum = a + b;
        self.last_sum
    }
}

#[derive(Interactive, Debug, Default)]
struct ParentStruct {
    pub child1: ChildStruct,
    pub child2: ChildStruct,
}

#[derive(InteractiveRoot, Debug, Default)]
struct Root {
    pub parent: ParentStruct,
}

fn main() -> std::io::Result<()> {
    use std::io;
    use std::io::Write;

    let mut repl = Root::default();

    loop {
        let mut input = String::new();
        print!(">>> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;
        println!("{}", repl.eval_to_debug_string(&input));
    }
}
