#![allow(dead_code, unused_imports, unused)]

use chrono::NaiveDate;
pub use nom::bytes::complete::tag;
use nom::{
    bytes::complete::{escaped_transform, take_while},
    character::{
        complete::{digit1, none_of},
        is_digit,
    },
    combinator::{map_parser, map_res, recognize},
    error::{ErrorKind, FromExternalError},
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

fn parse_date<'a>(i: &'a str) -> IResult<&'a str, NaiveDate> {
    map_res(
        recognize(tuple((digit1, tag("-"), digit1, tag("-"), digit1))),
        |s: &str| s.parse::<NaiveDate>(),
    )(i)
}

fn parse_quoted<'a>(i: &'a str) -> IResult<&'a str, String> {
    delimited(
        tag("\""),
        map_parser(
            recognize(separated_list0(tag("\"\""), many0(none_of("\"")))),
            escaped_transform(none_of("\""), '\"', tag("\"")),
        ),
        tag("\""),
    )(i)
}

#[derive(Debug)]
struct TxEntry<'a> {
    account: &'a str,
    amount: f64,
    currency: &'a str,
    // cost: f64,
    // cost_currency: &'a str,
}

fn parse_tx_entry<'a>(i: &'a str) -> IResult<&'a str, TxEntry> {
    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date = "    2021-01-01        ";

    let x = parse_date(date);

    // let (leftover_input, output) = parse_date("1-1-2021")?;
    // println!("parsed: {}", output);
    // println!("unparsed: {}", leftover_input);
    Ok(())
}

#[cfg(test)]
mod parser {
    use super::*;

    #[test]
    fn test_parse_date() {
        let date = "2021-01-01";
        let (_, o) = parse_date(date).unwrap();
        assert_eq!(o, NaiveDate::from_ymd(2021, 1, 1));
    }

    #[test]
    fn test_parse_quoted() {
        let s = "\"*(21)\"";
        let (_, o) = parse_quoted(s).unwrap();
        assert_eq!(o, "*(21)");
    }

    #[test]
    fn test_parse_quoted_failure() {
        let s = "x\"sssss\"";
        let res = parse_quoted(s);
        assert!(res.is_err());
    }
}
