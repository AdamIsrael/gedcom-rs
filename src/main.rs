extern crate gedcom_test;

use gedcom_test::parse;
use gedcom_test::types::*;

use std::env;
use std::fs::File;

use std::io::{self, BufRead};
use std::path::Path;

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
            // Strip off any weird leading spaces
            if buffer.strip_prefix('\u{FEFF}').is_some() {
                buffer.remove(0);
            }

            if let Some(ch) = buffer.chars().next() {
                if ch == '0' && !record.is_empty() {
                    // We found a new record, beginning with buffer, so
                    // process the data in `record` before continuing

                    // Peek at the next line to see where we're at.
                    let (buff, line) = parse::peek_line(&record).unwrap();

                    match line.tag {
                        "HEAD" => {
                            gedcom.header = Header::parse(buff.to_string());
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
                        "SUBM" => {}
                        _ => {}
                    };

                    record.clear();
                }
            }
            record = record + &buffer.clone() + "\n";
        }

        println!("{:#?}", gedcom.header);
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

fn usage(msg: &str) {
    if !msg.is_empty() {
        println!("{msg}");
    }
    println!("Usage: gedcom-test ./path/to/gedcom.ged");
    std::process::exit(0x0100);
}

#[cfg(test)]
mod tests {
    // use super::*;

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
