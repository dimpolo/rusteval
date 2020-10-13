#![doc(hidden)]

use crate::{ArgParseError, InteractiveError};

pub trait ArgParse: Sized {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError>;
}

#[doc(hidden)]
pub fn parse_arg<'a, T: ArgParse>(
    method_name: &'a str,
    args_iterator: &mut impl Iterator<Item = &'a str>,
) -> crate::Result<'a, T> {
    let arg_str = args_iterator.next().unwrap(); // length was already checked
    ArgParse::arg_parse(arg_str).map_err(|e| InteractiveError::ArgParseError {
        method_name,
        error: e,
    })
}

#[doc(hidden)]
pub fn parse_0_args<'a>(method_name: &'a str, args: &'a str) -> crate::Result<'a, ()> {
    let args_count = args.split_terminator(',').count();

    if args_count != 0 {
        return Err(InteractiveError::WrongNumberOfArguments {
            method_name,
            expected: 0,
            found: args_count,
        });
    }
    Ok(())
}

#[doc(hidden)]
pub fn parse_1_arg<'a, T0: ArgParse>(
    method_name: &'a str,
    args: &'a str,
) -> crate::Result<'a, (T0,)> {
    let args_count = args.split_terminator(',').count();

    if args_count != 1 {
        return Err(InteractiveError::WrongNumberOfArguments {
            method_name,
            expected: 1,
            found: args_count,
        });
    }
    let mut args_iterator = args.split_terminator(',');
    let arg0 = parse_arg(method_name, &mut args_iterator)?;
    Ok((arg0,))
}

#[doc(hidden)]
pub fn parse_2_args<'a, T0: ArgParse, T1: ArgParse>(
    method_name: &'a str,
    args: &'a str,
) -> crate::Result<'a, (T0, T1)> {
    let args_count = args.split_terminator(',').count();

    if args_count != 2 {
        return Err(InteractiveError::WrongNumberOfArguments {
            method_name,
            expected: 2,
            found: args_count,
        });
    }
    let mut args_iterator = args.split_terminator(',');
    let arg0 = parse_arg(method_name, &mut args_iterator)?;
    let arg1 = parse_arg(method_name, &mut args_iterator)?;
    Ok((arg0, arg1))
}

#[doc(hidden)]
pub fn parse_3_args<'a, T0: ArgParse, T1: ArgParse, T2: ArgParse>(
    method_name: &'a str,
    args: &'a str,
) -> crate::Result<'a, (T0, T1, T2)> {
    let args_count = args.split_terminator(',').count();

    if args_count != 3 {
        return Err(InteractiveError::WrongNumberOfArguments {
            method_name,
            expected: 3,
            found: args_count,
        });
    }
    let mut args_iterator = args.split_terminator(',');
    let arg0 = parse_arg(method_name, &mut args_iterator)?;
    let arg1 = parse_arg(method_name, &mut args_iterator)?;
    let arg2 = parse_arg(method_name, &mut args_iterator)?;
    Ok((arg0, arg1, arg2))
}

#[doc(hidden)]
pub fn parse_4_args<'a, T0: ArgParse, T1: ArgParse, T2: ArgParse, T3: ArgParse>(
    method_name: &'a str,
    args: &'a str,
) -> crate::Result<'a, (T0, T1, T2, T3)> {
    let args_count = args.split_terminator(',').count();

    if args_count != 4 {
        return Err(InteractiveError::WrongNumberOfArguments {
            method_name,
            expected: 4,
            found: args_count,
        });
    }
    let mut args_iterator = args.split_terminator(',');
    let arg0 = parse_arg(method_name, &mut args_iterator)?;
    let arg1 = parse_arg(method_name, &mut args_iterator)?;
    let arg2 = parse_arg(method_name, &mut args_iterator)?;
    let arg3 = parse_arg(method_name, &mut args_iterator)?;
    Ok((arg0, arg1, arg2, arg3))
}

#[doc(hidden)]
pub fn parse_5_args<'a, T0: ArgParse, T1: ArgParse, T2: ArgParse, T3: ArgParse, T4: ArgParse>(
    method_name: &'a str,
    args: &'a str,
) -> crate::Result<'a, (T0, T1, T2, T3, T4)> {
    let args_count = args.split_terminator(',').count();

    if args_count != 5 {
        return Err(InteractiveError::WrongNumberOfArguments {
            method_name,
            expected: 5,
            found: args_count,
        });
    }
    let mut args_iterator = args.split_terminator(',');
    let arg0 = parse_arg(method_name, &mut args_iterator)?;
    let arg1 = parse_arg(method_name, &mut args_iterator)?;
    let arg2 = parse_arg(method_name, &mut args_iterator)?;
    let arg3 = parse_arg(method_name, &mut args_iterator)?;
    let arg4 = parse_arg(method_name, &mut args_iterator)?;
    Ok((arg0, arg1, arg2, arg3, arg4))
}

macro_rules! parse_int {
    ($($t:ty),*) => (
      $(impl ArgParse for $t {
        fn arg_parse(s: &str) -> Result<Self, ArgParseError> {
            s.trim().parse().map_err(ArgParseError::ParseIntError)
        }
      })*
    )
}

parse_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl ArgParse for bool {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError> {
        s.trim().parse().map_err(ArgParseError::ParseBoolError)
    }
}

impl ArgParse for f32 {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError> {
        s.trim().parse().map_err(ArgParseError::ParseFloatError)
    }
}

impl ArgParse for f64 {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError> {
        s.trim().parse().map_err(ArgParseError::ParseFloatError)
    }
}

#[cfg(feature = "std")]
impl ArgParse for char {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError> {
        let char_candidate = snailquote::unescape(s).map_err(|_| ArgParseError::Stuff)?;
        char_candidate
            .parse()
            .map_err(ArgParseError::ParseCharError)
    }
}

#[cfg(feature = "std")]
impl ArgParse for String {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError> {
        snailquote::unescape(s).map_err(|_| ArgParseError::Stuff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arg_parse::parse_5_args;

    fn test_parse_one_arg<T: ArgParse + PartialEq + core::fmt::Debug>(arg: &str, expected: T) {
        let result: T = parse_1_arg("", arg).unwrap().0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_floats() {
        test_parse_one_arg("1", 1f32);
        test_parse_one_arg("1", 1f64);
        test_parse_one_arg("-1", -1f32);
        test_parse_one_arg("-1.0", -1f32);
    }

    #[test]
    fn test_ints() {
        test_parse_one_arg("1", 1u8);
        test_parse_one_arg("-1", -1i128);
    }

    #[test]
    fn test_bool() {
        test_parse_one_arg("true", true);
        test_parse_one_arg("false", false);
    }

    #[test]
    fn test_char() {
        test_parse_one_arg("\"t\"", 't');
    }

    #[test]
    fn test_escape_char() {
        test_parse_one_arg("\"\\n\"", '\n');
    }

    #[test]
    fn test_easy_string() {
        test_parse_one_arg("\"test\"", String::from("test"));
    }

    #[test]
    fn test_parse_five_args() {
        let result: (u8, u16, u32, u64, u128) = parse_5_args("", "1, 2, 3, 4, 5").unwrap();
        assert_eq!(result, (1, 2, 3, 4, 5));
    }
}
