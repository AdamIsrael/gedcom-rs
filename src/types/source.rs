use crate::parse;
// use crate::types::corporation::Corporation;

use super::{corporation::Corporation, Line, SourceData};

// +1 SOUR <APPROVED_SYSTEM_ID>
//     +2 VERS <VERSION_NUMBER>
//     +2 NAME <NAME_OF_PRODUCT>
//     +2 CORP <NAME_OF_BUSINESS>
//         +3 <<ADDRESS_STRUCTURE>>
//     +2 DATA <NAME_OF_SOURCE_DATA>
//         +3 DATE <PUBLICATION_DATE>
//         +3 COPR <COPYRIGHT_SOURCE_DATA>
//         +4 [CONT|CONC]<COPYRIGHT_SOURCE_DATA>

// 1 SOUR Ancestry.com Family Trees
// 2 NAME Ancestry.com Member Trees
// 2 VERS 2021.07
// 2 _TREE Ambrose Bierce Family Tree
// 3 RIN 116823582
// 3 _ENV prd
// 2 CORP Ancestry.com
// 3 PHON 801-705-7000
// 3 WWW www.ancestry.com
// 3 ADDR 1300 West Traverse Parkway
// 4 CONT Lehi, UT  84043
// 4 CONT USA

#[derive(Debug, Default)]
pub struct Source {
    /// A corporation tag contains the name of the corporation and its address.
    pub corporation: Option<Corporation>,
    // pub data: Option<Data>,
    pub name: Option<String>,
    pub source: String,
    pub data: Option<SourceData>,
    // pub copyright: Option<Copyright>,
    pub version: Option<String>,
}

impl Source {
    /// Parse a SOUR record
    pub fn parse(mut buffer: &str) -> (&str, Option<Source>) {
        let mut source = Source {
            corporation: None,
            data: None,
            name: None,
            source: "".to_string(),
            version: None,
        };
        let mut line: Line;

        (_, line) = parse::peek_line(buffer).unwrap();

        // Verify we have a SOUR record
        if line.level == 1 && line.tag == "SOUR" {
            // Consume the first line
            (buffer, line) = parse::line(buffer).unwrap();

            source.source = line.value.unwrap_or("").to_string();

            let (_, mut lvl) = parse::peek_level(buffer).unwrap();

            while lvl >= line.level {
                let inner_line: Line;

                // We don't want to consume the line yet because we may need
                // the original for a parser.
                (_, inner_line) = parse::peek_line(buffer).unwrap();

                // println!("Evaluating tag: {:?}", inner_line.tag);
                match inner_line.tag {
                    // An ancestry-specific tag
                    "_TREE" => {
                        // The value of tree contains the tree name, which is useful,
                        // but not a part of the GEDCOM spec.
                        // The next level (3) may contain RIN, some sort of internal id
                        // but is probably not useful for anything
                        println!("Skipping _TREE");
                        // Consume the line
                        (buffer, _) = parse::line(buffer).unwrap();
                    }
                    "CORP" => {
                        (buffer, source.corporation) = Corporation::parse(buffer);
                    }
                    "NAME" => {
                        source.name = Some(inner_line.value.unwrap_or("").to_string());
                        (buffer, _) = parse::line(buffer).unwrap();
                    }
                    "VERS" => {
                        source.version = Some(inner_line.value.unwrap_or("").to_string());
                        (buffer, _) = parse::line(buffer).unwrap();
                    }
                    "DATA" => {
                        (buffer, source.data) = SourceData::parse(buffer);
                    }
                    _ => {
                        println!("Unknown line: {:?}", inner_line);

                        // consume the line so we can parse the next
                        (buffer, _) = parse::line(buffer).unwrap();
                    }
                }

                // Peek at the next level
                if !buffer.is_empty() {
                    (_, lvl) = parse::peek_level(buffer).unwrap();
                    if lvl <= 1 {
                        break;
                    }
                } else {
                    break;
                }
                // if buffer.is_empty() || lvl <= 1 {
                //     println!("Aborting SOUR.");
                //     break;
                // }
            }
        }

        (buffer, Some(source))
    }
}

