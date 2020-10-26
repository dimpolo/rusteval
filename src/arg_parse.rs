#![doc(hidden)]

use crate::{ArgParseError, InteractiveError};

pub trait ArgParse: Sized {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>>;
}

pub fn parse_arg<'a, T: ArgParse>(
    method_name: &'a str,
    haystack: &mut &'a str,
    expected: usize,
    found: usize,
) -> crate::Result<'a, T> {
    let arg_str = get_next_arg(method_name, haystack, expected, found)?;

    ArgParse::arg_parse(arg_str).map_err(|e| InteractiveError::ArgParseError {
        method_name,
        error: e,
    })
}
pub fn get_next_arg<'a>(
    method_name: &'a str,
    haystack: &mut &'a str,
    expected: usize,
    found: usize,
) -> crate::Result<'a, &'a str> {
    match find_next_separator_index(haystack) {
        Some(arg_end_idx) => {
            let (arg_str, rest_str) = haystack.split_at(arg_end_idx);
            let arg_str = arg_str.trim();
            if arg_str.is_empty() {
                // no arg before separator
                return Err(InteractiveError::SyntaxError {});
            }
            *haystack = &rest_str[1..]; // skip separator
            Ok(arg_str)
        }
        None => {
            let arg_str = haystack.trim();
            if arg_str.is_empty() {
                // not enough args
                return Err(InteractiveError::WrongNumberOfArguments {
                    method_name,
                    expected,
                    found,
                });
            }
            *haystack = "";
            Ok(arg_str)
        }
    }
}

pub fn clear_args<'a>(
    method_name: &'a str,
    haystack: &mut &'a str,
    expected: usize,
    mut found: usize,
) -> crate::Result<'a, ()> {
    if !haystack.is_empty() {
        loop {
            get_next_arg(method_name, haystack, expected, found)?;
            found += 1;
        }
    }

    Ok(())
}

fn find_next_separator_index(s: &str) -> Option<usize> {
    let mut chars = s.char_indices();
    let mut inside_single_quotes = false;
    let mut inside_double_quotes = false;

    while let Some((idx, c)) = chars.next() {
        match c {
            '\\' => {
                chars.next();
            }
            ',' => {
                if !inside_double_quotes && !inside_single_quotes {
                    return Some(idx);
                }
            }
            '\'' => {
                if !inside_double_quotes {
                    inside_single_quotes = !inside_single_quotes;
                }
            }
            '"' => {
                if !inside_single_quotes {
                    inside_double_quotes = !inside_double_quotes;
                }
            }
            _ => {}
        }
    }
    None
}

pub fn parse_0_args<'a>(method_name: &'a str, mut args: &'a str) -> crate::Result<'a, ()> {
    clear_args(method_name, &mut args, 0, 0)
}

macro_rules! parse_x_args {
    ($funcname:ident::<$($TN:ident),*>, ($($i:literal),*), x=$x:literal) => {
        #[allow(non_snake_case)]
        pub fn $funcname<'a, $($TN: ArgParse,)*>(
            method_name: &'a str,
            mut args: &'a str,
        ) -> crate::Result<'a, ($($TN,)*)> {
            $(let $TN  = parse_arg(method_name, &mut args, $x, $i)?;)*
            clear_args(method_name, &mut args, $x, $x)?;
            Ok(($($TN,)*))
        }
    };
}

parse_x_args!(parse_1_arg::<T0>, (0), x = 1);
parse_x_args!(parse_2_args::<T0, T1>, (0, 1), x = 2);
parse_x_args!(parse_3_args::<T0, T1, T2>, (0, 1, 2), x = 3);
parse_x_args!(parse_4_args::<T0, T1, T2, T3>, (0, 1, 2, 3), x = 4);
parse_x_args!(parse_5_args::<T0, T1, T2, T3, T4>, (0, 1, 2, 3, 4), x = 5);
parse_x_args!(
    parse_6_args::<T0, T1, T2, T3, T4, T5>,
    (0, 1, 2, 3, 4, 5),
    x = 6
);

macro_rules! parse_int {
    ($($t:ty),*) => (
      $(impl ArgParse for $t {
        fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
            s.parse().map_err(ArgParseError::ParseIntError)
        }
      })*
    )
}

parse_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl ArgParse for bool {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
        s.parse().map_err(ArgParseError::ParseBoolError)
    }
}

impl ArgParse for f32 {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
        s.parse().map_err(ArgParseError::ParseFloatError)
    }
}

impl ArgParse for f64 {
    fn arg_parse(s: &str) -> Result<Self, ArgParseError<'_>> {
        s.parse().map_err(ArgParseError::ParseFloatError)
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
#[cfg(feature = "std")]
fn unescape_str(s: &str) -> Result<String, ArgParseError<'_>> {
    let mut chars = s.chars();
    if chars.next() != Some('\"') {
        return Err(ArgParseError::UnescapeError(s));
    }
    if chars.next_back() != Some('\"') {
        return Err(ArgParseError::UnescapeError(s));
    }

    let mut res = String::with_capacity(chars.as_str().len());

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
// https://doc.rust-lang.org/reference/tokens.html
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
            'x' => parse_ascii(after_backslash),
            'u' => parse_unicode(after_backslash),
            _ => None,
        },
    }
}

// "41" -> Some('A')
fn parse_ascii(after_x: &mut core::str::Chars<'_>) -> Option<char> {
    let hex = after_x.as_str().get(..2)?;
    let ascii = u32::from_str_radix(hex, 16).ok()?;
    if ascii > 0x7f {
        return None;
    };
    let res = core::char::from_u32(ascii);

    // pop used chars
    after_x.nth(1);
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
        after_u.nth(hex_end);
        res
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_string_with_comma() {
        test_parse_one_arg("\"1, 2\"", String::from("1, 2"));
    }

    #[test]
    fn test_parse_five_args() {
        let result: (u8, u16, u32, u64, u128) = parse_5_args("", "1, 2, 3, 4, 5").unwrap();
        assert_eq!(result, (1, 2, 3, 4, 5));
    }

    #[test]
    fn test_find_separator() {
        assert_eq!(find_next_separator_index("\",\", \",\""), Some(3));
        assert_eq!(find_next_separator_index("',', ','"), Some(3));
        assert_eq!(find_next_separator_index("4, 5"), Some(1));
    }

    #[test]
    fn test_too_many_args() {
        assert_eq!(
            parse_2_args::<u32, u32>("test", "1, 2, 3, 4").unwrap_err(),
            InteractiveError::WrongNumberOfArguments {
                method_name: "test",
                expected: 2,
                found: 4
            }
        )
    }

    #[test]
    fn test_too_few_args() {
        assert_eq!(
            parse_2_args::<u32, u32>("test", "1").unwrap_err(),
            InteractiveError::WrongNumberOfArguments {
                method_name: "test",
                expected: 2,
                found: 1
            }
        )
    }
}
