extern crate repl_derive;

use core::fmt::Debug;

use anyhow::{anyhow, bail};

use repl::Interactive;
use repl_derive::{repl, Interactive};

impl TestStruct {
    fn ping(&self) {
        println!("pong");
    }

    fn answer(&self) -> i32 {
        42
    }
}

#[derive(Default, Debug)]
struct TestStruct2 {
    pub field: i32,
}
/*
impl TestStruct2 {
    fn ping(&self) {
        println!("pong");
    }

    fn answer(&self) -> i32 {
        42
    }
}

impl InteractiveCall for TestStruct2 {
    fn call(&mut self, method_name: &str) -> anyhow::Result<Option<Box<dyn Debug>>> {
        match method_name {
            "ping" => {
                self.ping();
                Ok(None)
            }
            "answer" => Ok(Some(Box::new(self.answer()) as Box<dyn Debug>)),
            _ => Err(anyhow!("TestStruct has no method {}", method_name)),
        }
    }
}*/

struct TestRepl {
    test: TestStruct,
}

impl TestRepl {
    fn new(test: TestStruct) -> Self {
        Self { test }
    }

    fn get(&mut self, name: &str, attr_name: &str) {
        let result = match name {
            "test" => self.test.__interactive_get_attribute(attr_name),
            _ => Err(repl::InteractiveError::InstanceNotFound {
                instance_name: name,
            }),
        };

        match result {
            Ok(result) => println!("{:?}", result),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn main() {
    let test = TestStruct::default();

    let mut repl = TestRepl::new(test);

    let mut _macro_repl = repl!(test: TestStruct);

    // repl.call("test", "ping");
    // repl.call("test", "answer");
    repl.get("test", "attr");
    repl.get("test", "ts2");
    // repl.call2("test", "ts2", "ping");

    /*
    repl.exec("test.ping()");
    repl.exec("test.answer()");
    repl.exec("test.attr()");
    repl.exec("test.ts2.ping()");
    repl.exec("test.add(6, 9)");
    repl.exec("test.ts2.add(6, 9)");
    */
}

#[derive(Interactive, Default, Debug)]
struct TestStruct {
    pub attr: u32,
    pub ts2: TestStruct2,
}
