// use std::str::FromStr;
use std::fmt;

use winnow::ascii::{alphanumeric1, digit1, line_ending, space0, till_line_ending};
use winnow::combinator::{opt, preceded, separated_pair};
use winnow::error::StrContext;
use winnow::prelude::*;
use winnow::stream::Stream;
use winnow::token::{literal, take_till};

/// A GEDCOM line
/// level + delim (space) + \[optional_xref_ID\] + tag + \[optional_line_value\] + terminator
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Line<'a> {
    pub level: u8,
    pub xref: &'a str,
    pub tag: &'a str,
    pub value: &'a str,
}

impl<'b> fmt::Display for Line<'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.xref.is_empty() {
            write!(
                f,
                "{} {} {} {}",
                self.level, self.xref, self.tag, self.value
            )
        } else {
            write!(f, "{} {} {}", self.level, self.tag, self.value)
        }
    }
}
impl<'b> Line<'b> {
    pub fn parse(input: &mut &'b str) -> PResult<Line<'b>> {
        let mut line = Line {
            level: 0,
            xref: "",
            tag: "",
            value: "",
        };
        // println!("Parsing line...");
        // println!("Starting input: '{}'", input);
        if !input.is_empty() {
            // We could rewrite this into a sequence of parsers, something like this:
            // let (level, _, xref, _, tag, delim, value) = (
            //     Self::level,
            //     Self::delim,
            //     Self::xref,
            //     Self::delim,
            //     Self::tag,
            //     Self::delim,
            //     Self::value,
            // )
            //     .parse_next(input).unwrap();

            let level = Self::level(input);
            match level {
                Ok(lvl) => {
                    line.level = lvl;
                    let _ = Self::delim(input);
                    match Self::xref(input) {
                        Ok(xref) => {
                            line.xref = xref;
                        }
                        Err(_e) => {
                            // Failed to parse xref - treat as no xref present
                            // This handles malformed xref like "@INVALID" without closing @
                            line.xref = "";
                        }
                    }
                    if !line.xref.is_empty() {
                        let _ = Self::delim(input);
                    }
                    line.tag = Self::tag(input)?;
                    let _ = Self::delim(input);

                    let is_eol = Self::peek_eol(input)?;
                    if is_eol {
                        Self::eol(input)?;
                    } else {
                        Self::delim(input)?;
                        line.value = Self::value(input)?;

                        let is_eol = Self::peek_eol(input)?;
                        if is_eol {
                            Self::eol(input)?;
                        }
                    }
                }
                Err(_e) => {
                    let _ = Self::eol(input);
                    /*
                    There's a case where a line is simply the extension of the
                    previous line because of an embedded newline. This is common
                    in Ancestry source data, IME. Technically, it's incorrect
                    according to spec; the data should use a CONC/CONT to indicate
                    a break on a new line.

                    What we can attempt to do is parse the line as the value, as
                    if it were a CONCatonation. We don't have a line level, nor
                    do we know what the previous line is, so we'll set it to
                    u8::MAX, I guess, and add a special use-case for that.
                     */

                    // line.level = u8::MAX;
                    // line.tag = "CONC";
                    // line.value = Self::value(input)?;
                    // println!("New value: '{:?}'", line);

                    // there's a case where the value of a line contains a newline,
                    // breaking it into its own line. I think it's techically
                    // invalid, according to spec; it should use CONC/CONT.
                    // It's common in Ancestry source data so may as well work
                    // to handle it.
                }
            }
        } else {
            // There's a few instances where we're passed an empty input.
            // This might be a parsing error, but might not be. More testing!
            // println!("Empty input");
        }
        Ok(line)
    }

    /// Peek ahead at the next line without consuming it.
    pub fn peek(input: &mut &'b str) -> PResult<Line<'b>> {
        let start = input.checkpoint();
        let line = Line::parse(input)?;

        input.reset(start);
        Ok(line)
    }

    /// Parse a number from the string, but return it as an actual Rust number, not a string.
    fn level(input: &mut &str) -> PResult<u8> {
        // parse_to works because it uses FromStr, which is effectively
        // a convienence function around try_map
        // digit1.try_map(str::parse).parse_next(input)
        digit1
            .context(StrContext::Label("level"))
            .parse_to()
            .parse_next(input)
    }

    // Parse a number from the string, but return it as an actual Rust number, not a string.
    // fn peek_level<'s>(input: &mut &'s str) -> PResult<u8> {
    //     let start = input.checkpoint();
    //
    //     let level = Self::level(input).unwrap();
    //     input.reset(start);
    //     Ok(level)
    // }

    /// Parse the delimiter
    fn delim(input: &mut &'b str) -> PResult<&'b str> {
        space0.context(StrContext::Label("delim")).parse_next(input)
    }

    fn eol(input: &mut &'b str) -> PResult<&'b str> {
        // multispace0.context(StrContext::Label("eol2")).parse_next(input)
        line_ending
            .context(StrContext::Label("eol"))
            .parse_next(input)

        // println!("EOL start input: '{}'", input);
        // let res = line_ending.context(StrContext::Label("eol")).parse_next(input);
        // println!("EOL end input: '{}'", input);

        // res
    }

    /// Peek at the next character to see if it's a newline
    fn peek_eol(input: &mut &'b str) -> PResult<bool> {
        if input.starts_with('\n') || input.starts_with("\r\n") {
            return Ok(true);
        }

        // let start = input.checkpoint();
        // let res = Self::eol(input);
        // input.reset(start);

        // if !res.is_err() {
        //     let is_eol = res.unwrap();
        //     return Ok(!is_eol.is_empty());
        // }
        Ok(false)
        // let is_eol = Self::eol(input).unwrap();

        // input.reset(start);
        // Ok(!is_eol.is_empty())
    }

    fn tag(input: &mut &'b str) -> PResult<&'b str> {
        // one of: a-zA-Z_
        let parser = preceded(opt(literal("_")), alphanumeric1)
            .recognize()
            .verify(|o: &str| o.len() <= 31);

        parser.context(StrContext::Label("tag")).parse_next(input)
    }

    fn value(input: &mut &'b str) -> PResult<&'b str> {
        till_line_ending
            .context(StrContext::Label("value"))
            .parse_next(input)
    }

    /// Parse the xref, if present
    ///
    /// TODO: Return the leading/trailing @ portion of the xref
    fn xref(input: &mut &'b str) -> PResult<&'b str> {
        if input.starts_with('@') {
            let mut parser =
                separated_pair(literal("@"), take_till(0.., |c| c == '@'), literal("@"))
                    .recognize();
            return parser.parse_next(input);

            // println!("Parsing xref: '{}'", input);
            // let mut parser = delimited(tag("@"), take_till(0.., |c| c == '@'), tag("@"));
            // let res = parser.context(StrContext::Label("xref")).parse_next(input);

            // if !res.is_err() {
            //     let mut xref = res.unwrap();
            //     xref += "@";
            //     return Ok("@1@");
            // }
            // take_till(1.., |c| c == '@').parse_next(input)
            // let mut parser = delimited(
            //     tag("@"),
            //     is_not("@"),
            //     tag("@"),
            // );

            // parser(input)
        }
        Ok("")
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lines() {
        let mut data = vec![
            "0 HEAD",
            "1 CHAR UTF-8",
            "1 SOUR Ancestry.com Family Trees",
            "2 DATA Name of source data",
            "3 DATE 1 JAN 1998",
            "3 COPR Copyright of source data",
            "1 SUBM @U1@",
            "0 @U1@ SUBM",
        ];

        let line = Line::parse(&mut data[0]).unwrap();
        assert!(line.level == 0 && line.tag == "HEAD");

        let line = Line::parse(&mut data[1]).unwrap();
        assert!(line.level == 1 && line.tag == "CHAR" && line.value == "UTF-8");

        let line = Line::parse(&mut data[2]).unwrap();
        assert!(line.level == 1 && line.tag == "SOUR" && line.value == "Ancestry.com Family Trees");

        let line = Line::parse(&mut data[3]).unwrap();
        assert!(line.level == 2 && line.tag == "DATA" && line.value == "Name of source data");

        let line = Line::parse(&mut data[4]).unwrap();
        assert!(line.level == 3 && line.tag == "DATE" && line.value == "1 JAN 1998");

        let line = Line::parse(&mut data[5]).unwrap();
        assert!(line.level == 3 && line.tag == "COPR" && line.value == "Copyright of source data");

        let line = Line::parse(&mut data[6]).unwrap();
        assert!(line.level == 1 && line.tag == "SUBM" && line.value == "@U1@");

        let line = Line::parse(&mut data[7]).unwrap();
        // TODO: Update this to include the wrapping @ when I figure out how to make nom do that.
        assert!(line.level == 0 && line.tag == "SUBM" && line.value == "" && line.xref == "@U1@");
    }

    #[test]
    fn test_parse_line_with_xref_in_tag() {
        let mut input = "0 @I1@ INDI\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 0);
        assert_eq!(line.xref, "@I1@");
        assert_eq!(line.tag, "INDI");
        assert_eq!(line.value, "");
    }

    #[test]
    fn test_parse_line_with_xref_in_value() {
        let mut input = "1 FAMC @F1@\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 1);
        assert_eq!(line.xref, "");
        assert_eq!(line.tag, "FAMC");
        assert_eq!(line.value, "@F1@");
    }

    #[test]
    fn test_parse_line_no_value() {
        let mut input = "0 HEAD\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 0);
        assert_eq!(line.tag, "HEAD");
        assert_eq!(line.value, "");
    }

    #[test]
    fn test_parse_line_with_spaces_in_value() {
        let mut input = "1 NAME John    /Doe/\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 1);
        assert_eq!(line.tag, "NAME");
        assert_eq!(line.value, "John    /Doe/");
    }

    #[test]
    fn test_parse_line_with_newline() {
        let mut input = "1 NAME Test\n2 GIVN First\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 1);
        assert_eq!(line.tag, "NAME");
        assert_eq!(line.value, "Test");
        // Input should be advanced to next line
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 2);
    }

    #[test]
    fn test_parse_line_with_crlf() {
        let mut input = "1 NAME Test\r\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 1);
        assert_eq!(line.tag, "NAME");
        assert_eq!(line.value, "Test");
    }

    #[test]
    fn test_parse_empty_input() {
        let mut input = "";
        let line = Line::parse(&mut input).unwrap();
        // Empty input returns default line
        assert_eq!(line.level, 0);
        assert_eq!(line.tag, "");
    }

    #[test]
    fn test_parse_custom_tag() {
        let mut input = "1 _CUSTOM Value\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 1);
        assert_eq!(line.tag, "_CUSTOM");
        assert_eq!(line.value, "Value");
    }

    #[test]
    fn test_parse_numeric_tag() {
        let mut input = "1 DATE1 Value\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 1);
        assert_eq!(line.tag, "DATE1");
        assert_eq!(line.value, "Value");
    }

    #[test]
    fn test_peek_line() {
        let mut input = "1 NAME Test\n2 GIVN First\n";
        let peeked = Line::peek(&mut input).unwrap();
        assert_eq!(peeked.level, 1);
        assert_eq!(peeked.tag, "NAME");

        // Input should not be consumed
        let parsed = Line::parse(&mut input).unwrap();
        assert_eq!(parsed.level, 1);
        assert_eq!(parsed.tag, "NAME");
    }

    #[test]
    fn test_line_display_with_xref() {
        let line = Line {
            level: 0,
            xref: "@I1@",
            tag: "INDI",
            value: "",
        };
        assert_eq!(format!("{}", line), "0 @I1@ INDI ");
    }

    #[test]
    fn test_line_display_without_xref() {
        let line = Line {
            level: 1,
            xref: "",
            tag: "NAME",
            value: "John /Doe/",
        };
        assert_eq!(format!("{}", line), "1 NAME John /Doe/");
    }

    #[test]
    fn test_line_default() {
        let line = Line::default();
        assert_eq!(line.level, 0);
        assert_eq!(line.xref, "");
        assert_eq!(line.tag, "");
        assert_eq!(line.value, "");
    }

    #[test]
    fn test_line_eq() {
        let line1 = Line {
            level: 1,
            xref: "",
            tag: "NAME",
            value: "Test",
        };
        let line2 = Line {
            level: 1,
            xref: "",
            tag: "NAME",
            value: "Test",
        };
        assert_eq!(line1, line2);
    }

    #[test]
    fn test_line_clone() {
        let line1 = Line {
            level: 1,
            xref: "@I1@",
            tag: "INDI",
            value: "Value",
        };
        let line2 = line1.clone();
        assert_eq!(line1, line2);
    }

    #[test]
    fn test_parse_high_level_number() {
        let mut input = "15 TAG Value\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.level, 15);
        assert_eq!(line.tag, "TAG");
    }

    #[test]
    fn test_parse_line_with_trailing_spaces() {
        let mut input = "1 NAME Test   \n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.value, "Test   ");
    }

    #[test]
    fn test_parse_line_multiple_spaces_before_value() {
        let mut input = "1 NAME    Test\n";
        let line = Line::parse(&mut input).unwrap();
        assert_eq!(line.value, "Test");
    }
}
