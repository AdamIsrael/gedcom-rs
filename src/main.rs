extern crate gedcom_rs;

use gedcom_rs::parse::{parse_gedcom, GedcomConfig};

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse flags and filename
    let mut config = GedcomConfig::new();
    let mut filename: Option<&String> = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--help" | "-h" => usage(""),
            "--verbose" | "-v" => config.verbose = true,
            _ => {
                if filename.is_some() {
                    usage(&format!("Unexpected argument: {}", arg));
                }
                filename = Some(arg);
            }
        }
    }

    let filename = match filename {
        Some(f) => f,
        None => {
            usage("Missing filename.");
            unreachable!()
        }
    };

    match parse_gedcom(filename, &config) {
        Ok(gedcom) => {
            // TODO: print a pretty summary of the gedcom. Use `tabled` crate?
            println!("{:#?}", gedcom);
        }
        Err(e) => {
            eprintln!("Error parsing GEDCOM file: {}", e);
            process::exit(1);
        }
    }
}

fn usage(msg: &str) {
    if !msg.is_empty() {
        println!("{msg}");
    }
    println!("Usage: gedcom-test [OPTIONS] <FILE>");
    println!();
    println!("Arguments:");
    println!("  <FILE>  Path to the GEDCOM file to parse");
    println!();
    println!("Options:");
    println!("  -v, --verbose    Show detailed encoding warnings and diagnostics");
    println!("  -h, --help       Show this help message");
    std::process::exit(0x0100);
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use gedcom_rs::parse::{parse_gedcom, GedcomConfig};

    #[test]
    fn test_complete_gedcom() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // Test the header
        // println!("Gedcom: {:?}", gedcom.header);
        // Test the copyright header
        assert!(gedcom.header.copyright.is_some());
        let copyright = gedcom.header.copyright.unwrap();
        assert!(
            copyright == "© 1997 by H. Eichmann, parts © 1999-2000 by J. A. Nairn.".to_string()
        );

        // Test the note field
        assert!(gedcom.header.note.is_some());
        let note = gedcom.header.note.unwrap();
        assert!(note.starts_with("This file demonstrates all tags that are allowed in GEDCOM 5.5."));
        assert!(note.ends_with("GEDCOM 5.5 specs on the Internet at <http://homepages.rootsweb.com/~pmcbride/gedcom/55gctoc.htm>."));
    }

    #[test]
    fn test_file_not_found() {
        let result = parse_gedcom("./nonexistent.ged", &GedcomConfig::new());
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, gedcom_rs::error::GedcomError::FileNotFound(_)));
        }
    }

    #[test]
    fn test_source_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 2 SOURCE records
        assert_eq!(gedcom.sources.len(), 2);

        // Test first source record
        let s1 = &gedcom.sources[0];
        assert!(s1.xref.is_some());
        assert_eq!(s1.xref.as_ref().unwrap().to_string(), "@S1@");
        assert!(s1.title.is_some());
        let title = s1.title.as_ref().unwrap();
        assert!(title.starts_with("Everything You Every Wanted to Know about GEDCOM Tags"));
        assert!(title.contains("Were Afraid to Ask!"));
        assert_eq!(s1.abbreviation.as_deref(), Some("All About GEDCOM Tags"));

        // Test second source record
        let s2 = &gedcom.sources[1];
        assert!(s2.xref.is_some());
        assert_eq!(s2.xref.as_ref().unwrap().to_string(), "@S2@");
        assert_eq!(
            s2.title.as_deref(),
            Some("All I Know About GEDCOM, I Learned on the Internet")
        );
        assert_eq!(s2.abbreviation.as_deref(), Some("What I Know About GEDCOM"));
        assert_eq!(s2.author.as_deref(), Some("Second Source Author"));
    }

    #[test]
    fn test_note_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 27 NOTE records
        assert_eq!(gedcom.notes.len(), 27);

        // Test first note record
        let n1 = &gedcom.notes[0];
        assert!(n1.xref.is_some());
        assert_eq!(n1.xref.as_ref().unwrap().to_string(), "@N1@");
        assert_eq!(
            n1.note,
            "Test link to a graphics file about the main Submitter of this file."
        );

        // Test second note record (has source citation and multi-line content)
        let n2 = &gedcom.notes[1];
        assert_eq!(n2.xref.as_ref().unwrap().to_string(), "@N2@");
        assert!(n2.note.contains("Family History Library"));
        assert!(n2.note.contains("REPOSITORY Record"));
        assert!(n2.note.contains("Are they all imported?"));
        assert_eq!(n2.source_citations.len(), 1);
        assert_eq!(n2.source_citations[0].xref, Some("@S1@".to_string()));
    }

    #[test]
    fn test_multimedia_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 16 MULTIMEDIA records
        assert_eq!(gedcom.multimedia.len(), 16);

        // Test first multimedia record
        let m1 = &gedcom.multimedia[0];
        assert!(m1.xref.is_some());
        assert_eq!(m1.xref.as_ref().unwrap().to_string(), "@M1@");
        assert_eq!(m1.files.len(), 1);
        assert_eq!(m1.files[0].file_reference, "photo.jpeg");
        assert_eq!(m1.files[0].format.as_deref(), Some("JPEG"));
        assert_eq!(m1.files[0].media_type.as_deref(), Some("photo"));
        assert_eq!(
            m1.files[0].title.as_deref(),
            Some("Picture of the book cover")
        );

        // Test second multimedia record
        let m2 = &gedcom.multimedia[1];
        assert_eq!(m2.xref.as_ref().unwrap().to_string(), "@M2@");
    }

    #[test]
    fn test_repository_parsing() {
        let gedcom = parse_gedcom("./data/complete.ged", &GedcomConfig::new()).unwrap();

        // complete.ged has 1 REPOSITORY record
        assert_eq!(gedcom.repositories.len(), 1);

        // Test the repository record
        let r1 = &gedcom.repositories[0];
        assert!(r1.xref.is_some());
        assert_eq!(r1.xref.as_ref().unwrap().to_string(), "@R1@");
        assert_eq!(r1.name, Some("Family History Library".to_string()));

        // Check address
        assert!(r1.address.is_some());
        let addr = r1.address.as_ref().unwrap();
        assert_eq!(addr.addr1.as_deref(), Some("35 North West Temple"));
        assert_eq!(addr.city.as_deref(), Some("Salt Lake City"));
        assert_eq!(addr.state.as_deref(), Some("UT"));

        // Check phones (part of address)
        assert_eq!(addr.phone.len(), 3);
        assert_eq!(addr.phone[0], "+1-801-240-2331");

        // Check note reference
        assert_eq!(r1.notes.len(), 1);
        assert_eq!(r1.notes[0].note, Some("@N2@".to_string()));
    }

    // #[test]
    // /// Tests a possible bug in Ancestry's format, if a line break is embedded within the content of a note
    // /// As far as I can tell, it's a \n embedded into the note, at least, from a hex dump of that content.
    // fn newline_in_note() {
    //     let data = vec![
    //         "0 @S313871942@ SOUR",
    //         "1 TITL Germany, Lutheran Baptisms, Marriages, and Burials, 1567-1945",
    //         "1 AUTH Ancestry.com",
    //         "1 PUBL Ancestry.com Operations, Inc.",
    //         "1 NOTE <p>Mikrofilm Sammlung.  Familysearch.org</p>",
    //         "<p>Originale:  Lutherische Kirchenbücher, 1567-1945. Various sources.</p>",
    //         "1 _APID 1,61250::0",
    //     ];

    //     // assert_eq!(expected, line("\r")("0 HEAD\r").unwrap());
    //     // assert_eq!(expected, line("\n")("0 HEAD\n").unwrap());
    //     // assert_eq!(expected, line("\r\n")("0 HEAD\r\n").unwrap());
    // }
}
