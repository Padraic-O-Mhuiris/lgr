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
    bytes::{complete::take, streaming::tag},
    character::{
        complete::{digit1, one_of, satisfy},
        is_digit, is_hex_digit,
    },
    combinator::{map, map_parser, map_res, recognize},
    error::{self, ErrorKind, FromExternalError, ParseError},
    multi::{fold_many1, fold_many_m_n, many1},
    sequence::{terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

fn parse_numeric<'a>(l: usize) -> impl FnMut(&'a str) -> IResult<&'a str, u32> {
    move |i: &'a str| {
        map_res(
            recognize(fold_many_m_n(
                l,
                l,
                satisfy(|c| c.is_digit(10)),
                || (),
                |(), _c| (),
            )),
            u32::from_str,
        )(i)
    }
}

// fn parse_date<'a>(i: &'a str) -> IResult<&'a str, Date> {
//     let mut yyyy_parser = parse_numeric(4);
//     let mut mm_parser = parse_numeric(2);
//     let mut dd_parser = parse_numeric(2);

//     map_res(
//         tuple((yyyy_parser, tag("-"), mm_parser, tag("-"), dd_parser)),
//         |(yyyy, _, mm, _, dd)| {
//             dbg!(yyyy);
//             dbg!(mm);
//             dbg!(dd);
//             Ok((
//                 "",
//                 Date {
//                     year: yyyy,
//                     month: mm,
//                     day: dd,
//                 },
//             ))
//         },
//     )
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date = "1000-01-01";
    let res = parse_numeric(4)(date);
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
