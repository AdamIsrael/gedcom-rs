use super::Line;
use crate::parse;

use winnow::prelude::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Address {
    pub addr1: Option<String>,
    pub addr2: Option<String>,
    pub addr3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Vec<String>,
    pub email: Vec<String>,
    pub fax: Vec<String>,
    pub www: Vec<String>,
}

impl Address {
    pub fn parse(buffer: &mut &str) -> PResult<Address> {
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

        let mut line = Line::peek(buffer).unwrap();
        let min_level = line.level;

        // Only iterate through the ADDR records
        while line.level >= min_level {
            line = Line::peek(buffer).unwrap();

            let mut consume = true;
            match line.tag {
                "ADDR" => {
                    address.addr1 = parse::get_tag_value(buffer).unwrap();
                    // println!("Input after get_tag_value: \n'{}'", buffer);
                    consume = false;
                }
                "ADR1" => {
                    address.addr1 = Some(line.value.to_string());
                }
                "ADR2" => {
                    address.addr2 = Some(line.value.to_string());
                }
                "ADR3" => {
                    address.addr3 = Some(line.value.to_string());
                }
                "CONT" => {} // Ignore, it's a special case handled by ADDR
                "CONC" => {} // Ignore, it's a special case handled by ADDR
                "CITY" => {
                    address.city = Some(line.value.to_string());
                }
                "STAE" => {
                    address.state = Some(line.value.to_string());
                }
                "POST" => {
                    address.postal_code = Some(line.value.to_string());
                }
                "CTRY" => {
                    address.country = Some(line.value.to_string());
                }
                "PHON" => {
                    address.phone.push(line.value.to_string());
                }
                "EMAIL" => {
                    address.email.push(line.value.to_string());
                }
                "FAX" => {
                    address.fax.push(line.value.to_string());
                }
                "WWW" => {
                    address.www.push(line.value.to_string());
                }
                _ => {
                    // We've hit a non-address tag, so break out of the loop
                    break;
                }
            }
            // println!("Buffer before: {}", buffer.len());
            if consume {
                Line::parse(buffer).unwrap();
            }
            // println!("Buffer after: {}", buffer.len());
            // (buffer, _) = Line::parse(buffer).unwrap();

            // Grab the next line, if there is one, or short-circuit the loop
            line = Line::peek(buffer).unwrap();
            // (_, line) = Line::peek(buffer).unwrap();
        }
        Ok(address)
    }
}

/// Parse the Address entity
///
/// This could be formatted one of three (valid) ways:
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
/// 3 ADDR 1300 West Traverse Parkway
/// 4 CONT Lehi, UT  84043
/// 4 CONC USA
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
/// Why did I do it this way, vs implementing `parse` on the Address?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_addr_tag() {
        let mut data = "3 ADDR\n";
        let line = Line::parse(&mut data).unwrap();

        // let (str, line) = Line::parse(&data).unwrap();

        assert!(data.len() == 0);
        assert!(line.level == 3);
        assert!(line.xref == "");
        assert!(line.tag == "ADDR");
        assert!(line.value == "");
    }

    #[test]
    fn parse_adr1_tag() {
        let mut data = "4 ADR1 RSAC Software\n";
        let line = Line::parse(&mut data).unwrap();

        assert!(data.is_empty());
        assert!(line.level == 4);
        assert!(line.xref == "");
        assert!(line.tag == "ADR1");
        assert!(line.value == "RSAC Software");
    }

    #[test]
    fn parse_full_address() {
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

        let input = data.join("\n");
        let mut record = input.as_str();

        let addr = Address::parse(&mut record).unwrap();

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
    fn parse_full_address2() {
        let data = vec![
            "1 ADDR",
            "2 ADR1 RSAC Software",
            "2 ADR2 7108 South Pine Cone Street",
            "2 ADR3 Ste 1",
            "2 CITY Salt Lake City",
            "2 STAE UT",
            "2 POST 84121",
            "2 CTRY USA",
            "1 PHON +1-801-942-7768",
            "1 PHON +1-801-555-1212",
            "1 PHON +1-801-942-1148",
            "1 EMAIL a@@example.com",
            "1 EMAIL b@@example.com",
            "1 EMAIL c@@example.com",
            "1 FAX +1-801-942-7768",
            "1 FAX +1-801-555-1212",
            "1 FAX +1-801-942-1148",
            "1 WWW https://www.example.com",
            "1 WWW https://www.example.org",
            "1 WWW https://www.example.net",
        ]
        .join("\n");

        let mut record = data.as_str();
        let addr = Address::parse(&mut record).unwrap();

        // println!("addr1: {:?}", addr.addr1);
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
    fn parse_addr_cont() {
        let data = vec![
            "3 ADDR 1300 West Traverse Parkway",
            "4 CONT Lehi, UT  84043",
            "4 CONT USA",
            "3 PHON +1-801-942-7768",
            "3 PHON +1-801-555-1212",
            "3 PHON +1-801-942-1148",
        ]
        .join("\n");

        let mut record = data.as_str();
        let address = Address::parse(&mut record);
        let addr = address.unwrap();

        assert!(addr.addr1 == Some("1300 West Traverse Parkway\nLehi, UT  84043\nUSA".to_string()));

        assert!(addr.phone.contains(&"+1-801-942-7768".to_string()));
        assert!(addr.phone.contains(&"+1-801-555-1212".to_string()));
        assert!(addr.phone.contains(&"+1-801-942-1148".to_string()));
    }

    #[test]
    /// Test the address block as used by Ancestry
    fn parse_addr_conc() {
        let data = vec![
            "3 ADDR 1300 West Traverse Parkway",
            "4 CONT Lehi, UT  84043",
            "4 CONC USA",
        ]
        .join("\n");

        let mut record = data.as_str();
        let address = Address::parse(&mut record);
        let addr = address.unwrap();
        assert!(addr.addr1 == Some("1300 West Traverse Parkway\nLehi, UT  84043 USA".to_string()));
    }
}
