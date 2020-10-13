#![doc(hidden)]

use crate::{InteractiveError, Result};
use core::str::FromStr;

#[doc(hidden)]
pub fn parse_0_args<'a>(_func_name: &'a str, args: &'a str) -> Result<'a, ()> {
    let args_count = args.split_terminator(',').count();

    if args_count != 0 {
        return Err(InteractiveError::WrongNumberOfArguments {
            expected: 0,
            found: args_count,
        });
    }
    Ok(())
}

#[doc(hidden)]
pub fn parse_1_arg<'a, T0: FromStr>(func_name: &'a str, args: &'a str) -> Result<'a, (T0,)> {
    let args_count = args.split_terminator(',').count();

    if args_count != 1 {
        return Err(InteractiveError::WrongNumberOfArguments {
            expected: 1,
            found: args_count,
        });
    }
    let mut args_iterator = args.split_terminator(',');
    let arg0 = parse_arg(func_name, args, &mut args_iterator)?;
    Ok((arg0,))
}

#[doc(hidden)]
pub fn parse_2_args<'a, T0: FromStr, T1: FromStr>(
    func_name: &'a str,
    args: &'a str,
) -> Result<'a, (T0, T1)> {
    let args_count = args.split_terminator(',').count();

    if args_count != 2 {
        return Err(InteractiveError::WrongNumberOfArguments {
            expected: 2,
            found: args_count,
        });
    }
    let mut args_iterator = args.split_terminator(',');
    let arg0 = parse_arg(func_name, args, &mut args_iterator)?;
    let arg1 = parse_arg(func_name, args, &mut args_iterator)?;
    Ok((arg0, arg1))
}

#[doc(hidden)]
pub fn parse_3_args<'a, T0: FromStr, T1: FromStr, T2: FromStr>(
    func_name: &'a str,
    args: &'a str,
) -> Result<'a, (T0, T1, T2)> {
    let args_count = args.split_terminator(',').count();

    if args_count != 3 {
        return Err(InteractiveError::WrongNumberOfArguments {
            expected: 3,
            found: args_count,
        });
    }
    let mut args_iterator = args.split_terminator(',');
    let arg0 = parse_arg(func_name, args, &mut args_iterator)?;
    let arg1 = parse_arg(func_name, args, &mut args_iterator)?;
    let arg2 = parse_arg(func_name, args, &mut args_iterator)?;
    Ok((arg0, arg1, arg2))
}

#[doc(hidden)]
pub fn parse_arg<'a, T: FromStr>(
    _func_name: &'a str,
    args: &'a str,
    args_iterator: &mut impl Iterator<Item = &'a str>,
) -> Result<'a, T> {
    args_iterator
        .next()
        .unwrap() // length was already checked
        .trim()
        .parse()
        .map_err(|_| InteractiveError::ArgsError { given_args: args })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(parse_2_args("", "1, 2").unwrap(), (1f32, 2f32));
    }
}
