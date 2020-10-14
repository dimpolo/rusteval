#![feature(min_specialization)]

use rustyline::completion::Completer;
use rustyline::Context;
use rustyline::Editor;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

use minus_i::{Function, Interactive, InteractiveRoot, Methods};

#[derive(Interactive, Debug, Default)]
struct ChildStruct {
    last_sum: f32,
}

#[Methods]
impl ChildStruct {
    fn add(&mut self, a: f32, b: f32) -> f32 {
        self.last_sum = a + b;
        self.last_sum
    }
}

#[derive(Interactive, Debug, Default)]
struct ParentStruct {
    child1: ChildStruct,
    child2: ChildStruct,
}

#[derive(InteractiveRoot, Debug, Default)]
struct Root {
    parent: ParentStruct,
}

#[Function]
fn add_one(a: u32) -> u32 {
    a + 1
}

fn main() -> rustyline::Result<()> {
    let root = Root::default();
    let h = RustyLine { root };

    let mut rl = Editor::new();
    rl.set_helper(Some(h));

    loop {
        let input = rl.readline(">>> ")?;
        let root = &mut rl.helper_mut().unwrap().root;
        println!("{}", root.eval_to_string(&input));
    }
}

#[derive(Helper, Highlighter, Hinter, Validator)]
struct RustyLine {
    root: Root,
}

/// Uses get_queried_object to get a reference to the object before the last entered '.'
/// This object then is used to feed the RustyLine Completer with
/// get_all_field_names and get_all_method_names
impl Completer for RustyLine {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        if let Ok((current_object, rest_line)) = self.root.get_queried_object(line) {
            let start_len = line.len() - rest_line.len();

            let candidates = current_object
                .get_all_field_names()
                .iter()
                .chain(current_object.get_all_method_names())
                .filter(|candidate| candidate.starts_with(&line[start_len..pos]))
                .map(|s| s.to_string())
                .collect();

            Ok((start_len, candidates))
        } else {
            Ok((0, vec![]))
        }
    }
}
