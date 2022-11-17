#![allow(dead_code, unused_imports, unused)]

// pub use nom::bytes::complete::tag;
// use nom::{
//     bytes::complete::{escaped_transform, take_while},
//     character::{
//         complete::{digit1, none_of},
//         is_digit,
//     },
//     combinator::{map_parser, map_res, recognize},
//     error::{ErrorKind, FromExternalError},
//     multi::{many0, separated_list0},
//     sequence::{delimited, tuple},
//     IResult,
// };

// fn parse_quoted<'a>(i: &'a str) -> IResult<&'a str, String> {
//     delimited(
//         tag("\""),
//         map_parser(
//             recognize(separated_list0(tag("\"\""), many0(none_of("\"")))),
//             escaped_transform(none_of("\""), '\"', tag("\"")),
//         ),
//         tag("\""),
//     )(i)
// }

// #[derive(Debug)]
// struct TxEntry<'a> {
//     account: &'a str,
//     amount: f64,
//     currency: &'a str,
//     // cost: f64,
//     // cost_currency: &'a str,
// }

// fn parse_tx_entry<'a>(i: &'a str) -> IResult<&'a str, TxEntry> {
//     todo!()
// }

use std::{num::ParseIntError, str::FromStr};

use nom::{
    bytes::complete::take,
    character::{
        complete::{digit1, one_of, satisfy},
        is_digit,
    },
    combinator::{map_parser, map_res, recognize},
    error::{self, ErrorKind, FromExternalError, ParseError},
    multi::{fold_many1, many1},
    sequence::terminated,
    IResult, Parser,
};

struct Date {
    year: u32,
    month: u8,
    day: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum DateErrorKind {
    UnparseableYear,
    InvalidYear,
    IncorrectFormat,
    InvalidMonth,
    InvalidDay,
}

#[derive(Debug, PartialEq)]
pub enum LgrError<I> {
    DateError(DateErrorKind),
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for LgrError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        LgrError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

fn parse_date_yyyy<'a>(i: &'a str) -> IResult<&'a str, u8> {
    //satisfy(|c: &str| c.len() == 4)(i);

    todo!();

    //map_res(map_parser(take(4u8), digit1), |s: &str| s.parse::<u8>())(i)
}

fn parse_date_mm<'a>(i: &'a str) -> IResult<&'a str, u8> {
    (map_res(map_parser(take(2u8), digit1), |o: &str| o.parse::<u8>()))(i)

    // let is_valid_month = month > 0 || month <= 12;
    // if is_valid_month {
    //     Ok((rest, month))
    // } else {
    //     Err(nom::Err::Error(LgrError::DateError(
    //         DateErrorKind::InvalidMonth,
    //     )))
    // }

    // let (rest, o): (&str, &str) = p(i)?;

    // match o.parse::<u8>() {
    //     Ok(n) => {
    //         let is_valid_month = n > 0 || n <= 12;
    //         if is_valid_month {
    //             Ok((rest, n))
    //         } else {
    //         }
    //     }

    //     Err(e) => Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
    //         i,
    //         ErrorKind::Digit,
    //     ))),
    // }

    // Ok((rest, o.parse::<u8>()?))
}

//fn parse_date<'a>(i: &'a str) -> IResult<&'a str, Date, LgrError<&str>> {
//     let year_parser = map_res(recognize(digit1), i.parse);
//     todo!()
// }

// match i.parse::<NaiveDate>() {
//     Ok(d) => Ok((i, d)),
//     Err(e) => Err(nom::Err::Error(LgrParserError::DateError(i, e.kind()))),
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date = "1000-01-01";

    let res = recognize(many1(terminated(one_of("0123456789"))));
    println!("{:?}", res);

    // let (leftover_input, output) = parse_date("1-1-2021")?;
    // println!("parsed: {}", output);
    // println!("unparsed: {}", leftover_input);
    Ok(())
}

#[cfg(test)]
mod parser {
    use super::*;

    #[test]
    fn test_parse_date() {}

    // #[test]
    // fn test_parse_quoted() {
    //     let s = "\"*(21)\"";
    //     let (_, o) = parse_quoted(s).unwrap();
    //     assert_eq!(o, "*(21)");
    // }

    // #[test]
    // fn test_parse_quoted_failure() {
    //     let s = "x\"sssss\"";
    //     let res = parse_quoted(s);
    //     assert!(res.is_err());
    // }
}
