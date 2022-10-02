use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, character::complete::digit1,
    combinator::{opt, map_res}, combinator::recognize, combinator::value, error::FromExternalError,
    error::ParseError, sequence::tuple, IResult, Parser,
};

fn parser_null<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, (), E> {
    value((), tag("null")).parse(input)
}

fn parse_bool<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, bool, E> {
    alt((value(true, tag("true")), value(false, tag("false")))).parse(input)
}

fn parse_number<
    'i,
    E: ParseError<&'i str> + FromExternalError<&'i str, std::num::ParseFloatError>,
>(
    input: &'i str,
) -> IResult<&'i str, f64, E> {
    map_res(
        recognize(tuple((
            opt(char('-')),
            digit1,
            opt(tuple((char('.'), digit1))),
            opt(tuple((
                alt((char('e'), char('E'))),
                opt(alt((char('+'), char('-')))),
            ))),
        ))),
        |float_str: &'i str| float_str.parse(),
    )
    .parse(input)
}



fn main() {
    println!("Hello, world!");
}