#[cfg(test)]
mod tests {
    use crate::types::DateTime;

    use super::{Source, SourceData};

    #[test]
    fn parse() {
        let data = vec![
            "1 SOUR GEDitCOM",
            "2 VERS 2.9.4",
            "2 NAME GEDitCOM",
            "2 CORP RSAC Software",
            "3 ADDR",
            "4 ADR1 RSAC Software",
            "4 ADR2 7108 South Pine Cone Street",
            "4 ADR3 Ste 1",
            "4 CITY Salt Lake City",
            "4 STAE UT",
            "4 POST 84121",
            "4 CTRY USA",
            "3 PHON +1-801-942-7768",
            "3 PHON +1-801-555-1212",
            "3 PHON +1-801-942-1148",
            "3 EMAIL a@@example.com",
            "3 EMAIL b@@example.com",
            "3 EMAIL c@@example.com",
            "3 FAX +1-801-942-7768",
            "3 FAX +1-801-555-1212",
            "3 FAX +1-801-942-1148",
            "3 WWW https://www.example.com",
            "3 WWW https://www.example.org",
            "3 WWW https://www.example.net",
            "2 DATA Name of source data",
            "3 DATE 1 JAN 1998",
            "3 COPR Copyright of source data",
        ];

        let (_data, source) = Source::parse(&data.join("\n"));
        let sour = source.unwrap();

        assert_eq!(sour.source, "GEDitCOM".to_string());
        assert_eq!(sour.name, Some("GEDitCOM".to_string()));
        assert_eq!(sour.version, Some("2.9.4".to_string()));
        assert_eq!(
            sour.data,
            Some(SourceData {
                name: Some("Name of source data".to_string()),
                date: Some(DateTime {
                    date: Some("1 JAN 1998".to_string()),
                    time: None
                }),
                copyright: Some("Copyright of source data".to_string()),
            })
        );
        let corp = sour.corporation.unwrap();

        assert_eq!(corp.name, Some("RSAC Software".to_string()));

        let corp_address: crate::types::Address = corp.address.unwrap();
        assert_eq!(corp_address.addr1, Some("RSAC Software".to_string()));
        assert_eq!(
            corp_address.addr2,
            Some("7108 South Pine Cone Street".to_string())
        );
        assert_eq!(corp_address.addr3, Some("Ste 1".to_string()));
        assert_eq!(corp_address.city, Some("Salt Lake City".to_string()));
        assert_eq!(corp_address.state, Some("UT".to_string()));
        assert_eq!(corp_address.postal_code, Some("84121".to_string()));
        assert_eq!(corp_address.country, Some("USA".to_string()));

        assert!(corp_address.phone.contains(&"+1-801-942-7768".to_string()));
        assert!(corp_address.phone.contains(&"+1-801-555-1212".to_string()));
        assert!(corp_address.phone.contains(&"+1-801-942-1148".to_string()));
        assert!(corp_address.email.contains(&"a@@example.com".to_string()));
        assert!(corp_address.email.contains(&"b@@example.com".to_string()));
        assert!(corp_address.email.contains(&"c@@example.com".to_string()));
        assert!(corp_address.fax.contains(&"+1-801-942-1148".to_string()));
        assert!(corp_address.fax.contains(&"+1-801-942-1148".to_string()));
        assert!(corp_address.fax.contains(&"+1-801-942-1148".to_string()));
        assert!(corp_address
            .www
            .contains(&"https://www.example.com".to_string()));
        assert!(corp_address
            .www
            .contains(&"https://www.example.org".to_string()));
        assert!(corp_address
            .www
            .contains(&"https://www.example.net".to_string()));
    }
}
