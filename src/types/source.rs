// use crate::parse;
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

#[derive(Clone, Debug, Default)]
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
        let mut source = Source::default();

        let Ok(line) = Line::peek(&mut buffer) else {
            return (buffer, Some(source));
        };

        // Verify we have a SOUR record
        if line.level == 1 && line.tag == "SOUR" {
            // Consume the first line
            let Ok(line) = Line::parse(&mut buffer) else {
                return (buffer, Some(source));
            };

            source.source = line.value.to_string();

            let Ok(mut next) = Line::peek(&mut buffer) else {
                return (buffer, Some(source));
            };

            while next.level >= line.level {
                // We don't want to consume the line yet because we may need
                // the original for a parser.
                let Ok(inner_line) = Line::peek(&mut buffer) else {
                    break;
                };

                // println!("Evaluating tag: {:?}", inner_line.tag);
                match inner_line.tag {
                    // An ancestry-specific tag
                    "_TREE" => {
                        // The next level (3) may contain RIN, some sort of internal id
                        // but is probably not useful for anything
                        // Consume the line
                        let _ = Line::parse(&mut buffer);
                    }
                    "CORP" => {
                        (buffer, source.corporation) = Corporation::parse(buffer);
                    }
                    "NAME" => {
                        source.name = Some(inner_line.value.to_string());
                        let _ = Line::parse(&mut buffer);
                    }
                    "VERS" => {
                        source.version = Some(inner_line.value.to_string());
                        let _ = Line::parse(&mut buffer);
                    }
                    "DATA" => {
                        (buffer, source.data) = SourceData::parse(buffer);
                    }
                    _ => {
                        // consume the line so we can parse the next
                        let _ = Line::parse(&mut buffer);
                    }
                }

                // Peek at the next level
                if !buffer.is_empty() {
                    let Ok(n) = Line::peek(&mut buffer) else {
                        break;
                    };
                    next = n;
                    if next.level <= 1 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        (buffer, Some(source))
    }
}

#[allow(clippy::unwrap_used)]
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
