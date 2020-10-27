use rusteval::{Function, Interactive, InteractiveRoot, Methods};

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
fn split_str_at(s: &str, mid: usize) -> (&str, &str) {
    s.split_at(mid)
}

fn main() -> std::io::Result<()> {
    use std::io;
    use std::io::Write;

    let mut root = Root::default();
    let mut input = String::new();

    loop {
        print!(">>> ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;
        println!("{}", root.eval_to_string(&input));
    }
}
