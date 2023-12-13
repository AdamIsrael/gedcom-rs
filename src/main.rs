extern crate gedcom_test;

use gedcom_test::parse::parse_gedcom;

use std::env;

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

    let gedcom = parse_gedcom(filename);

    // TODO: print a pretty summary of the gedcom. Use `tabled` crate?
    println!("{:#?}", gedcom);
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
    fn test_complete_gedcom() {
        let gedcom = parse_gedcom("./data/complete.ged");

        // Test the header

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
