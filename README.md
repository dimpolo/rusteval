# rusteval

This crate provides traits and macros that make your application's structs and functions interactive.

Annotating a struct with `#[derive(Interactive)]`, a struct's methods with `#[Methods]`
and a free function with `#[Function]` will implement a set of traits,
that will allow you to access them as if Rust had a REPL.

Use this crate as an alternative for "print debugging" or
as an ergonomic testing API.

This crate is `no_std` compatible so you can use it to interact with embedded devices
and blink those LEDs from a USB or UART connection.

## Usage
* Annotate everything you want to access with [`Interactive`], [`Methods`] and [`Function`]
* Define a new struct that owns or holds references to the objects you want to access
* Derive [`InteractiveRoot`] for it
* Use the trait's methods to evaluate a string
(the simplest one is [`eval_to_string`](InteractiveRoot::eval_to_string) but others allow for more custom behaviour)
* Accessing a field will give you its Debug representation
* Calling a function or a method will parse its arguments and give you the Debug representation of its return value

[`Interactive`]: macro@Interactive
[`Methods`]: macro@Methods
[`Function`]: macro@Function
[`InteractiveRoot`]: macro@InteractiveRoot

Since this crate makes a lot of use of the [`Debug`] trait the helper macro [`PartialDebug`] is provided.
It implements `Debug` for a struct replacing all fields that do not implement `Debug` with a placeholder.

[`Debug`]: core::fmt::Debug

#### CLI Usage
Functions like [`get_all_field_names`](Interactive::get_all_field_names) are provided.
This makes it possible to implement things like auto-completion.

Have a look at the autocomplete example for how this might be done using the [rustyline](https://docs.rs/crate/rustyline) crate.

## Example
```rust
use rusteval::{Interactive, Methods, InteractiveRoot, Function, PartialDebug};

#[derive(Default)]
struct NoDebug;

#[derive(Interactive, PartialDebug, Default)]
struct ChildStruct {
    last_sum: f32,
    no_debug: NoDebug,
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
    child: ChildStruct,
}

#[derive(InteractiveRoot, Debug, Default)]
struct Root {
    parent: ParentStruct,
}

#[Function]
fn split_str_at(s: &str, mid: usize) -> (&str, &str) {
    s.split_at(mid)
}

let mut root = Root::default();
assert_eq!(root.eval_to_string("parent.child.add(4.2, 6.9)"), "11.1");
assert_eq!(root.eval_to_string("parent.child"), "ChildStruct { last_sum: 11.1, no_debug: Unknown }");
// split_str_at("foobar", 3) => ("foo", "bar")
assert_eq!(root.eval_to_string("split_str_at(\"foobar\", 3)"), "(\"foo\", \"bar\")");
```

## How it works
This crate makes use of the unstable `specialization` feature, so it is only available on nightly.

Methods like `try_as_interactive` are implemented on all types.
The method normally returns an error but in the specialized case
a trait object (`&dyn Interactive` in this case) is returned.

The macros then implement getters that look something like this:
```rust
fn get_field<'a>(&'a self, field_name: &'a str) -> Result<'_, &dyn Interactive> {
    match field_name {
        "field1" => self.field1.try_as_interactive(),
        "field2" => self.field2.try_as_interactive(),
        _ => Err(InteractiveError::FieldNotFound {
            type_name: "Struct",
            field_name,
        }),
    }
}
```

See the macro's documentation for more details.

## Current limitations:
* Methods and functions can only be made interactive if their argument types are supported
* Enums are not supported


#### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
