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
    // multispace0,
    // multispace1,
    not_line_ending,
    space0,
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
    // tuple,
};
// use nom::ParseTo;

/// A line of GEDCOM data
type Line<'a> = (
    u8,              // level
    Option<&'a str>, // xref
    Option<&'a str>, // tag
    Option<&'a str>, // value
);

/// Peek at the next character to see if it's a newline
pub fn peek_eol(input: &str) -> IResult<&str, bool> {
    let (input, is_eol) = eol(input).unwrap_or((input, ""));
    Ok((input, !is_eol.is_empty()))
}

/// Peek the level of the next line
pub fn peek_level(input: &str) -> IResult<&str, u8> {
    let mut parser = map_res(peek(digit1), u8::from_str);
    parser(input)
}

/// Peek at the tag in the next line.
///
/// This allows us to check what the next tag is so we can determine if we need
/// to keep processing, i.e., a CONT.
pub fn peek_tag(input: &str) -> IResult<&str, &str> {
    let (_, l) = line(input).unwrap();

    Ok((input, l.2.unwrap_or("")))
}

/// Parse a single line of GEDCOM data
pub fn line(input: &str) -> IResult<&str, Line> {
    /*
    New strategy: let the wookie win.

    Parse the whole line out of the input first. Then parse the individual elements
    out into variables, present or not.

    Lastly, return the input minus our line, and a tuple representing level, xref, tag, value

    */

    let mut _level: u8 = 0;
    let mut _xref: &str = "";
    let mut _tag: &str = "";
    let mut _value: &str = "";
    let is_eol: bool;
    let mut tmp: &str = "";
    let mut _value: &str = "";

    if !input.is_empty() {
        (tmp, _level) = level(input).unwrap();
        (tmp, _) = delim(tmp).unwrap();
        (tmp, _xref) = xref(tmp).unwrap();
        (tmp, _tag) = tag(tmp.trim_start()).unwrap();
        (tmp, _) = delim(tmp).unwrap();
        (tmp, is_eol) = peek_eol(tmp).unwrap();

        if !is_eol {
            (tmp, _) = delim(tmp).unwrap();

            (tmp, _value) = value(tmp).unwrap();
        }

        tmp = eol(tmp).unwrap_or((tmp, "")).0;
    }

    Ok((tmp, (_level, Some(_xref), Some(_tag), Some(_value))))
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

/// What did I mean to do with this? gg
/// I think it takes the input and returns a tuple containing the tag and it's
/// optional value? I lost the thread, though, and need to retrace my steps.
// fn get_tag_value(input: &str) -> IResult<&str, (&str, &str)> {

//     Ok((input, ("", "")))
// }

/// Parse the buffer if the CONT tag is found and return the resulting string.
/// TODO: Refactor this. It should handle CONT and CONC.
pub fn cont(input: &str) -> IResult<&str, &str> {
    // let line: (u8, Option<&str>, Option<&str>, Option<&str>);
    // let buffer: &str;
    let (buffer, line) = super::parse::line(input).unwrap();

    if line.2 == Some("CONT") {
        Ok((buffer, line.3.unwrap()))
    } else {
        Ok((buffer, ""))
    }
}

/// Parse a number from the string, but return it as an actual Rust number, not a string.
pub fn level(input: &str) -> IResult<&str, u8> {
    let mut parser = map_res(digit1, u8::from_str);
    parser(input)
}

/// Parse the delimiter
pub fn delim(input: &str) -> IResult<&str, &str> {
    space0(input)
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
