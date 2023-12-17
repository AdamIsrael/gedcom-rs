// use crate::types::{Address, Line, Source};
// use super::types::Line;
use super::types::*;

use std::fs::File;

use std::io::{self, BufRead};
use std::path::Path;

use winnow::prelude::*;

/// This is pretty much a kludge to strip out U+FEFF, a Zero Width No-Break Space
/// https://www.compart.com/en/unicode/U+FEFF
///
/// So far, I've only seen this with one GEDCOM, as the starting byte.
// pub fn zero_with_no_break_space(input: &mut &str) -> PResult<&str> {
//     if input.starts_with('\u{FEFF}') {
//         let parser = tag("\u{FEFF}");

//         parser.parse_next(input)
//     } else {
//         Ok("")
//     }
// }

/// What did I mean to do with this? gg
/// I think it takes the input and returns a tuple containing the tag and it's
/// optional value? I lost the thread, though, and need to retrace my steps.
// fn get_tag_value(input: &str) -> IResult<&str, (&str, &str)> {

//     Ok((input, ("", "")))
// }

/// Read the next tag's value and any continuations
pub fn get_tag_value(input: &mut &str) -> PResult<Option<String>> {
    let mut line = Line::parse(input).unwrap();

    // Seed the value with the initial value
    let mut text: String = line.value.to_string();

    line = Line::peek(input).unwrap();
    while line.tag == "CONC" || line.tag == "CONT" {
        // consume
        line = Line::parse(input).unwrap();

        if line.tag == "CONT" {
            text += "\n";
        } else {
            text += " ";
        }
        text += line.value;

        // peek ahead
        line = Line::peek(input).unwrap();
    }

    Ok(Some(text))
}

/// Parse the buffer if the CONC tag is found and return the resulting string.
// pub fn conc(input: &mut &str) -> PResult<Option<String>> {
//     let line = Line::parse(input).unwrap();

//     if line.tag == "CONC" {
//         Ok(Some(line.value.to_string()))
//     } else {
//         Ok(None)
//     }
// }

/// Parse the buffer if the CONT tag is found and return the resulting string.
/// TODO: Refactor this. It should handle CONT and CONC.
// pub fn cont(input: &mut &str) -> PResult<Option<String>> {
//     let line = Line::parse(input).unwrap();

