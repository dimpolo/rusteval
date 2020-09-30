#![feature(min_specialization)]

use rustyline::Context;
use rustyline::Editor;
use rustyline_derive::{Helper, Highlighter, Validator};

use repl::{Interactive, InteractiveMethods, InteractiveRoot};
use rustyline::completion::Completer;
use rustyline::hint::Hinter;

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

fn main() -> rustyline::Result<()> {
    let root = Root::default();
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
    pub root: Root,
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
