use core::fmt::{Display, Formatter};

/// The result type of most interactive methods
pub type Result<'a, T> = core::result::Result<T, InteractiveError<'a>>;

/// The main error type of this crate
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InteractiveError<'a> {
    #[allow(missing_docs)]
    InteractiveNotImplemented { type_name: &'a str },
    #[allow(missing_docs)]
    MethodsNotImplemented { type_name: &'a str },
    #[allow(missing_docs)]
    DebugNotImplemented { type_name: &'static str },
    #[allow(missing_docs)]
    FieldNotFound {
        type_name: &'a str,
        field_name: &'a str,
    },
    #[allow(missing_docs)]
    MethodNotFound {
        type_name: &'a str,
        method_name: &'a str,
    },
    #[allow(missing_docs)]
    WrongNumberOfArguments {
        method_name: &'a str,
        expected: usize,
        found: usize,
    },
    #[allow(missing_docs)]
    ArgParseError {
        method_name: &'a str,
        error: ArgParseError,
    },
    #[allow(missing_docs)]
    SyntaxError,
    #[allow(missing_docs)]
    FunctionNotFound { function_name: &'a str },
}

impl Display for InteractiveError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            InteractiveError::InteractiveNotImplemented { type_name } => {
                write!(f, "`{}` doesn't implement `Interactive`", type_name)
            }
            InteractiveError::MethodsNotImplemented { type_name } => {
                write!(f, "`{}` doesn't implement `Methods`", type_name)
            }
            InteractiveError::DebugNotImplemented { type_name } => {
                write!(f, "´{}´ doesn't implement ´Debug´", type_name)
            }
            InteractiveError::FieldNotFound {
                type_name,
                field_name,
            } => write!(
                f,
                "No field `{}` found for type `{}`",
                field_name, type_name
            ),
            InteractiveError::MethodNotFound {
                method_name,
                type_name,
            } => write!(
                f,
                "No method named `{}` found for type `{}`",
                method_name, type_name
            ),
            InteractiveError::WrongNumberOfArguments {
                method_name,
                expected,
                found,
            } => {
                let arguments_1 = if *expected == 1 {
                    "argument"
                } else {
                    "arguments"
                };
                let arguments_2 = if *found == 1 { "argument" } else { "arguments" };
                let was_were = if *found == 1 { "was" } else { "were" };
                write!(
                    f,
                    "´{}´ takes {} {} but {} {} {} supplied",
                    method_name, expected, arguments_1, found, arguments_2, was_were
                )
            }
            InteractiveError::ArgParseError { error, .. } => write!(
                f,
                "Couldn't parse method/function argument(s)\n{:?}",
                error // TODO improve message
            ),
            InteractiveError::SyntaxError => write!(f, "Syntax Error"),
            InteractiveError::FunctionNotFound { function_name } => {
                write!(f, "No function named `{}` found", function_name)
            }
        }
    }
}
/// Contains information about function or method argument parsing errors.
///
/// It is used inside the [`InteractiveError::ArgParseError`] variant.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArgParseError {
    #[allow(missing_docs)]
    ParseIntError(core::num::ParseIntError),
    #[allow(missing_docs)]
    ParseCharError(core::char::ParseCharError),
    #[allow(missing_docs)]
    ParseFloatError(core::num::ParseFloatError),
    #[allow(missing_docs)]
    ParseBoolError(core::str::ParseBoolError),

    #[cfg(feature = "std")]
    /// `snailquote` error as string.
    /// Produced when parsing string-like types.
    UnescapeError(std::string::String),
}

impl core_error::Error for InteractiveError<'_> {}
