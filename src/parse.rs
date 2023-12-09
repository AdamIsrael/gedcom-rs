// use crate::types::{Address, Line, Source};
// use super::types::Line;
use super::types::*;

use std::fs::File;

use std::io::{self, BufRead};
use std::path::Path;

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

// /// A line of GEDCOM data
// type Line<'a> = (
//     u8,              // level
//     Option<&'a str>, // xref
//     Option<&'a str>, // tag
//     Option<&'a str>, // value
// );

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

    Ok((input, l.tag))
}

// /// nop -- just testing
// pub fn nop(input: &str) -> IResult<&str, Line> {
//     let line = Line {
//         level: 0,
//         xref: Some("adsf"),
//         tag: "HEAD",
//         value: Some("a value"),
//     };

//     Ok(("", line))
// }

/// Peek at the next line.
pub fn peek_line(input: &str) -> IResult<&str, Line> {
    let (_, l) = line(input).unwrap();

    Ok((input, l))
}

/// Parse a single line of GEDCOM data
pub fn line(input: &str) -> IResult<&str, Line> {
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
        match level(input) {
            Ok((mut buffer, _level)) => {
                line.level = _level;

                (buffer, _) = delim(buffer).unwrap();
                (buffer, line.xref) = xref(buffer).unwrap();
                (buffer, line.tag) = tag(buffer.trim_start()).unwrap();
                (buffer, _) = delim(buffer).unwrap();
                (buffer, is_eol) = peek_eol(buffer).unwrap();

                if !is_eol {
                    (buffer, _) = delim(buffer).unwrap();

                    (buffer, line.value) = value(buffer).unwrap();
                }

                tmp = eol(buffer).unwrap_or((buffer, "")).0;
            }
            Err(e) => {
                println!("Error parsing level: {}", e);
            }
        }
        // (tmp, line.level) = level(input).unwrap();
        // (tmp, _) = delim(tmp).unwrap();
        // (tmp, line.xref) = xref(tmp).unwrap();
        // (tmp,  line.tag) = tag(tmp.trim_start()).unwrap();
        // (tmp, _) = delim(tmp).unwrap();
        // (tmp, is_eol) = peek_eol(tmp).unwrap();

        // if !is_eol {
        //     (tmp, _) = delim(tmp).unwrap();

        //     (tmp, line.value) = value(tmp).unwrap();
        // }

        // tmp = eol(tmp).unwrap_or((tmp, "")).0;
    }
    // println!("End of buffer: '{}'", tmp);
    Ok((tmp, line))

    // Ok((tmp, (_level, Some(_xref), Some(_tag), Some(_value))))
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

/// Read the next tag's value and any continuations
pub fn get_tag_value(input: &str) -> IResult<&str, Option<String>> {
    let mut text: String = String::from("");
    let mut line;
    let mut buffer;

    (buffer, line) = super::parse::line(input).unwrap();
    text += line.value;

    (_, line) = super::parse::peek_line(buffer).unwrap();
    while line.tag == "CONC" || line.tag == "CONT" {
        // consume
        (buffer, line) = super::parse::line(buffer).unwrap();

        // allocate
        text += line.value;
        if line.tag == "CONT" {
            text += "\n";
        }

        // peek ahead
        (_, line) = super::parse::peek_line(buffer).unwrap();
    }

    Ok((buffer, Some(text)))
}

/// Parse the buffer if the CONC tag is found and return the resulting string.
pub fn conc(input: &str) -> IResult<&str, &str> {
    let (buffer, line) = super::parse::line(input).unwrap();

    if line.tag == "CONC" {
        Ok((buffer, line.value))
    } else {
        Ok((buffer, ""))
    }
}

/// Parse the buffer if the CONT tag is found and return the resulting string.
/// TODO: Refactor this. It should handle CONT and CONC.
pub fn cont(input: &str) -> IResult<&str, &str> {
    // let line: (u8, Option<&str>, Option<&str>, Option<&str>);
    // let buffer: &str;
    let (buffer, line) = super::parse::line(input).unwrap();

    if line.tag == "CONT" {
        Ok((buffer, line.value))
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

/// Parse a GEDCOM file
pub fn parse_gedcom(filename: &str) -> Gedcom {
    // Initialize an empty gedcom
    let mut gedcom = Gedcom {
        header: Header {
            encoding: None,
            copyright: None,
            date: None,
            destination: None,
            gedcom_version: None,
            language: None,
            filename: None,
            note: None,
            source: None,
            submitter: None,
            submission: None,
        },
        individuals: vec![],
    };

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String

        // Read through the lines and build a buffer of <records>, each starting
        // with a zero and ending with the last line before the next. Then feed that
        // buffer to a nom parser to split it into Lines?

        // This is kind of like a buffered read, specific to the GEDCOM format
        // We read into the buffer until we hit a new record, and then parse that
        // record into a struct.
        let mut record: String = String::new();

        for mut buffer in lines.flatten() {
            // Strip off any leading Zero Width No-Break Space
            if buffer.strip_prefix('\u{FEFF}').is_some() {
                buffer.remove(0);
            }

            if let Some(ch) = buffer.chars().next() {
                if ch == '0' && !record.is_empty() {
                    // We found a new record, beginning with buffer, so
                    // process the data in `record` before continuing

                    // Peek at the next line to see where we're at.
                    let (buff, line) = peek_line(&record).unwrap();

                    match line.tag {
                        "HEAD" => {
                            gedcom.header = Header::parse(buff.to_string());
                            // If a SUBM is found, find the record
                            // if gedcom.header.submitter.is_some()
                            //     && gedcom.header.submitter.unwrap().xref.is_some()
                            // {
                            //     // let xref = gedcom.header.submitter.unwrap().xref.unwrap();
                            //     let xref = "@U1@".to_string();
                            //     // Create a copy of the buffer
                            //     let mut foo = String::from("");

                            //     for l in lines.flatten() {
                            //         foo += l.as_str();
                            //     }
                            //     gedcom.header.submitter = Submitter::find_by_xref(&foo, xref)
                            // }
                        }
                        "INDI" => {
                            let indi = Individual::parse(buff.to_string());
                            // TODO: Remove the if. This is just to clean up the output for debugging.
                            if indi.xref.clone().unwrap() == "I1" {
                                gedcom.individuals.push(indi);
                            }
                        }
                        "SOUR" => {}
                        "REPO" => {}
                        "OBJE" => {}
                        "FAM" => {}
                        "SUBM" => {
                            // The record of the submitter of the family tree
                            // Not always present (it exists in complete.ged)

                            // TODO: Need to fix the parsing of xref to not strip off the @
                            if line.xref == "U1" {
                                let subm = gedcom.header.submitter.clone();
                                if subm.is_some() && subm.unwrap().xref.is_some() {
                                    gedcom.header.submitter =
                                        Submitter::find_by_xref(buff, "@U1@".to_string());
                                }
                            }
                        }
                        _ => {}
                    };

                    record.clear();
                }
            }
            record = record + &buffer.clone() + "\n";
        }

        // println!("{:#?}", gedcom.header);
        // TODO: print a pretty summary of the gedcom. Use `tabled` crate?

        // TODO: should gedcom.header.submitter be a Vec? Can there be more than
        // one submitter?
        // println!("\tsubmitters: {}", 1);
        // println!("\tindividuals: {}", gedcom.individuals.len());
        // TODO: families
        // TODO: repositories
        // TODO: sources
        // TODO: multimedia
    }
    gedcom
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
