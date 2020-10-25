#![doc(hidden)]

use crate::{ArgParseError, InteractiveError};

pub trait ArgParse: Sized {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>>;
}

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
        fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
            s.trim().parse().map_err(ArgParseError::ParseIntError)
        }
      })*
    )
}

parse_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl ArgParse for bool {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
        s.trim().parse().map_err(ArgParseError::ParseBoolError)
    }
}

impl ArgParse for f32 {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
        s.trim().parse().map_err(ArgParseError::ParseFloatError)
    }
}

impl ArgParse for f64 {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
        s.trim().parse().map_err(ArgParseError::ParseFloatError)
    }
}

impl ArgParse for char {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
        unescape_char(s)
    }
}

#[cfg(feature = "std")]
impl ArgParse for String {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
        unescape_str(s)
    }
}

// "'A'" -> Ok('A')
fn unescape_char(s: &str) -> Result<char, ArgParseError<'_>> {
    let mut chars = s.chars();
    if chars.next() != Some('\'') {
        return Err(ArgParseError::UnescapeError(s));
    }
    if chars.next_back() != Some('\'') {
        return Err(ArgParseError::UnescapeError(s));
    }
    if chars.as_str().starts_with('\\') {
        chars.next(); // pop '\\'
        get_escaped_char(&mut chars).ok_or(ArgParseError::UnescapeError(s))
    } else {
        chars
            .as_str()
            .parse()
            .map_err(ArgParseError::ParseCharError)
    }
}

// "\"asfd\"" -> Ok("asdf")
fn unescape_str(s: &str) -> Result<String, ArgParseError<'_>> {
    let mut chars = s.chars();
    if chars.next() != Some('\"') {
        return Err(ArgParseError::UnescapeError(s));
    }
    if chars.next_back() != Some('\"') {
        return Err(ArgParseError::UnescapeError(s));
    }

    let mut res = String::with_capacity(chars.as_str().len() - 2);

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                let c = get_escaped_char(&mut chars).ok_or(ArgParseError::UnescapeError(s))?;
                res.push(c)
            }
            _ => res.push(c),
        }
    }

    Ok(res)
}

// "n" -> Some('\n')
fn get_escaped_char(after_backslash: &mut core::str::Chars<'_>) -> Option<char> {
    match after_backslash.next() {
        None => None,
        Some(char) => match char {
            'n' => Some('\n'),
            'r' => Some('\r'),
            't' => Some('\t'),
            '\\' => Some('\\'),
            '0' => Some('\0'),
            '\'' => Some('\''),
            '\"' => Some('\"'),
            'x' => parse_hex(after_backslash),
            'u' => parse_unicode(after_backslash),
            _ => None,
        },
    }
}

// "41" -> Some('A')
fn parse_hex(after_x: &mut core::str::Chars<'_>) -> Option<char> {
    let hex = after_x.as_str().get(..2)?;
    let u = u32::from_str_radix(hex, 16).ok()?;
    if u > 0x7f {
        return None;
    };
    let res = core::char::from_u32(u);

    // pop used chars
    after_x.next();
    after_x.next();
    res
}

// "{2764}" -> Some('❤')
fn parse_unicode(after_u: &mut core::str::Chars<'_>) -> Option<char> {
    if after_u.next() != Some('{') {
        return None;
    }
    if let Some(hex_end) = after_u.as_str().find('}') {
        let hex = &after_u.as_str()[..hex_end];
        let res = u32::from_str_radix(hex, 16)
            .ok()
            .and_then(core::char::from_u32);

        // pop used chars
        for _ in 0..=hex_end {
            after_u.next();
        }
        res
    } else {
        None
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
        test_parse_one_arg("'t'", 't');
    }

    #[test]
    fn test_escape_char() {
        test_parse_one_arg("'\\n'", '\n');
    }

    #[test]
    fn test_heart() {
        test_parse_one_arg("'\\u{2764}'", '❤');
    }

    #[test]
    fn test_hex() {
        test_parse_one_arg("'\\x41'", 'A');
    }

    #[test]
    fn test_easy_string() {
        test_parse_one_arg("\"test\"", String::from("test"));
    }

    #[test]
    fn test_complex_string() {
        test_parse_one_arg(
            "\" test \\n '\\u{2764}' \\r \\\"täst\\\" \\x41\"",
            String::from(" test \n '❤' \r \"täst\" A"),
        );
    }

    #[test]
    fn test_parse_five_args() {
        let result: (u8, u16, u32, u64, u128) = parse_5_args("", "1, 2, 3, 4, 5").unwrap();
        assert_eq!(result, (1, 2, 3, 4, 5));
    }
}