//     if line.tag == "CONT" {
//         Ok(Some(line.value.to_string()))
//     } else {
//         Ok(None)
//     }
// }

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
            // println!("Buffer: \n'{}'", buffer);
            // record = buffer.clone() + "\n";

            if let Some(ch) = buffer.chars().next() {
                if ch == '0' && !record.is_empty() {
                    let mut input: &str = record.as_str();

                    // Peek at the first line in the record so we know how
                    // to parse it.
                    let line = Line::peek(&mut input).unwrap();
                    // println!("Got a line: {:?}", line);
                    match line.tag {
                        "HEAD" => {
                            // println!("Parsing HEAD: \n{}", input);
                            gedcom.header = Header::parse(input.to_string());
                        }
                        "INDI" => {
                            let indi = Individual::parse(&mut input);
                            // TODO: Remove the if. This is just to clean up the output for debugging.
                            // if indi.xref.clone().unwrap() == "@I1@" {
                            gedcom.individuals.push(indi);
                            // }
                        }
                        "SOUR" => {}
                        "REPO" => {}
                        "OBJE" => {
                            // let obj = Object::parse(buff);
                            // println!("{:?}", obj);
                        }
                        "FAM" => {}
                        "SUBM" => {
                            // // The record of the submitter of the family tree
                            // // Not always present (it exists in complete.ged)
                            // if let Some(ref subm) = gedcom.header.submitter {
                            //     if let Some(xref) = &subm.xref {
                            //         gedcom.header.submitter =
                            //             Submitter::find_by_xref(buff, xref.to_string());
                            //     }
                            // }
                        }
                        _ => {}
                    };

                    record.clear();
                }
                record = record + &buffer.clone() + "\n";
                // println!("Record: {:?}", record);
            }

            // match Line::peek(&mut linebuff) {
            //     Ok(line) => {
            //         if line.level == 0 && line.tag == "HEAD" {
            //             // Consume the line
            //             Line::parse(&mut linebuff).unwrap();
            //         } else if line.level == 1 {
            //             // println!("Found an inner tag: {}", line.tag);
            //             match line.tag {
            //                 "CHAR" => {
            //                     gedcom.header.encoding = Some(line.value.to_string());
            //                     Line::parse(&mut linebuff).unwrap();
            //                 }
            //                 "INDI" => {
            //                     let indi = Individual::parse(buff.to_string());
            //                     // TODO: Remove the if. This is just to clean up the output for debugging.
            //                     if indi.xref.clone().unwrap() == "@I1@" {
            //                         gedcom.individuals.push(indi);
            //                     }
            //                 }
            //                 "SOUR" => {}
            //                 "REPO" => {}
            //                 "OBJE" => {
            //                     let obj = Object::parse(buff);
            //                     println!("{:?}", obj);
            //                 }
            //                 "FAM" => {}
            //                 "SUBM" => {
            //                     // The record of the submitter of the family tree
            //                     // Not always present (it exists in complete.ged)
            //                     if let Some(ref subm) = gedcom.header.submitter {
            //                         if let Some(xref) = &subm.xref {
            //                             gedcom.header.submitter =
            //                                 Submitter::find_by_xref(buff, xref.to_string());
            //                         }
            //                     }
            //                 }
            //                 _ => {
            //                     // println!("Unhandled header tag: {}", line.tag);
            //                     // (_, _) = Line::parse(&buffer).unwrap();
            //                 }
            //             };
            //         // } else {
            //         //     (_, _) = Line::parse(&buffer).unwrap();
            //         }

            //         // println!("line: {:?}", line);

            //     }
            //     Err(_e) => {
            //         println!("Error parsing line: '{}'", buffer);
            //     }
            // }

            // if let Some(ch) = buffer.chars().next() {
            //     if ch == '0' && !record.is_empty() {
            //         // We found a new record, beginning with buffer, so
            //         // process the data in `record` before continuing

            //         // Peek at the next line to see where we're at.
            //         // let (buff, line) = Line::peek(&record).unwrap();
            //         let line = Line::peek(record).unwrap();

            //         match line.tag {
            //             "HEAD" => {
            //                 gedcom.header = Header::parse(buff.to_string());
            //             }
            //             "INDI" => {
            //                 let indi = Individual::parse(buff.to_string());
            //                 // TODO: Remove the if. This is just to clean up the output for debugging.
            //                 if indi.xref.clone().unwrap() == "@I1@" {
            //                     gedcom.individuals.push(indi);
            //                 }
            //             }
            //             "SOUR" => {}
            //             "REPO" => {}
            //             "OBJE" => {
            //                 let obj = Object::parse(buff);
            //                 println!("{:?}", obj);
            //             }
            //             "FAM" => {}
            //             "SUBM" => {
            //                 // The record of the submitter of the family tree
            //                 // Not always present (it exists in complete.ged)
            //                 if let Some(ref subm) = gedcom.header.submitter {
            //                     if let Some(xref) = &subm.xref {
            //                         gedcom.header.submitter =
            //                             Submitter::find_by_xref(buff, xref.to_string());
            //                     }
            //                 }
            //             }
            //             _ => {}
            //         };

            //         record.clear();
            //     }
            // }
            // record = record + &buffer.clone() + "\n";
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_tag_value() {
        let mut input = "3 ADDR 1300 West Traverse Parkway\n4 CONT Lehi, UT 84043\n4 CONC USA";
        let output = "1300 West Traverse Parkway\nLehi, UT 84043 USA";

        let res = get_tag_value(&mut input).unwrap();
        if let Some(value) = res {
            assert!(output == value);
        }
        assert!(input.len() == 0);
    }
}
