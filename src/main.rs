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

fn sp<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    take_while(move |c| " \t\r\n".contains(c))(i)
}

// fn parse_title<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
//     delimited(tag('"'), ,tag('"'))
// }

fn parse_quoted(input: &str) -> IResult<&str, String> {
    let seq = recognize(separated_list0(tag("\"\""), many0(none_of("\""))));
    let unquote = escaped_transform(none_of("\""), '\"', tag("\""));
    let res = delimited(tag("\""), map_parser(seq, unquote), tag("\""))(input)?;

    Ok(res)
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
    use nom::{error::ParseError, Err};

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
        let err = parse_date(date).unwrap_err();
        assert_eq!(
            err,
            Err() // nom::Err::Error(Err::Error(()){
                  //     input: "2021-01-32",
                  //     code: ErrorKind::MapRes
                  // })
        );
    }
}
