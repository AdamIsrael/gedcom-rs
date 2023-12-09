use super::Line;
use crate::parse;
// use crate::types::address::Address;
use crate::types::{Address, DateTime, Note};

// n @<XREF:SUBM>@ SUBM {1:1}
// +1 NAME <SUBMITTER_NAME> {1:1} p.63
// +1 <<ADDRESS_STRUCTURE>> {0:1}* p.31
// +1 <<MULTIMEDIA_LINK>> {0:M} p.37, 26
// +1 LANG <LANGUAGE_PREFERENCE> {0:3} p.51
// +1 RFN <SUBMITTER_REGISTERED_RFN> {0:1} p.63
// +1 RIN <AUTOMATED_RECORD_ID> {0:1} p.43
// +1 <<NOTE_STRUCTURE>> {0:M} p.37
// +1 <<CHANGE_DATE>>

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Submitter {
    /// The pointer to the SUBM record
    pub xref: Option<String>,
    pub name: Option<String>,
    pub address: Option<Address>,
    pub media: Vec<String>,
    /// A list of languages in which the submitter prefers to communicate
    /// listed in order of priority.
    pub lang: Vec<String>,
    pub rfn: Option<String>,
    pub rin: Option<String>,
    pub note: Option<Note>,
    pub change_date: Option<DateTime>,
}

impl Submitter {
    pub fn find_by_xref(mut buffer: &str, xref: String) -> Option<Submitter> {
        // println!("find_by_xref::buffer: {:?}", buffer);
        let mut submitter = Submitter {
            xref: Some(xref),
            name: None,
            address: None,
            media: vec![],
            lang: vec![],
            rfn: None,
            rin: None,
            note: None,
            change_date: None,
        };
        let mut line: Line;
        (_, line) = parse::peek_line(buffer).unwrap();

        while !buffer.is_empty() {
            // this is only going to match one line. We want to skip forward
            // until we reach this line, and then process until we hit either EOF or a new 0 level
            // if line.level == 0 && xref == line.xref.unwrap() {
            if line.level == 0 {
                // Peek at the next line so we know how to parse it.
                (_, line) = parse::peek_line(buffer).unwrap();

                // Loop through the rest of the record
                while line.level > 0 || !buffer.is_empty() {
                    match line.tag {
                        "NAME" => {
                            submitter.name = Some(line.value.to_string());
                            (buffer, _) = parse::line(buffer).unwrap();
                        }
                        "ADDR" => {
                            (buffer, submitter.address) = Address::parse(buffer);
                        }
                        "OBJE" => {
                            // Parse the object id and add it to the list
                            let media_xref = line.value;
                            submitter.media.push(media_xref.to_string());
                            (buffer, _) = parse::line(buffer).unwrap();
                            // TODO: find the media object and parse it
                        }
                        "RIN" => {
                            (buffer, line) = parse::line(buffer).unwrap();
                            submitter.rin = Some(line.value.to_string());
                            // println!("!! {:}", line.tag);
                        }
                        "CHAN" => {
                            // Parse the date/time
                            (buffer, _) = parse::line(buffer).unwrap();
                            (buffer, submitter.change_date) = DateTime::parse(buffer);
                        }
                        "LANG" => {
                            let lang = line.value;
                            submitter.lang.push(lang.to_string());
                            (buffer, _) = parse::line(buffer).unwrap();
                        }
                        "NOTE" => {
                            (buffer, submitter.note) = Note::parse(buffer);
                        }
                        "RFN" => {
                            let rfn = line.value;
                            submitter.rfn = Some(rfn.to_string());
                            (buffer, _) = parse::line(buffer).unwrap();
                        }
                        _ => {
                            // Advance the buffer past the unknown line
                            (buffer, _) = parse::line(buffer).unwrap();
                        }
                    }
                    (_, line) = parse::peek_line(buffer).unwrap();
                }
            } else {
                (buffer, line) = parse::line(buffer).unwrap();
            }
        }

        Some(submitter)
    }

    /// Parses a SUBM block
    pub fn parse(mut buffer: &str) -> (&str, Option<Submitter>) {
        let mut submitter = Submitter {
            xref: None,
            name: None,
            address: None,
            media: vec![],
            lang: vec![],
            rfn: None,
            rin: None,
            note: None,
            change_date: None,
        };

        let mut line: Line;

        (_, line) = parse::peek_line(buffer).unwrap();
        if line.level == 1 && line.tag == "SUBM" {
            // advance our position in the buffer
            (buffer, line) = parse::line(buffer).unwrap();
            // This is a temporary hack, because parse::xref strips @ from the id
            let xref = line.value;
            submitter.xref = Some(xref.to_owned());
        }

        (buffer, Some(submitter))
    }
}

