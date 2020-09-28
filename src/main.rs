#![feature(min_specialization)]

use repl::{Interactive, InteractiveMethods, InteractiveRoot};

#[cfg(feature = "std")]
fn main() -> std::io::Result<()> {
    use std::io;
    use std::io::Write;

    let mut repl = GenRepl::default();

    loop {
        let mut input = String::new();
        print!(">>> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;
        println!("{}", repl.eval_to_debug_string(&input));
    }
}

#[cfg(not(feature = "std"))]
fn main() {}

#[derive(Interactive, Default, Debug)]
struct TestStruct {
    pub a: bool,
}

#[InteractiveMethods]
impl TestStruct {
    pub fn try_ping(&self) -> Result<String, ()> {
        Ok("pong".into())
    }
    pub fn answer(&self) {
        println!("42");
    }
}

#[derive(Interactive, Debug, Default)]
struct ParentStruct {
    pub child: TestStruct,
}

#[derive(InteractiveRoot, Default, Debug)]
struct GenRepl {
    pub parent: ParentStruct,
}

impl GenRepl {
    #[cfg(feature = "std")]
    fn eval_to_debug_string(&mut self, expression: &str) -> String {
        self.try_eval(expression, |result| format!("{:?}", result))
    }
}
