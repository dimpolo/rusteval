use core::fmt::{Display, Formatter};

/// The result type of most interactive methods
pub type Result<'a, T> = core::result::Result<T, InteractiveError<'a>>;

/// The main error type of this crate
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum InteractiveError<'a> {
    #[allow(missing_docs)]
    MethodNotFound {
        type_name: &'a str,
        method_name: &'a str,
    },
    #[allow(missing_docs)]
    FieldNotFound {
        type_name: &'a str,
        field_name: &'a str,
    },
    #[allow(missing_docs)]
    WrongNumberOfArguments { expected: usize, found: usize },
    #[allow(missing_docs)]
    ArgsError { given_args: &'a str }, // TODO add more detailed error
    #[allow(missing_docs)]
    SyntaxError,
    #[allow(missing_docs)]
    DebugNotImplemented { type_name: &'static str },
    #[allow(missing_docs)]
    FunctionNotFound { function_name: &'a str },
}

impl Display for InteractiveError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InteractiveError::MethodNotFound {
                method_name,
                type_name,
            } => write!(
                f,
                "No method named `{method_name}` found for type `{type_name}`",
            ),
            InteractiveError::FieldNotFound {
                type_name,
                field_name,
            } => write!(f, "No field `{field_name}` found for type `{type_name}`",),
            InteractiveError::WrongNumberOfArguments { expected, found } => {
                let arguments_1 = if *expected == 1 {
                    "argument"
                } else {
                    "arguments"
                };
                let arguments_2 = if *found == 1 { "argument" } else { "arguments" };
                let was_were = if *found == 1 { "was" } else { "were" };
                write!(
                    f,
                    "This function takes {expected} {arguments_1} but {found} {arguments_2} {was_were} supplied",
                )
            }
            InteractiveError::ArgsError { given_args } => write!(
                f,
                "Could not parse `{given_args}` as method/function argument(s)." // TODO improve message
            ),
            InteractiveError::SyntaxError => write!(f, "Syntax Error"),
            InteractiveError::DebugNotImplemented { .. } => write!(f, "unimplemented!()"),
            InteractiveError::FunctionNotFound { function_name } => {
                write!(f, "No function named `{function_name}` found")
            }
        }
    }
}

impl core_error::Error for InteractiveError<'_> {}
