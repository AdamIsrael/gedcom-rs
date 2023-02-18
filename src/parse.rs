// use crate::types::{Address, Line, Source};

use std::str::FromStr;

// use nom::character::complete::newline;
use nom::character::complete::digit1;

use nom::{
    // sequence::delimited,
    // character::complete::char,
    bytes::complete::is_not,
    IResult,
};

// use nom::{
//     error::{make_error, ErrorKind, ParseError},
//     Needed,
// };
// use nom::{
//     AsChar, Compare, CompareResult, ExtendInto, InputIter, InputLength, InputTake,
//     InputTakeAtPosition, Offset, Slice,
// };
// use nom::branch::alt;

// use nom::bytes::complete::{take_while, take_while1};
use nom::character::complete::{
    alphanumeric1,
    // alpha1,
    // one_of,
    line_ending,
    multispace0,
    multispace1,
    not_line_ending,
};
use nom::combinator::{
    // all_consuming,
    // map,
    // map_opt,
    map_res,
    opt,
    peek,
    recognize,
    verify,
};
use nom::sequence::{
    delimited,
    // pair,
    preceded,
    // separated_pair,
    // terminated,
    tuple,
};
// use nom::ParseTo;
// use nom::sequence::Tuple;

/// Peek the level of the next line
pub fn peek_level(input: &str) -> IResult<&str, u8> {
    let mut parser = map_res(peek(digit1), u8::from_str);
    parser(input)
}

type Line<'a> = (
    u8,
    &'a str,
    Option<&'a str>,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
);

/// Parse a single line
pub fn line(input: &str) -> IResult<&str, Line> {
    // pub fn line(input: &str) -> IResult<&str, (u8, &str, Option<&str>, &str, &str, &str, &str)> {
    // println!("Input: '{input}'");
    // 2 TIME 16:56:08

    let mut parser = tuple((
        // level
        map_res(digit1, u8::from_str),
        // delim
        multispace1,
        // xref
        opt(delimited(
            nom::bytes::complete::tag("@"),
            is_not("@"),
            nom::bytes::complete::tag("@"),
        )),
        // tag
        verify(
            recognize(preceded(opt(nom::bytes::complete::tag("_")), alphanumeric1)),
            |o: &str| o.len() <= 31,
        ),
        // todo: this is either going to be a space followed by a value,
        // or a newline, and the way it's written now it's going to consume the newline
        // and treat the next line as this line's value.
        // Need to peek the next character; if it's a newline, we're done. Otherwise,
        // continue to parse the line as we have it below.

        // delim
        multispace0,
        // value
        not_line_ending,
        // eol
        line_ending,
    ));

    parser(input)
}

// fn source(input: &str) -> IResult<&str, Source> {
//     let source = Source {
//         source: "".to_string(),
//         name: None,
//         version: None,
//         address: None
//     };

//     // return IResult.ok(input, source);
// }

// fn parse_level(input: &str) -> IResult<&str, &str> {
//     digit1(input)
// }

/// This is pretty much a kludge to strip out U+FEFF, a Zero Width No-Break Space
/// https://www.compart.com/en/unicode/U+FEFF
///
/// So far, I've only seen this with one GEDCOM, as the starting byte.
pub fn zero_with_no_break_space(input: &str) -> IResult<&str, &str> {
    if input.starts_with('\u{FEFF}') {
        let parser = nom::bytes::complete::tag("\u{FEFF}");

        parser(input)
    } else {
        Ok((input, ""))
    }
}

/// Parse a number from the string, but return it as an actual Rust number, not a string.
pub fn level(input: &str) -> IResult<&str, u8> {
    // pub fn level(input: &str) -> IResult<&str, u8> {
    // let (input, _) = all_consuming(tag("\u{FEFF}"))(input)?;
    // println!("Input: {input}");
    let mut parser = map_res(digit1, u8::from_str);
    parser(input)
}

/// Parse the delimiter
pub fn delim(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

/// Parse the xref, if present
///
/// TODO: Return the leading/trailing @ portion of the xref
pub fn xref(input: &str) -> IResult<&str, &str> {
    if input.starts_with('@') {
        let mut parser = delimited(
            nom::bytes::complete::tag("@"),
            is_not("@"),
            nom::bytes::complete::tag("@"),
        );
        parser(input)
    } else {
        Ok((input, ""))
    }
}

pub fn tag(input: &str) -> IResult<&str, &str> {
    // one of: a-zA-Z_
    // alpha1(input)
    verify(
        recognize(preceded(opt(nom::bytes::complete::tag("_")), alphanumeric1)),
        |o: &str| o.len() <= 31,
    )(input)
}

pub fn value(input: &str) -> IResult<&str, &str> {
    not_line_ending(input)
}

pub fn eol(input: &str) -> IResult<&str, &str> {
    line_ending(input)
}
