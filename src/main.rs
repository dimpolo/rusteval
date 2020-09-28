#![feature(min_specialization)]
#![feature(str_split_once)]

use repl::{Interactive, InteractiveMethods, Repl, Result};

#[derive(Default)]
struct GenRepl {
    parent: ParentStruct,
}

impl Repl for GenRepl {
    fn get_root_object<'a, F, R>(
        &mut self,
        object_name: &'a str,
    ) -> Result<&mut dyn Interactive<'a, F, R>> {
        match object_name {
            "parent" => Ok(&mut self.parent),
            _ => Err(repl::InteractiveError::InstanceNotFound {
                instance_name: object_name,
            }),
        }
    }

    fn eval_root_object<'a, F, R>(&'a self, object_name: &'a str, f: F) -> R
    where
        F: Fn(Result<&dyn ::core::fmt::Debug>) -> R,
    {
        match object_name {
            "parent" => f(Ok(&self.parent)),
            _ => f(Err(repl::InteractiveError::InstanceNotFound {
                instance_name: object_name,
            })),
        }
    }
}

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
    pub fn try_ping(&self) -> core::result::Result<String, ()> {
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
