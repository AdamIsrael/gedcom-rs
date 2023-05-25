use crate::parse;
use crate::types::{
    Address,
    // Line,
    Source,
};

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
            // let (mut tmp, level) = parse::level(&record).unwrap();
            // let (mut tmp, _) = parse::delim(&tmp).unwrap();
            // let (mut tmp, xref) = parse::xref(&tmp).unwrap();
            // let (mut tmp, tag) = parse::tag(&tmp.trim_start()).unwrap();
            // let (mut tmp, _) = parse::delim(&tmp).unwrap();
            // let (mut tmp, value) = parse::value(&tmp).unwrap();
            // let (mut tmp, _) = parse::eol(&tmp).unwrap();

            // (u8, &str, Option<&str>, &str, &str, &str, &str)
            let level: u8;
            let xref: Option<&str>;
            let tag: Option<&str>;
            let value: Option<&str>;
            let mut buffer: &str;

            // (buffer, (level, _, xref, tag, _, value, _)) = parse::line(&record).unwrap();
            (buffer, (level, xref, tag, value)) = parse::line(&record).unwrap();

            // let (mut tmp, (level, _, xref, tag, _, value, _)) = parse::line(&record).unwrap();
            let _xref = xref.unwrap_or("");

            // println!("[header] Level: {level}, xref: '{_xref}', tag: '{tag:?}', value: '{value:?}'");

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
            address: None,
            corporation: None,
            email: None,
            fax: None,
            name: None,
            phone: None,
            source: "".to_string(),
            version: None,
            www: None,
        };

        let (_, mut lvl) = parse::peek_level(buffer).unwrap();
        // println!("Level: {lvl}");
        while lvl >= 2 {
            let (mut str, tpl) = parse::line(buffer).unwrap();
            // let (buffer, (level, xref, tag, value)) = parse::line(&record).unwrap();

            // println!("Value: level: {}, tag {} = '{}'", tpl.1, tpl.3, tpl.5);
            match tpl.2.unwrap() {
                "ADDR" => {
                    println!("[debug] parsing address: {buffer}");
                    (str, source.address) = Self::parse_address(buffer);
                }
                "NAME" => {
                    source.name = Some(tpl.3.unwrap_or("").to_string());
                }
                "VERS" => {
                    source.version = Some(tpl.3.unwrap_or("").to_string());
                }
                // An ancestry-speecific tag
                "_TREE" => {}
                "CORP" => {
                    // println!("Got CORP: ({}) {:?}", tpl.1, tpl.5);
                    // println!("{:?}", tpl);
                    source.corporation = Some(tpl.3.unwrap_or("").to_string());

                    // TODO: Make this its own function
                }
                _ => {}
            }
            // Update the buffer with the remainder of data
            // TODO: Clean this up. It's hella fugly.
            buffer = str;

            // Peek at the next level
            (_, lvl) = parse::peek_level(str).unwrap();
        }

        (buffer, Some(source))
    }

    /// Parse the Address entity
    ///
    /// This could be formatted one of two (valid) ways:
    ///
    /// ```
    /// /*
    /// 3 ADDR 1300 West Traverse Parkway   
    /// 4 CONT Lehi, UT  84043   
    /// 4 CONT USA   
    /// */
    /// ```
    ///
    /// or:
    ///
    /// ```
    /// /*
    /// 3 ADDR
    /// 4 ADR1 RSAC Software
    /// 4 ADR2 7108 South Pine Cone Street
    /// 4 ADR3 Ste 1
    /// 4 CITY Salt Lake City
    /// 4 STAE UT
    /// 4 POST 84121
    /// 4 CTRY USA
    /// */
    /// ```
    ///
    fn parse_address(mut buffer: &str) -> (&str, Option<Address>) {
        let mut address = Address {
            addr1: None,
            addr2: None,
            addr3: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            phone: vec![],
            email: vec![],
            fax: vec![],
            www: vec![],
        };

        // Eat the ADDR record
        // (buffer, _) = parse::line(&buffer).unwrap();
        println!("[DEBUG] Parsing address: '{buffer}'");

        let (_, mut lvl) = parse::peek_level(buffer).unwrap();

        // Only iterate through the ADDR records
        while lvl >= 3 {
            // println!("Buffer: '{buffer}'");
            let (mut str, tpl) = parse::line(buffer).unwrap();
            // println!("Tuple: {tpl:?}");

            // println!("Value: level: {}, tag '{}' = '{}'", tpl.0, tpl.1.unwrap_or(""), tpl.3.unwrap_or(""));
            match tpl.2.unwrap() {
                "ADDR" => {
                    // TODO: Should we attempt to parse this? Or stuff it all
                    // into addr1? It's not like it's a searchable field.
                    let mut addr: String = String::from("");

                    addr += tpl.3.unwrap_or("");

                    // address.addr1 = Some(tpl.3.unwrap_or("").to_string());

                    // handle CONT/CONC; but what's the best way to append that data?
                    // CONT implies that we're continuing the data, i.e., adding a
                    // newline to preserve the formatting
                    // CONC implies that we're concatenating the line

                    let mut tag;
                    // TODO: Need to check the result and bail from this block if
                    // it returns an error
                    (_, tag) = parse::peek_tag(str).unwrap_or(("", ""));
                    println!("\nNEXT: {tag}");

                    while tag == "CONT" {
                        let (asdf, cont) = parse::cont(str).unwrap();
                        println!("[parse_address] CONT: {cont}");
                        addr += "\n";
                        addr += cont;

                        str = asdf;

                        (_, tag) = parse::peek_tag(str).unwrap();
                    }

                    address.addr1 = Some(addr);
                }
                "ADR1" => {
                    address.addr1 = Some(tpl.3.unwrap_or("").to_string());
                }
                "ADR2" => {
                    address.addr2 = Some(tpl.3.unwrap_or("").to_string());
                }
                "ADR3" => {
                    address.addr3 = Some(tpl.3.unwrap_or("").to_string());
                }
                "CITY" => {
                    address.city = Some(tpl.3.unwrap_or("").to_string());
                }
                "STAE" => {
                    address.state = Some(tpl.3.unwrap_or("").to_string());
                }
                "POST" => {
                    address.postal_code = Some(tpl.3.unwrap_or("").to_string());
                }
                "CTRY" => {
                    address.country = Some(tpl.3.unwrap_or("").to_string());
                }
                "PHON" => {
                    address.phone.push(tpl.3.unwrap_or("").to_string());
                }
                "EMAIL" => {
                    address.email.push(tpl.3.unwrap_or("").to_string());
                }
                "FAX" => {
                    address.fax.push(tpl.3.unwrap_or("").to_string());
                }
                "WWW" => {
                    address.www.push(tpl.3.unwrap_or("").to_string());
                }
                _ => {}
            }
            // Update the buffer with the remainder of data
            // TODO: Clean this up. It's hella fugly.
            buffer = str;

            // Peek at the next level
            // println!("{address:?}");
            // Grab the next level, if there is one, or short-circuit the loop
            (_, lvl) = parse::peek_level(str).unwrap_or(("", 0));
        }
        (buffer, Some(address))
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
    use super::*;

    #[test]
    fn parse_addr_tag() {
        let data = "3 ADDR\n";
        let (str, (level, xref, tag, value)) = parse::line(&data).unwrap();

        assert!(str.len() == 0);
        assert!(level == 3);
        assert!(xref == Some(""));
        assert!(tag == Some("ADDR"));
        assert!(value == Some(""));
    }

    #[test]
    fn parse_adr1_tag() {
        let data = "4 ADR1 RSAC Software\n";
        let (str, (level, xref, tag, value)) = parse::line(&data).unwrap();

        assert!(str.len() == 0);
        assert!(level == 4);
        assert!(xref == Some(""));
        assert!(tag == Some("ADR1"));
        assert!(value == Some("RSAC Software"));
    }

    #[test]
    fn parse_full_address1() {
        let data = vec![
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
        ];

        let (data, address) = super::Header::parse_address(data.join("\n").as_str());
        let addr = address.unwrap();

        println!("addr1: {:?}", addr.addr1);
        assert!(addr.addr1 == Some("RSAC Software".to_string()));
        assert!(addr.addr2 == Some("7108 South Pine Cone Street".to_string()));
        assert!(addr.addr3 == Some("Ste 1".to_string()));
        assert!(addr.city == Some("Salt Lake City".to_string()));
        assert!(addr.state == Some("UT".to_string()));
        assert!(addr.postal_code == Some("84121".to_string()));
        assert!(addr.country == Some("USA".to_string()));
        assert!(addr.phone.contains(&"+1-801-942-7768".to_string()));
        assert!(addr.phone.contains(&"+1-801-555-1212".to_string()));
        assert!(addr.phone.contains(&"+1-801-942-1148".to_string()));
        assert!(addr.email.contains(&"a@@example.com".to_string()));
        assert!(addr.email.contains(&"b@@example.com".to_string()));
        assert!(addr.email.contains(&"c@@example.com".to_string()));
        assert!(addr.fax.contains(&"+1-801-942-1148".to_string()));
        assert!(addr.fax.contains(&"+1-801-942-1148".to_string()));
        assert!(addr.fax.contains(&"+1-801-942-1148".to_string()));
        assert!(addr.www.contains(&"https://www.example.com".to_string()));
        assert!(addr.www.contains(&"https://www.example.org".to_string()));
        assert!(addr.www.contains(&"https://www.example.net".to_string()));
    }

    #[test]
    /// Test the address block as used by Ancestry
    fn parse_full_address2() {
        let data = vec![
            "3 ADDR 1300 West Traverse Parkway",
            "4 CONT Lehi, UT  84043",
            "4 CONT USA",
        ];

        let (data, address) = super::Header::parse_address(data.join("\n").as_str());
        let addr = address.unwrap();

        println!("Actual addr: '{:?}'", addr);
        assert!(addr.addr1 == Some("1300 West Traverse Parkway\nLehi, UT  84043\nUSA".to_string()));
        // assert!(addr.addr2 == Some("7108 South Pine Cone Street".to_string()));
        // assert!(addr.addr3 == Some("Ste 1".to_string()));
        // assert!(addr.city == Some("Salt Lake City".to_string()));
        // assert!(addr.state == Some("UT".to_string()));
        // assert!(addr.postal_code == Some("84121".to_string()));
        // assert!(addr.country == Some("USA".to_string()));
    }
}
