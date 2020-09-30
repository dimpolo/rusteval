#![feature(min_specialization)]

use rustyline::Context;
use rustyline::Editor;
use rustyline_derive::{Helper, Highlighter, Validator};

use repl::{Interactive, InteractiveMethods, InteractiveRoot};
use rustyline::completion::Completer;
use rustyline::hint::Hinter;

#[derive(Interactive, Default, Debug)]
struct TestStruct {
    pub inner: bool,
}

#[InteractiveMethods]
impl TestStruct {
    pub fn try_ping(&self) -> Result<String, ()> {
        Ok("pong".into())
    }
    pub fn answer(&self) {
        println!("42");
    }
    pub fn add(&self, a: f32, b: f32) -> f32 {
        a + b
    }
}

#[derive(InteractiveRoot, Default, Debug)]
struct MyInteractiveRoot {
    pub test: TestStruct,
}

fn main() -> rustyline::Result<()> {
    let root = MyInteractiveRoot::default();
    let h = RustyLine { root };

    let mut rl = Editor::new();
    rl.set_helper(Some(h));

    loop {
        let input = rl.readline(">>> ")?;
        println!(
            "{}",
            rl.helper_mut().unwrap().root.eval_to_debug_string(&input)
        );
    }
}

#[derive(Helper, Validator, Highlighter)]
struct RustyLine {
    pub root: MyInteractiveRoot,
}

impl Hinter for RustyLine {
    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if pos < line.len() {
            return None;
        }

        let (current_object, rest_line) =
            InteractiveRoot::<(), ()>::get_queried_object(&self.root, line).ok()?;

        let start_len = line.len().saturating_sub(rest_line.len());

        current_object
            .get_all_interactive_field_names()
            .iter()
            .chain(current_object.get_all_interactive_method_names())
            .filter_map(|hint| {
                if pos > start_len && hint.starts_with(&line[start_len..pos]) {
                    Some(hint[pos - start_len..].to_owned())
                } else {
                    None
                }
            })
            .next()
    }
}

impl Completer for RustyLine {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        if let Ok((current_object, rest_line)) =
            InteractiveRoot::<(), ()>::get_queried_object(&self.root, line)
        {
            let start_len = line.len().saturating_sub(rest_line.len());

            let candidates = current_object
                .get_all_interactive_field_names()
                .iter()
                .chain(current_object.get_all_interactive_method_names())
                .filter(|candidate| {
                    line.get(start_len..pos)
                        .map(|start| candidate.starts_with(start))
                        .unwrap_or(true)
                })
                .map(|s| s.to_string())
                .collect();

            Ok((start_len, candidates))
        } else {
            Ok((0, vec![]))
        }
    }
}

pub trait Candidate2 {
    /// Text to display when listing alternatives.
    fn display(&self) -> &str;
    /// Text to insert in line.
    fn replacement(&self) -> &str;
}
impl Candidate2 for &str {
    fn display(&self) -> &str {
        self
    }

    fn replacement(&self) -> &str {
        self
    }
}
