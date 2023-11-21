use crate::parse;
// use crate::types::corporation;
// use crate::types::Copyright;
// use crate::types::Note;
use crate::types::{Source, Submitter};

use super::DateTime;
use super::Gedc;
use super::Line;

/*
HEADER:= n HEAD
+1 SOUR <APPROVED_SYSTEM_ID>
    +2 VERS <VERSION_NUMBER>
    +2 NAME <NAME_OF_PRODUCT>
    +2 CORP <NAME_OF_BUSINESS>
        +3 <<ADDRESS_STRUCTURE>>
    +2 DATA <NAME_OF_SOURCE_DATA>
        +3 DATE <PUBLICATION_DATE>
        +3 COPR <COPYRIGHT_SOURCE_DATA>
        +4 [CONT|CONC]<COPYRIGHT_SOURCE_DATA>
+1 DEST <RECEIVING_SYSTEM_NAME>
+1 DATE <TRANSMISSION_DATE>
    +2 TIME <TIME_VALUE>
+1 SUBM @<XREF:SUBM>@
+1 SUBN @<XREF:SUBN>@
+1 FILE <FILE_NAME>
+1 COPR <COPYRIGHT_GEDCOM_FILE> +1 GEDC
    +2 VERS <VERSION_NUMBER>
    +2 FORM <GEDCOM_FORM> +1 CHAR <CHARACTER_SET>
    +2 VERS <VERSION_NUMBER> +1 LANG <LANGUAGE_OF_TEXT> +1 PLAC
    +2 FORM <PLACE_HIERARCHY>
+1 NOTE <GEDCOM_CONTENT_DESCRIPTION>
    +2 [CONC|CONT] <GEDCOM_CONTENT_DESCRIPTION>
*/

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Header {
    pub encoding: Option<String>,
    pub copyright: Option<String>,
    pub date: Option<DateTime>,
    pub destination: Option<String>,
    pub gedcom_version: Option<Gedc>,
    pub language: Option<String>,
    pub filename: Option<String>,
    pub note: Option<String>,
    pub source: Option<Source>,
    pub submitter: Option<Submitter>,
    pub submission: Option<String>,
}

