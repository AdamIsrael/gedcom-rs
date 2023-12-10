// use crate::types::{Address, Line, Source};
// use super::types::Line;
use super::types::*;

use std::fs::File;

use std::io::{self, BufRead};
use std::path::Path;


// use nom::character::complete::newline;
// use nom::character::complete::digit1;

use nom::{
    // sequence::delimited,
    // character::complete::char,
    // bytes::complete::is_not,
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
// use nom::character::complete::{
//     alphanumeric1,
//     // alpha1,
//     // one_of,
//     line_ending,
//     // multispace0,
//     // multispace1,
//     not_line_ending,
//     space0,
// };
// use nom::combinator::{
//     // all_consuming,
//     // map,
//     // map_opt,
//     map_res,
//     opt,
//     peek,
//     recognize,
//     verify,
// };
// use nom::sequence::{
//     delimited,
//     // pair,
//     preceded,
//     // separated_pair,
//     // terminated,
//     // tuple,
// };
// use nom::ParseTo;

// /// A line of GEDCOM data
// type Line<'a> = (
//     u8,              // level
//     Option<&'a str>, // xref
//     Option<&'a str>, // tag
//     Option<&'a str>, // value
// );


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

    (buffer, line) = Line::parse(input).unwrap();
    text += line.value;

    (_, line) = Line::peek(buffer).unwrap();
    while line.tag == "CONC" || line.tag == "CONT" {
        // consume
        (buffer, line) = Line::parse(buffer).unwrap();

        // allocate
        text += line.value;
        if line.tag == "CONT" {
            text += "\n";
        }

        // peek ahead
        (_, line) = Line::peek(buffer).unwrap();
    }

    Ok((buffer, Some(text)))
}

/// Parse the buffer if the CONC tag is found and return the resulting string.
pub fn conc(input: &str) -> IResult<&str, &str> {
    let (buffer, line) = Line::parse(input).unwrap();

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
    let (buffer, line) = Line::parse(input).unwrap();

    if line.tag == "CONT" {
        Ok((buffer, line.value))
    } else {
        Ok((buffer, ""))
    }
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
                    let (buff, line) = Line::peek(&record).unwrap();

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
