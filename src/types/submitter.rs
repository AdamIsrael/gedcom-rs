use crate::types::{Address, Line, Note, Object, Xref};

// SUBMITTER_RECORD:=
// n @<XREF:SUBM>@ SUBM {1:1}
// +1 NAME <SUBMITTER_NAME> {1:1} p.63
// +1 <<ADDRESS_STRUCTURE>> {0:1}* p.31
// +1 <<MULTIMEDIA_LINK>> {0:M} p.37, 26
// +1 LANG <LANGUAGE_PREFERENCE> {0:3} p.51
// +1 RFN <SUBMITTER_REGISTERED_RFN> {0:1} p.63
// +1 RIN <AUTOMATED_RECORD_ID> {0:1} p.43
// +1 <<NOTE_STRUCTURE>> {0:M} p.37
// +1 <<CHANGE_DATE>> {0:1} p.31

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Submitter {
    /// The cross-reference identifier (e.g., @U1@)
    pub xref: Xref,

    /// Name of the submitter (required)
    pub name: Option<String>,

    /// Address structure
    pub address: Option<Address>,

    /// Multimedia links
    pub multimedia_links: Vec<Object>,

    /// Language preferences (up to 3)
    pub languages: Vec<String>,

    /// Submitter registered RFN
    pub registered_rfn: Option<String>,

    /// Automated record ID
    pub automated_record_id: Option<String>,

    /// Notes
    pub notes: Vec<Note>,

    /// Change date - stores the DATE value from CHAN/DATE
    pub change_date: Option<String>,
}