impl Header {
    pub fn parse(mut record: String) -> Header {
        let mut header = Header {
            encoding: None,
            copyright: None,
            // corporation: None,
            date: None,
            destination: None,
            gedcom_version: None,
            language: None,
            filename: None,
            note: None,
            source: None,
            submitter: None,
            submission: None,
        };

        // do parser stuff here
        while !record.is_empty() {
            let buffer: &str;
            let line: Line;

            (_, line) = parse::peek_line(&record).unwrap();

            // Inspect the top-level tags only.
            if line.level == 0 && line.tag == "HEAD" {
                // Consume the line
                // println!("Consuming HEAD");
                (buffer, _) = parse::line(&record).unwrap();
            } else if line.level == 1 {
                // println!("Found an inner tag: {}", line.tag);
                match line.tag {
                    "CHAR" => {
                        header.encoding = Some(line.value.unwrap_or("").to_string());
                        (buffer, _) = parse::line(&record).unwrap();
                    }
                    "COPR" => {
                        (buffer, header.copyright) = parse::get_tag_value(&record).unwrap();

                        // header.copyright = Some(line.value.unwrap_or("").to_string());
                        // (buffer, _) = parse::line(&record).unwrap();
                        // (buffer, header.copyright) = Copyright::parse(&record);
                    }
                    // "CORP" => {
                    //     println!("parsing CORP");
                    //     (buffer, header.corporation) = corporation::Corporation::parse(&record);
                    // }
                    "DATE" => {
                        // We're doing lazy parsing of the date, because parsing
                        // date strings is hard. For now.
                        (buffer, header.date) = DateTime::parse(&record);
                    }
                    "DEST" => {
                        header.destination = Some(line.value.unwrap_or("").to_string());
                        (buffer, _) = parse::line(&record).unwrap();
                    }
                    "FILE" => {
                        header.filename = Some(line.value.unwrap_or("").to_string());
                        (buffer, _) = parse::line(&record).unwrap();
                    }
                    "GEDC" => {
                        (buffer, header.gedcom_version) = Gedc::parse(&record);
                    }
                    "LANG" => {
                        header.language = Some(line.value.unwrap_or("").to_string());
                        (buffer, _) = parse::line(&record).unwrap();
                    }
                    "NOTE" => {
                        // This is just parsing the value of a line, and any
                        // CONC/CONT that follows. Rewrite
                        (buffer, header.note) = parse::get_tag_value(&record).unwrap();
                        // let note: Option<Note>;
                        // (buffer, note) = Note::parse(&record);
                        // header.note = note;
                    }
                    "SOUR" => {
                        (buffer, header.source) = Source::parse(&record);
                    }
                    "SUBM" => {
                        (buffer, header.submitter) = Submitter::parse(&record);
                    }
                    _ => {
                        // println!("Unhandled header tag: {}", line.tag);
                        (buffer, _) = parse::line(&record).unwrap();
                    }
                };
            } else {
                (buffer, _) = parse::line(&record).unwrap();
            }

            record = buffer.to_string();
        }
        header
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{corporation::Corporation, Address, DateTime, Form};

    use super::Header;

    #[test]
    fn parse_header() {
        let data = vec![
            "0 HEAD",
            "1 CHAR UTF-8",
            "1 SOUR Ancestry.com Family Trees",
            "2 DATA Name of source data",
            "3 DATE 1 JAN 1998",
            "3 COPR Copyright of source data",
            "2 VERS (2010.3)",
            "2 NAME Ancestry.com Family Trees",
            "2 CORP Ancestry.com",
            "3 ADDR",
            "4 ADR1 Example Software",
            "4 ADR2 123 Main Street",
            "4 ADR3 Ste 1",
            "4 CITY Anytown",
            "4 STAE IL",
            "4 POST 55555",
            "4 CTRY USA",
            "3 PHON +1-800-555-1111",
            "3 PHON +1-800-555-1212",
            "3 PHON +1-800-555-1313",
            "3 EMAIL a@example.com",
            "3 EMAIL b@example.com",
            "3 EMAIL c@example.com",
            "3 FAX +1-800-555-1414",
            "3 FAX +1-800-555-1515",
            "3 FAX +1-800-555-1616",
            "3 WWW https://www.example.com",
            "3 WWW https://www.example.org",
            "3 WWW https://www.example.net",
            "1 SUBM @U1@",
            "1 GEDC",
            "2 VERS 5.5",
            "2 FORM LINEAGE-LINKED",
            "3 VERS 5.5",
            "1 COPR A copyright statement",
            "1 LANG English",
            "1 DATE 1 JAN 2023",
            "2 TIME 12:13:14.15",
            // The submitter record
            "0 @U1@ SUBM",
            "1 NAME Adam Israel",
            "1 ADDR",
            "2 ADR1 Example Software",
            "2 ADR2 123 Main Street",
            "2 ADR3 Ste 1",
            "2 CITY Anytown",
            "2 STAE IL ",
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
            "1 RIN 1",
            "1 CHAN",
            "2 DATE 7 SEP 2000",
            "3 TIME 8:35:36",
        ];

        let header = Header::parse(data.join("\n"));

        // encoding
        assert!(header.encoding.is_some());
        assert!(header.encoding == Some("UTF-8".to_string()));

        // copyright
        assert!(header.copyright.is_some());
        assert!(header.copyright == Some("A copyright statement".to_string()));

        // source
        assert!(header.source.is_some());
        assert!(header.source.as_ref().unwrap().source == "Ancestry.com Family Trees".to_string());
        assert!(header.source.as_ref().unwrap().version == Some("(2010.3)".to_string()));

        assert!(
            header
                .source
                .as_ref()
                .unwrap()
                .data
                .as_ref()
                .unwrap()
                .copyright
                == Some("Copyright of source data".to_string())
        );
        assert!(
            header.source.as_ref().unwrap().data.as_ref().unwrap().date
                == Some(DateTime {
                    date: Some("1 JAN 1998".to_string()),
                    time: None,
                })
        );
        assert!(
            header.source.as_ref().unwrap().data.as_ref().unwrap().name
                == Some("Name of source data".to_string())
        );

        assert!(
            header.source.as_ref().unwrap().name == Some("Ancestry.com Family Trees".to_string())
        );
        assert!(
            header.source.as_ref().unwrap().corporation
                == Some(Corporation {
                    name: Some("Ancestry.com".to_string()),
                    address: Some(Address {
                        addr1: Some("Example Software".to_string()),
                        addr2: Some("123 Main Street".to_string()),
                        addr3: Some("Ste 1".to_string()),
                        city: Some("Anytown".to_string()),
                        state: Some("IL".to_string()),
                        postal_code: Some("55555".to_string()),
                        country: Some("USA".to_string()),
                        phone: vec![
                            "+1-800-555-1111".to_string(),
                            "+1-800-555-1212".to_string(),
                            "+1-800-555-1313".to_string(),
                        ],
                        email: vec![
                            "a@example.com".to_string(),
                            "b@example.com".to_string(),
                            "c@example.com".to_string(),
                        ],
                        fax: vec![
                            "+1-800-555-1414".to_string(),
                            "+1-800-555-1515".to_string(),
                            "+1-800-555-1616".to_string(),
                        ],
                        www: vec![
                            "https://www.example.com".to_string(),
                            "https://www.example.org".to_string(),
                            "https://www.example.net".to_string(),
                        ],
                    })
                })
        );

        // Version
        assert!(
            header.gedcom_version.as_ref().unwrap().form
                == Some(Form {
                    name: Some("LINEAGE-LINKED".to_string()),
                    version: Some("5.5".to_string()),
                })
        );
        assert!(header.gedcom_version.as_ref().unwrap().version == Some("5.5".to_string()));

        // language
        assert!(header.language.is_some());
        assert!(header.language == Some("English".to_string()));

        // datetime
        assert!(header.date.is_some());
        assert!(
            header.date
                == Some(DateTime {
                    date: Some("1 JAN 2023".to_string()),
                    time: Some("12:13:14.15".to_string())
                })
        );

        // submitter
        assert!(header.submitter.is_some());
    }
}