#[cfg(test)]
mod tests {
    use super::Submitter;

    #[test]
    fn parse_submitter() {
        let data = vec![
            "1 SUBM @U1@",
            // other records that we need to skip over
            "1 FILE TGC55C.ged",
            "1 COPR © 1997 by H. Eichmann, parts © 1999-2000 by J. A. Nairn.",
            // The submitter record
            "0 @U1@ SUBM",
            "1 NAME Adam Israel",
            "1 ADDR",
            "2 ADR1 Example Software",
            "2 ADR2 123 Main Street",
            "2 ADR3 Ste 1",
            "2 CITY Anytown",
            "2 STAE IL",
            "2 POST 55555",
            "2 CTRY USA",
            "1 PHON +1-800-555-1111",
            "1 PHON +1-800-555-1212",
            "1 PHON +1-800-555-1313",
            "1 EMAIL a@@example.com",
            "1 EMAIL b@@example.com",
            "1 EMAIL c@@example.com",
            "1 FAX +1-800-555-1414",
            "1 FAX +1-800-555-1515",
            "1 FAX +1-800-555-1616",
            "1 WWW https://www.example.com",
            "1 WWW https://www.example.org",
            "1 WWW https://www.example.net",
            "1 OBJE @M1@",
            "1 RFN 123456789",
            "1 RIN 1",
            "1 NOTE This is a test note.",
            "2 CONT And so is this.",
            "1 CHAN",
            "2 DATE 7 SEP 2000",
            "3 TIME 8:35:36",
            "1 LANG English",
            "1 LANG German",
        ];

        let (_, mut submitter) = Submitter::parse(data.join("\n").as_str());
        let xref = submitter.unwrap().xref;
        // println!("GOT xref: {:?}", xref);
        // let header = Header::parse(data.join("\n"));
        // Now, find the xref
        submitter = Submitter::find_by_xref(data.join("\n").as_str(), xref.unwrap());
        // println!("Submitter: {:?}", submitter);
        let s = submitter.unwrap();

        assert!(s.xref == Some("@U1@".to_string()));
        assert!(s.name == Some("Adam Israel".to_string()));

        let addr = s.address.unwrap();

        assert!(addr.addr1 == Some("Example Software".to_string()));
        assert!(addr.addr2 == Some("123 Main Street".to_string()));
        assert!(addr.addr3 == Some("Ste 1".to_string()));
        assert!(addr.city == Some("Anytown".to_string()));
        assert!(addr.state == Some("IL".to_string()));
        assert!(addr.postal_code == Some("55555".to_string()));
        assert!(addr.country == Some("USA".to_string()));
        assert!(addr.phone.contains(&"+1-800-555-1111".to_string()));
        assert!(addr.phone.contains(&"+1-800-555-1212".to_string()));
        assert!(addr.phone.contains(&"+1-800-555-1313".to_string()));
        assert!(addr.email.contains(&"a@@example.com".to_string()));
        assert!(addr.email.contains(&"b@@example.com".to_string()));
        assert!(addr.email.contains(&"c@@example.com".to_string()));
        assert!(addr.fax.contains(&"+1-800-555-1414".to_string()));
        assert!(addr.fax.contains(&"+1-800-555-1515".to_string()));
        assert!(addr.fax.contains(&"+1-800-555-1616".to_string()));
        assert!(addr.www.contains(&"https://www.example.com".to_string()));
        assert!(addr.www.contains(&"https://www.example.org".to_string()));
        assert!(addr.www.contains(&"https://www.example.net".to_string()));

        // TODO: Make sure this resolves to a Media record
        assert!(s.media.contains(&"@M1@".to_string()));

        assert!(s.lang.contains(&"English".to_string()));
        assert!(s.lang.contains(&"German".to_string()));

        assert!(s.rin == Some("1".to_string()));

        let date = s.change_date.unwrap();
        assert!(date.date == Some("7 SEP 2000".to_string()));
        assert!(date.time == Some("8:35:36".to_string()));

        // TODO: Implement these once the fields are implemented.
        assert!(s.rfn == Some("123456789".to_string()));
        println!("{:?}", s.note);
        let note = s.note.unwrap().note.unwrap();
        assert!(note.starts_with("This is a test note."));
        assert!(note.ends_with("And so is this.\n"));
    }
}
