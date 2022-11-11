#![allow(dead_code, unused)]

use chrono::NaiveDate;
pub use nom::bytes::complete::tag;
use nom::{
    bytes::complete::take_while,
    character::{complete::digit1, is_digit},
    combinator::{map_res, recognize},
    error::{ErrorKind, FromExternalError},
    sequence::tuple,
    IResult,
};

fn parse_date<'a>(i: &'a str) -> IResult<&'a str, NaiveDate> {
    map_res(
        recognize(tuple((digit1, tag("-"), digit1, tag("-"), digit1))),
        |s: &str| s.parse::<NaiveDate>(),
    )(i)
}

fn sp<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    take_while(move |c| " \t\r\n".contains(c))(i)
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
    fn test_sp() {
        let (_, o) = sp(" ").unwrap();
        assert_eq!(o, " ");
    }

    #[test]
    fn test_parse_date() {
        let date = "2021-01-01";
        let (_, o) = parse_date(date).unwrap();
        assert_eq!(o, NaiveDate::from_ymd(2021, 1, 1));
    }

    #[test]
    fn test_fail_parse_date() {
        let date = "2021-01-32";
        let (_, o) = parse_date(date).unwrap();
        assert_eq!(o, NaiveDate::from_ymd(2021, 1, 1));
    }
}
