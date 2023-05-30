use crate::parse;
use crate::types::Source;

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

// #[derive(Debug, Default)]
// #[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
// /// Header containing GEDCOM metadata
// pub struct Header<'a> {
//     pub encoding: Option<& 'a str>,
//     pub copyright: Option<& 'a str>,
//     pub corporation: Option<& 'a str>,
//     pub date: Option<& 'a str>,
//     pub destination: Option<& 'a str>,
//     pub gedcom_version: Option<& 'a str>,
//     pub language: Option<& 'a str>,
//     pub filename: Option<& 'a str>,
//     pub note: Option<& 'a str>,
//     pub source: Option<Source>,
//     pub submitter: Option<& 'a str>,
//     pub submission: Option<& 'a str>,
// }

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Header {
    pub encoding: Option<String>,
    pub copyright: Option<String>,
    pub corporation: Option<String>,
    pub date: Option<String>,
    pub destination: Option<String>,
    pub gedcom_version: Option<String>,
    pub language: Option<String>,
    pub filename: Option<String>,
    pub note: Option<String>,
    pub source: Option<Source>,
    // pub sources: Vec<Source>,
    pub submitter: Option<String>,
    pub submission: Option<String>,
}

impl Header {
    pub fn parse(mut record: String) -> Header {
        let mut header = Header {
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
        };

        // do parser stuff here
        while !record.is_empty() {
            let level: u8;
            let xref: Option<&str>;
            let tag: Option<&str>;
            let value: Option<&str>;
            let mut buffer: &str;

            (buffer, (level, xref, tag, value)) = parse::line(&record).unwrap();

            let _xref = xref.unwrap_or("");

            match tag.unwrap() {
                "CHAR" => {
                    header.encoding = Some(value.unwrap_or("").to_string());
                }
                "COPR" => {
                    // header.copyright = Some(Value);
                    println!("Found copyright!");
                }
                "DATE" => {
                    header.date = Some(value.unwrap_or("").to_string());

                    // Now we need to get the next line
                    let (_, lvl) = parse::peek_level(buffer).unwrap();
                    if lvl == (level + 1) {
                        // TODO: Store date and time separately? Parse the date properly
                        // to make it easier to search on? Lots of potentially invalid dates, though.
                        // about, between, circa, etc.
                        // parse the next line and get the value
                        let (_str, tpl) = parse::line(buffer).unwrap();
                        // This could be cleaner than accessing tpl.5. Probably:
                        // (_, _, _, _, value, _)

                        header.date =
                            Some(value.unwrap_or("").to_string() + " " + tpl.3.unwrap_or(""));
                    }
                }
                "SOUR" => {
                    (buffer, header.source) = Self::parse_source(buffer);
                }
                "SUBM" => {
                    println!("SUBM: xref: '{_xref}', value: '{value:?}', buffer: {buffer}");
                }
                _ => {}
            };

            record = buffer.to_string();
        }

        header
    }

    fn parse_source(mut buffer: &str) -> (&str, Option<Source>) {
        let mut source = Source {
            corporation: None,
            name: None,
            source: "".to_string(),
            version: None,
        };

        let (_, lvl) = parse::peek_level(buffer).unwrap();
        let (_, tag) = parse::peek_tag(buffer).unwrap();

        // Verify we have a SOUR record
        if lvl == 1 && tag == "SOUR" {
            let (str, tpl) = parse::line(buffer).unwrap();
            buffer = str;
            source.source = tpl.3.unwrap_or("").to_string();

            let (_, mut lvl) = parse::peek_level(buffer).unwrap();

            // println!("Level: {lvl}");
            while lvl >= 2 {
                let (mut str, tpl) = parse::line(buffer).unwrap();
                // let (buffer, (level, xref, tag, value)) = parse::line(&record).unwrap();

                // println!("Value: level: {:?}, tag {:?} = '{:?}'", tpl.1, tpl.3, tpl.5);
                // println!("tpl: {:?}", tpl);
                match tpl.2.unwrap() {
                    // An ancestry-speecific tag
                    "_TREE" => {
                        // The value of tree contains the tree name, which is useful,
                        // but not a part of the GEDCOM spec.
                        // The next level (3) may contain RIN, some sort of internal id
                        // but is probably not useful for anything
                    }
                    // "ADDR" => {
                    //     println!("[debug] parsing address: {buffer}");
                    //     (str, source.address) = Self::parse_address(buffer);
                    // }
                    "CORP" => {
                        (str, source.corporation) =
                            crate::types::corporation::parse_corporation(buffer);
                        // source.corporation = Some(tpl.3.unwrap_or("").to_string());

                        // What remains in the buffer may include an address
                    }
                    "NAME" => {
                        source.name = Some(tpl.3.unwrap_or("").to_string());
                    }
                    "VERS" => {
                        source.version = Some(tpl.3.unwrap_or("").to_string());
                    }
                    _ => {}
                }

                // Update the buffer with the remainder of data
                // TODO: Clean this up. It's hella fugly.
                buffer = str;

                // Peek at the next level
                if buffer.is_empty() {
                    break;
                }
                (_, lvl) = parse::peek_level(str).unwrap();
            }
        }

        (buffer, Some(source))
    }

    // fn parse_value(value: Option<String>) -> String {
    //     value.unwrap()
    // }

    // pub fn add_destination(&mut self, destination: String) {
    //     self.destinations.push(destination);
    // }

    // pub fn add_source(&mut self, source: Source) {
    //     self.sources.push(source);
    // }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn parse_source() {
        let data = vec![
            "1 SOUR Ancestry.com Family Trees",
            "2 NAME Ancestry.com Member Trees",
            "2 VERS 2021.07",
            "2 _TREE Ambrose Bierce Family Tree",
            "3 RIN 116823582",
            "3 _ENV prd",
            "2 CORP Ancestry.com",
            "3 PHON 801-705-7000",
            "3 WWW www.ancestry.com",
            "3 ADDR 1300 West Traverse Parkway",
            "4 CONT Lehi, UT  84043",
            "4 CONT USA",
        ];

        let (_data, source) = super::Header::parse_source(&data.join("\n"));
        let sour = source.unwrap();
        println!("source: {:?}", sour);

        assert!(sour.name == Some("Ancestry.com Member Trees".to_string()));
    }
}
