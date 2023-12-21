// use std::str::FromStr;
use std::fmt;

use winnow::ascii::{alphanumeric1, digit1, line_ending, not_line_ending, space0};
use winnow::combinator::{opt, preceded, separated_pair};
use winnow::error::StrContext;
use winnow::prelude::*;
use winnow::stream::Stream;
use winnow::token::{tag, take_till};

/// A GEDCOM line
/// level + delim (space) + [optional_xref_ID] + tag + [optional_line_value] + terminator
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Line<'a> {
    pub level: u8,
    pub xref: &'a str,
    pub tag: &'a str,
    pub value: &'a str,
}

impl<'b> fmt::Display for Line<'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.xref.is_empty() {
            write!(f, "{} {} {} {}", self.level, self.xref, self.tag, self.value)
        }
        else {
            write!(f, "{} {} {}", self.level,self.tag, self.value)
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
                            todo!();
                        }
                    }
                    if !line.xref.is_empty() {
                        let _ = Self::delim(input);
                    }
                    line.tag = Self::tag(input)?;
                    let _ = Self::delim(input);

                    let is_eol = Self::peek_eol(input)?;
                    if is_eol {
                        Self::eol(input).unwrap();
                    } else {
                        Self::delim(input).unwrap();
                        line.value = Self::value(input)?;

                        let is_eol = Self::peek_eol(input)?;
                        if is_eol {
                            Self::eol(input).unwrap();
                        }
                    }
                }
                Err(e) => {
                    println!("Err: {}", e);
                    println!("Error parsing line: '{}'", input);
                    Self::eol(input).unwrap();
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
        let line = Line::parse(input).unwrap();

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

    /// Parse a number from the string, but return it as an actual Rust number, not a string.
    // fn peek_level<'s>(input: &mut &'s str) -> PResult<u8> {
    //     let start = input.checkpoint();

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
        let parser = preceded(opt(tag("_")), alphanumeric1)
            .recognize()
            .verify(|o: &str| o.len() <= 31);

        parser.context(StrContext::Label("tag")).parse_next(input)
    }

    fn value(input: &mut &'b str) -> PResult<&'b str> {
        not_line_ending
            .context(StrContext::Label("value"))
            .parse_next(input)
    }

    /// Parse the xref, if present
    ///
    /// TODO: Return the leading/trailing @ portion of the xref
    fn xref(input: &mut &'b str) -> PResult<&'b str> {
        if input.starts_with('@') {
            let mut parser =
                separated_pair(tag("@"), take_till(0.., |c| c == '@'), tag("@")).recognize();
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
}
