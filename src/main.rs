use chrono::NaiveDate;
pub use nom::bytes::complete::tag;
use nom::{
    bytes::complete::take_while,
    character::complete::digit1,
    combinator::map_res,
    error::{ErrorKind, FromExternalError, ParseError},
    IResult,
};

// fn parse_date(input: &str) -> IResult<&str, &str> {
//     let parsed_date = input.parse::<NaiveDate>();

//     if parsed_date.is_ok() {
//         let parsed_date_str = parsed_date.unwrap().to_string();

//         return Ok((&parsed_date_str, &input[..parsed_date_str.len()]));
//     } else {
//         let parsed_date_err = parsed_date.unwrap_err();

//         return Err(nom::Err::Error(&parsed_date_err.to_string()));
//     }
// }

// fn parse_date<'t>() -> impl Parser<&'t str, NaiveDate, ()> {
//     move |input: &'t str| match input.parse::<NaiveDate>() {
//         Ok(tail) => Ok((&input[..tail.to_string().len()], tail)),
//         Err(_) => Err(nom::Err::Error(())),
//     }
// }

fn parse_date<'a, E: FromExternalError<&'a str, chrono::ParseError>>(
    i: &'a str,
) -> IResult<&'a str, NaiveDate, E> {
    take_while(|s: &str| s.parse::<NaiveDate>())(i)
        s.
}

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    take_while(move |c| " \t\r\n".contains(c))(i)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date = "    2021-01-01        ";

    // let (leftover_input, output) = parse_date("1-1-2021")?;
    // println!("parsed: {}", output);
    // println!("unparsed: {}", leftover_input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use nom::sequence::delimited;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_date() {
        let mut parser = delimited(sp, parse_date, sp);
        let date = "    2021-01-01       ";

        let (_, parsed) = parser(date).unwrap();

        assert_eq!(parsed.to_string(), "2021-01-01");
        println!()
    }
}
