use gedcom_test::parse;
use gedcom_test::types::*;

use std::env;
use std::fs::File;

use std::io::{self, BufRead};
use std::path::Path;

// impl Line {
//     /// Parse a single row of GEDCOM data into a Line
//     fn parse(input: &str) -> IResult<&str, Self> {
//         let level_parser = separated_pair(digit1, char(' '), alpha1);
//         // Defines a new parser which wraps the `two_words_parser`, then
//         // passes the resulting pair into a closure.
//         let mut person_parser = map(
//             level_parser,
//             |(level, xref)| Self {
//                 level: level,
//                 xref: xref,
//                 // tag: "".to_string(),
//                 // value: Some("".to_string())
//             },
//         );
//         // Use the parser
//         person_parser(input)
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => usage("Missing filename."),
        s if s > 2 => usage(&format!("Found more args than expected: {:?}", &args[1..])),
        _ => (),
    };

    let filename = &args[1];

    if filename == "--help" || filename == "-h" {
        usage("");
    }

    // Initialize an empty gedcom
    let mut gedcom = Gedcom {
        header: Header {
            encoding: None,
            copyright: None,
            corporation: None,
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
    };

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String

        // let mut records: Vec<Line> = Vec::new();

        // Read through the lines and build a buffer of <records>, each starting
        // with a zero and ending with the last line before the next. Then feed that
        // buffer to a nom parser to split it into Lines?

        // This is kind of like a buffered read, specific to the GEDCOM format
        // We read into the buffer until we hit a new record, and then parse that
        // record into a struct.
        let mut record: String = String::new();
        for mut buffer in lines.flatten() {
            // Strip off any weird leading spaces
            if buffer.strip_prefix('\u{FEFF}').is_some() {
                buffer.remove(0);
            }
            if let Some(ch) = buffer.chars().next() {
                if ch == '0' && !record.is_empty() {
                    // We found a new record, beginning with buffer, so
                    // process the data in `record` before continuing
                    // println!("found a record: {record}");

                    // parse the first line in the record to find out what tag this is
                    // let level: u8 = 0;
                    let (tmp, _level) = parse::level(&record).unwrap();
                    let (tmp, _) = parse::delim(tmp).unwrap();
                    let (tmp, _xref) = parse::xref(tmp).unwrap();
                    let (tmp, tag) = parse::tag(tmp.trim_start()).unwrap();
                    // let (tmp, _) = parse::delim(tmp).unwrap();
                    let (tmp, _) = parse::eol(tmp).unwrap();

                    // println!("Level: {level}, xref: '{xref}', tag: '{tag}', Buffer: '{record}'");
                    // record. = tmp.to_owned();

                    match tag {
                        "HEAD" => {
                            println!("Parsing a HEAD record!");
                            // println!("record: {tmp}");
                            // gedcom.header = parse_header(record);
                            gedcom.header = Header::parse(tmp.to_string());
                        }
                        "INDI" => {}
                        "SOUR" => {}
                        "REPO" => {}
                        "OBJE" => {}
                        "FAM" => {}
                        "SUBM" => {}
                        _ => {}
                    };

                    record.clear();
                    break;
                }
            }
            record = record + &buffer.clone() + "\n";
        }

        println!("{gedcom:?}");

        //     // let (buffer, _) = parse::zero_with_no_break_space(&buffer).unwrap();

        //     let (buffer, level) = parse::level(&buffer).unwrap();

        //     let (buffer, _) = parse::delim(&buffer).unwrap();
        //     let (buffer, xref) = parse::xref(&buffer).unwrap();
        //     let (buffer, tag) = parse::tag(&buffer.trim_start()).unwrap();
        //     let (buffer, _) = parse::delim(&buffer).unwrap();
        //     // println!("Level: {level}, xref: '{xref}', tag: '{tag}', Buffer: '{buffer}'");

        //     if level == 0 && !records.is_empty() {
        //         // need to get the first row from records
        //         let record = records.get(0).unwrap();
        //         // println!("Tag: {tag}");
        //         // Need to figure out why this fails, but the general idea
        //         // is to match on the tag and call the appropriate parse fn
        //         // maybe implement a get_tag method on the struct?
        //         // println!("Records: {records:?}");

        //         match record.tag.as_str() {
        //             "HEAD" => {
        //                 // Parse all of the lines in `records` to build
        //                 // a Header struct
        //                 // let foo = records.iter().map(|r| r.level + 1);

        //                 // let last_plus_1 = lines.iter().map(|r| r.level + 1);
        //                 // println!("last plus 1: {last_plus_1:?}");

        //                 let header = Header::parse(records.clone());
        //                 println!("Header: {header:?}")
        //             },
        //             _ => {},
        //         };

        //         // Parse the record
        //         // println!("Parsing a record");
        //         records.clear();
        //     }
        //     // I've got record and records, but do I need both? Or rename one.
        //     // I need a temp buffer (Vec<Line>) that stores the lines in this record
        //     // And then I need to build the data into a more proper struct that
        //     // represents the GEDCOM data, probably set in the above `match tag`.
        //     // let line = Line {
        //     //     level: level,
        //     //     xref: Some(xref.to_string()),
        //     //     tag: tag.to_string(),
        //     //     value: Some(buffer.to_string())
        //     // };
        //     // println!("Line: {line:?}");
        //     // record.push(line);
        //     let line = Line{
        //         level: level,
        //         xref: Some(xref.to_string()),
        //         tag: tag.to_string(),
        //         value: Some(buffer.to_string()),
        //     };

        //     // println!("Adding Line {line:?} to records");
        //     records.push(line);
        // }
        // }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn usage(msg: &str) {
    if !msg.is_empty() {
        println!("{msg}");
    }
    println!("Usage: gedcom-test ./path/to/gedcom.ged");
    std::process::exit(0x0100);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests a possible bug in Ancestry's format, if a line break is embedded within the content of a note
    /// As far as I can tell, it's a \n embedded into the note, at least, from a hex dump of that content.
    fn newline_in_note() {
        let data = vec![
            "0 @S313871942@ SOUR",
            "1 TITL Germany, Lutheran Baptisms, Marriages, and Burials, 1567-1945",
            "1 AUTH Ancestry.com",
            "1 PUBL Ancestry.com Operations, Inc.",
            "1 NOTE <p>Mikrofilm Sammlung.  Familysearch.org</p>",
            "<p>Originale:  Lutherische Kirchenbücher, 1567-1945. Various sources.</p>",
            "1 _APID 1,61250::0",
        ];

        // assert_eq!(expected, line("\r")("0 HEAD\r").unwrap());
        // assert_eq!(expected, line("\n")("0 HEAD\n").unwrap());
        // assert_eq!(expected, line("\r\n")("0 HEAD\r\n").unwrap());
    }

    // #[test]
    // fn parse_addr() {
    //     let data = vec![
    //         "3 ADDR",
    //         "4 ADR1 RSAC Software",
    //         "4 ADR2 7108 South Pine Cone Street",
    //         "4 ADR3 Ste 1",
    //         "4 CITY Salt Lake City",
    //         "4 STAE UT",
    //         "4 POST 84121",
    //         "4 CTRY USA",
    //         "3 PHON +1-801-942-7768",
    //         "3 PHON +1-801-555-1212",
    //         "3 PHON +1-801-942-1148",
    //         "3 EMAIL a@@example.com",
    //         "3 EMAIL b@@example.com",
    //         "3 EMAIL c@@example.com",
    //         "3 FAX +1-801-942-7768",
    //         "3 FAX +1-801-555-1212",
    //         "3 FAX +1-801-942-1148",
    //         "3 WWW https://www.example.com",
    //         "3 WWW https://www.example.org",
    //         "3 WWW https://www.example.net",
    //     ];
    //     addr = header::parse_address(data);

    // }
}