impl Submitter {
    /// Parse a SUBMITTER_RECORD (level 0 SUBM record)
    pub fn parse(record: &mut &str) -> Submitter {
        let mut submitter = Submitter::default();

        let Ok(line) = Line::peek(record) else {
            return submitter;
        };

        // Must be a level 0 SUBM record
        if line.level != 0 || line.tag != "SUBM" {
            return submitter;
        }

        // Extract xref from the level 0 line
        if !line.xref.is_empty() {
            submitter.xref = Xref::new(line.xref.to_string());
        }
        let _ = Line::parse(record);

        // Parse SUBM record fields
        while !record.is_empty() {
            let Ok(line) = Line::peek(record) else {
                break;
            };

            // Stop if we hit another level-0 record
            if line.level == 0 {
                break;
            }

            // Only process direct children (level 1)
            if line.level != 1 {
                let _ = Line::parse(record);
                continue;
            }

            let mut consume = true;

            match line.tag {
                "NAME" => {
                    submitter.name = Some(line.value.to_string());
                }
                "ADDR" => {
                    if let Ok(addr) = Address::parse(record) {
                        submitter.address = Some(addr);
                    }
                    consume = false;
                }
                "OBJE" => {
                    if let Ok(obj) = Object::parse(record) {
                        submitter.multimedia_links.push(obj);
                    }
                    consume = false;
                }
                "LANG" => {
                    // GEDCOM 5.5.1 allows up to 3 language preferences
                    if submitter.languages.len() < 3 {
                        submitter.languages.push(line.value.to_string());
                    }
                }
                "RFN" => {
                    submitter.registered_rfn = Some(line.value.to_string());
                }
                "RIN" => {
                    submitter.automated_record_id = Some(line.value.to_string());
                }
                "NOTE" => {
                    if let Ok(note) = Note::parse(record) {
                        submitter.notes.push(note);
                    }
                    consume = false;
                }
                "CHAN" => {
                    // Parse CHAN structure to get DATE value
                    let level = line.level;
                    let _ = Line::parse(record);

                    while !record.is_empty() {
                        let Ok(inner_line) = Line::peek(record) else {
                            break;
                        };

                        if inner_line.level <= level {
                            break;
                        }

                        if inner_line.level == level + 1 && inner_line.tag == "DATE" {
                            submitter.change_date = Some(inner_line.value.to_string());
                        }

                        let _ = Line::parse(record);
                    }
                    consume = false;
                }
                _ => {}
            }

            if consume {
                let _ = Line::parse(record);
            }
        }

        submitter
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::Submitter;

    #[test]
    fn test_submitter_complete() {
        let data = vec![
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
            "1 EMAIL a@example.com",
            "1 EMAIL b@example.com",
            "1 EMAIL c@example.com",
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

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let submitter = Submitter::parse(&mut record);

        assert_eq!(submitter.xref.as_str(), "@U1@");
        assert_eq!(submitter.name.as_ref().unwrap(), "Adam Israel");

        let addr = submitter.address.unwrap();
        assert_eq!(addr.addr1.as_ref().unwrap(), "Example Software");
        assert_eq!(addr.addr2.as_ref().unwrap(), "123 Main Street");
        assert_eq!(addr.addr3.as_ref().unwrap(), "Ste 1");
        assert_eq!(addr.city.as_ref().unwrap(), "Anytown");
        assert_eq!(addr.state.as_ref().unwrap(), "IL");
        assert_eq!(addr.postal_code.as_ref().unwrap(), "55555");
        assert_eq!(addr.country.as_ref().unwrap(), "USA");
        assert!(addr.phone.contains(&"+1-800-555-1111".to_string()));
        assert!(addr.phone.contains(&"+1-800-555-1212".to_string()));
        assert!(addr.phone.contains(&"+1-800-555-1313".to_string()));
        assert!(addr.email.contains(&"a@example.com".to_string()));
        assert!(addr.email.contains(&"b@example.com".to_string()));
        assert!(addr.email.contains(&"c@example.com".to_string()));
        assert!(addr.fax.contains(&"+1-800-555-1414".to_string()));
        assert!(addr.fax.contains(&"+1-800-555-1515".to_string()));
        assert!(addr.fax.contains(&"+1-800-555-1616".to_string()));
        assert!(addr.www.contains(&"https://www.example.com".to_string()));
        assert!(addr.www.contains(&"https://www.example.org".to_string()));
        assert!(addr.www.contains(&"https://www.example.net".to_string()));

        assert_eq!(submitter.multimedia_links.len(), 1);
        assert_eq!(submitter.multimedia_links[0].xref.as_ref().unwrap(), "@M1@");

        assert_eq!(submitter.languages.len(), 2);
        assert!(submitter.languages.contains(&"English".to_string()));
        assert!(submitter.languages.contains(&"German".to_string()));

        assert_eq!(submitter.registered_rfn.as_ref().unwrap(), "123456789");
        assert_eq!(submitter.automated_record_id.as_ref().unwrap(), "1");

        assert_eq!(submitter.change_date.as_ref().unwrap(), "7 SEP 2000");

        assert_eq!(submitter.notes.len(), 1);
        let note = submitter.notes[0].note.as_ref().unwrap();
        assert!(note.starts_with("This is a test note."));
        assert!(note.ends_with("And so is this."));
    }

    #[test]
    fn test_submitter_minimal() {
        let data = vec!["0 @U2@ SUBM", "1 NAME John Doe"];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let submitter = Submitter::parse(&mut record);

        assert_eq!(submitter.xref.as_str(), "@U2@");
        assert_eq!(submitter.name.as_ref().unwrap(), "John Doe");
        assert_eq!(submitter.address, None);
        assert_eq!(submitter.multimedia_links.len(), 0);
        assert_eq!(submitter.languages.len(), 0);
        assert_eq!(submitter.registered_rfn, None);
        assert_eq!(submitter.automated_record_id, None);
        assert_eq!(submitter.notes.len(), 0);
        assert_eq!(submitter.change_date, None);
    }

    #[test]
    fn test_submitter_with_multiple_notes() {
        let data = vec![
            "0 @U3@ SUBM",
            "1 NAME Jane Smith",
            "1 NOTE First note",
            "1 NOTE Second note",
            "1 NOTE @N1@",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let submitter = Submitter::parse(&mut record);

        assert_eq!(submitter.xref.as_str(), "@U3@");
        assert_eq!(submitter.notes.len(), 3);
        assert_eq!(submitter.notes[0].note.as_ref().unwrap(), "First note");
        assert_eq!(submitter.notes[1].note.as_ref().unwrap(), "Second note");
        assert_eq!(submitter.notes[2].note.as_ref().unwrap(), "@N1@");
    }

    #[test]
    fn test_submitter_language_limit() {
        let data = vec![
            "0 @U4@ SUBM",
            "1 NAME Test User",
            "1 LANG English",
            "1 LANG Spanish",
            "1 LANG French",
            "1 LANG German", // This should be ignored (max 3)
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let submitter = Submitter::parse(&mut record);

        assert_eq!(submitter.languages.len(), 3);
        assert!(submitter.languages.contains(&"English".to_string()));
        assert!(submitter.languages.contains(&"Spanish".to_string()));
        assert!(submitter.languages.contains(&"French".to_string()));
        assert!(!submitter.languages.contains(&"German".to_string()));
    }

    #[test]
    fn test_submitter_with_multiple_multimedia() {
        let data = vec![
            "0 @U5@ SUBM",
            "1 NAME Media User",
            "1 OBJE @M1@",
            "1 OBJE @M2@",
            "1 OBJE @M3@",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let submitter = Submitter::parse(&mut record);

        assert_eq!(submitter.multimedia_links.len(), 3);
        assert_eq!(submitter.multimedia_links[0].xref.as_ref().unwrap(), "@M1@");
        assert_eq!(submitter.multimedia_links[1].xref.as_ref().unwrap(), "@M2@");
        assert_eq!(submitter.multimedia_links[2].xref.as_ref().unwrap(), "@M3@");
    }
}
