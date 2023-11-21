use super::Line;
use crate::parse;

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
    pub fn parse(mut buffer: &str) -> (&str, Option<Address>) {
        // println!("DEBUG: {:?}", buffer);
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
        let mut line: Line;

        let (_, mut lvl) = parse::peek_level(buffer).unwrap();

        let min_level = lvl;

        // Only iterate through the ADDR records
        while lvl >= min_level {
            (_, line) = parse::peek_line(buffer).unwrap();

            match line.tag {
                "ADDR" => {
                    let mut addr: String = String::from("");

                    addr += line.value.unwrap_or("");

                    // handle CONT/CONC; but what's the best way to append that data?
                    // CONT implies that we're continuing the data, i.e., adding a
                    // newline to preserve the formatting
                    // CONC implies that we're concatenating the line

                    let mut tag;

                    // create a temp buffer to see if we have a CONC/CONT
                    let (mut addr_buffer, _) = parse::line(buffer).unwrap();
                    (_, tag) = parse::peek_tag(addr_buffer).unwrap_or(("", ""));

                    while tag == "CONT" || tag == "CONC" {
                        if tag == "CONT" {
                            let (asdf, cont) = parse::cont(addr_buffer).unwrap();

                            addr += "\n";
                            addr += cont;

                            addr_buffer = asdf;
                        } else if tag == "CONC" {
                            let (asdf, cont) = parse::conc(addr_buffer).unwrap();
                            addr += " ";
                            addr += cont;
                            addr_buffer = asdf;
                        }

                        (_, tag) = parse::peek_tag(addr_buffer).unwrap();
                    }
                    address.addr1 = Some(addr);
                }
                "ADR1" => {
                    address.addr1 = Some(line.value.unwrap_or("").to_string());
                }
                "ADR2" => {
                    address.addr2 = Some(line.value.unwrap_or("").to_string());
                }
                "ADR3" => {
                    address.addr3 = Some(line.value.unwrap_or("").to_string());
                }
                "CONT" => {} // Ignore, it's a special case handled by ADDR
                "CONC" => {} // Ignore, it's a special case handled by ADDR
                "CITY" => {
                    address.city = Some(line.value.unwrap_or("").to_string());
                }
                "STAE" => {
                    address.state = Some(line.value.unwrap_or("").to_string());
                }
                "POST" => {
                    address.postal_code = Some(line.value.unwrap_or("").to_string());
                }
                "CTRY" => {
                    address.country = Some(line.value.unwrap_or("").to_string());
                }
                "PHON" => {
                    address.phone.push(line.value.unwrap_or("").to_string());
                }
                "EMAIL" => {
                    address.email.push(line.value.unwrap_or("").to_string());
                }
                "FAX" => {
                    address.fax.push(line.value.unwrap_or("").to_string());
                }
                "WWW" => {
                    address.www.push(line.value.unwrap_or("").to_string());
                }
                _ => {
                    // We've hit a non-address tag, so break out of the loop
                    break;
                }
            }

            (buffer, _) = parse::line(buffer).unwrap();

            // Grab the next level, if there is one, or short-circuit the loop
            (_, lvl) = parse::peek_level(buffer).unwrap_or(("", 0));
        }
        (buffer, Some(address))
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
        let data = "3 ADDR\n";
        let (str, line) = parse::line(&data).unwrap();

        assert!(str.len() == 0);
        assert!(line.level == 3);
        assert!(line.xref == Some(""));
        assert!(line.tag == "ADDR");
        assert!(line.value == Some(""));
    }

    #[test]
    fn parse_adr1_tag() {
        let data = "4 ADR1 RSAC Software\n";
        let (str, line) = parse::line(&data).unwrap();

        assert!(str.len() == 0);
        assert!(line.level == 4);
        assert!(line.xref == Some(""));
        assert!(line.tag == "ADR1");
        assert!(line.value == Some("RSAC Software"));
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

        let (_data, address) = Address::parse(data.join("\n").as_str());
        let addr = address.unwrap();

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
        ];

        let (_data, address) = Address::parse(data.join("\n").as_str());
        let addr = address.unwrap();

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
        ];

        let (_data, address) = Address::parse(data.join("\n").as_str());
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
        ];

        let (_data, address) = Address::parse(data.join("\n").as_str());
        let addr = address.unwrap();

        assert!(addr.addr1 == Some("1300 West Traverse Parkway\nLehi, UT  84043 USA".to_string()));
    }
}
