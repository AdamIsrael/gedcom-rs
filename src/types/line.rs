use std::str::FromStr;

use nom::{bytes::complete::is_not, IResult};

use nom::character::complete::{alphanumeric1, digit1, line_ending, not_line_ending, space0};
use nom::combinator::{
    map_res,
    opt,
    // peek,
    recognize,
    verify,
};
use nom::sequence::{delimited, preceded};

/// A GEDCOM line
/// level + delim (space) + [optional_xref_ID] + tag + [optional_line_value] + terminator
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Line<'a> {
    pub level: u8,
    pub xref: &'a str,
    pub tag: &'a str,
    pub value: &'a str,
}

impl<'b> Line<'b> {
    /// Peek at the next line.
    pub fn peek(input: &str) -> IResult<&str, Line> {
        let (_, l) = Line::parse(input).unwrap();

        Ok((input, l))
    }
    /// Parse a single line of GEDCOM data
    pub fn parse(input: &str) -> IResult<&str, Line> {
        /*
        New strategy: let the wookie win.

        Parse the whole line out of the input first. Then parse the individual elements
        out into variables, present or not.

        Lastly, return the input minus our line, and a tuple representing level, xref, tag, value

        */

        // let mut _level: u8 = 0;
        // let mut _xref: &str = "";
        // let mut _tag: &str = "";
        // let mut _value: &str = "";
        let is_eol: bool;
        let mut tmp: &str = "";
        // let mut _value: &str = "";
        let mut line = Line {
            level: 0,
            xref: "",
            tag: "",
            value: "",
        };

        if !input.is_empty() {
            match Line::level(input) {
                Ok((mut buffer, _level)) => {
                    line.level = _level;

                    (buffer, _) = Line::delim(buffer).unwrap();
                    (buffer, line.xref) = Line::xref(buffer).unwrap();
                    (buffer, line.tag) = Line::tag(buffer.trim_start()).unwrap();
                    (buffer, _) = Line::delim(buffer).unwrap();
                    (buffer, is_eol) = Line::peek_eol(buffer).unwrap();

                    if !is_eol {
                        (buffer, _) = Line::delim(buffer).unwrap();

                        (buffer, line.value) = Line::value(buffer).unwrap();
                    }

                    tmp = Line::eol(buffer).unwrap_or((buffer, "")).0;
                }
                Err(e) => {
                    println!("Error parsing level: {}", e);
                }
            }
        }
        Ok((tmp, line))

        // Ok((tmp, (_level, Some(_xref), Some(_tag), Some(_value))))
    }

    /// Parse a number from the string, but return it as an actual Rust number, not a string.
    fn level(input: &str) -> IResult<&str, u8> {
        let mut parser = map_res(digit1, u8::from_str);
        parser(input)
    }

    /// Parse the delimiter
    fn delim(input: &str) -> IResult<&str, &str> {
        space0(input)
    }

    /// Parse the xref, if present
    ///
    /// TODO: Return the leading/trailing @ portion of the xref
    fn xref(input: &str) -> IResult<&str, &str> {
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

    fn tag(input: &str) -> IResult<&str, &str> {
        // one of: a-zA-Z_
        // alpha1(input)
        verify(
            recognize(preceded(opt(nom::bytes::complete::tag("_")), alphanumeric1)),
            |o: &str| o.len() <= 31,
        )(input)
    }

    fn value(input: &str) -> IResult<&str, &str> {
        not_line_ending(input)
    }

    fn eol(input: &str) -> IResult<&str, &str> {
        line_ending(input)
    }

    /// Peek at the next character to see if it's a newline
    fn peek_eol(input: &str) -> IResult<&str, bool> {
        let (input, is_eol) = Line::eol(input).unwrap_or((input, ""));
        Ok((input, !is_eol.is_empty()))
    }

    // /// Peek the level of the next line
    // fn peek_level(input: &str) -> IResult<&str, u8> {
    //     let mut parser = map_res(peek(digit1), u8::from_str);
    //     parser(input)
    // }

    // /// Peek at the tag in the next line.
    // ///
    // /// This allows us to check what the next tag is so we can determine if we need
    // /// to keep processing, i.e., a CONT.
    // fn peek_tag(input: &str) -> IResult<&str, &str> {
    //     let (_, l) = Line::parse(input).unwrap();

    //     Ok((input, l.tag))
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lines() {
        let data = vec![
            "0 HEAD",
            "1 CHAR UTF-8",
            "1 SOUR Ancestry.com Family Trees",
            "2 DATA Name of source data",
            "3 DATE 1 JAN 1998",
            "3 COPR Copyright of source data",
            "1 SUBM @U1@",
        ];

        let (_, line) = Line::parse(data[0]).unwrap();
        assert!(line.level == 0 && line.tag == "HEAD");

        let (_, line) = Line::parse(data[1]).unwrap();
        assert!(line.level == 1 && line.tag == "CHAR" && line.value == "UTF-8");

        let (_, line) = Line::parse(data[2]).unwrap();
        assert!(line.level == 1 && line.tag == "SOUR" && line.value == "Ancestry.com Family Trees");

        let (_, line) = Line::parse(data[3]).unwrap();
        assert!(line.level == 2 && line.tag == "DATA" && line.value == "Name of source data");

        let (_, line) = Line::parse(data[4]).unwrap();
        assert!(line.level == 3 && line.tag == "DATE" && line.value == "1 JAN 1998");

        let (_, line) = Line::parse(data[5]).unwrap();
        assert!(line.level == 3 && line.tag == "COPR" && line.value == "Copyright of source data");

        let (_, line) = Line::parse(data[6]).unwrap();
        assert!(line.level == 1 && line.tag == "SUBM" && line.value == "@U1@");

    }
}
