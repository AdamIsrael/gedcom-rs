use crate::parse;

#[derive(Debug, Default)]
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
pub fn parse_address(mut buffer: &str) -> (&str, Option<Address>) {
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
    let mut line: (u8, Option<&str>, Option<&str>, Option<&str>);

    let (_, mut lvl) = parse::peek_level(buffer).unwrap();

    // Only iterate through the ADDR records
    while lvl >= 3 {
        (buffer, line) = parse::line(buffer).unwrap();
        // let (mut str, tpl) = parse::line(buffer).unwrap();
        match line.2.unwrap() {
            "ADDR" => {
                // TODO: Should we attempt to parse this? Or stuff it all
                // into addr1? It's not like it's a searchable field.
                let mut addr: String = String::from("");

                addr += line.3.unwrap_or("");

                // handle CONT/CONC; but what's the best way to append that data?
                // CONT implies that we're continuing the data, i.e., adding a
                // newline to preserve the formatting
                // CONC implies that we're concatenating the line

                let mut tag;

                // TODO: Need to check the result and bail from this block if
                // it returns an error
                (_, tag) = parse::peek_tag(buffer).unwrap_or(("", ""));

                while tag == "CONT" {
                    let (asdf, cont) = parse::cont(buffer).unwrap();
                    addr += "\n";
                    addr += cont;

                    buffer = asdf;

                    (_, tag) = parse::peek_tag(buffer).unwrap();
                }

                address.addr1 = Some(addr);
            }
            "ADR1" => {
                address.addr1 = Some(line.3.unwrap_or("").to_string());
            }
            "ADR2" => {
                address.addr2 = Some(line.3.unwrap_or("").to_string());
            }
            "ADR3" => {
                address.addr3 = Some(line.3.unwrap_or("").to_string());
            }
            "CITY" => {
                address.city = Some(line.3.unwrap_or("").to_string());
            }
            "STAE" => {
                address.state = Some(line.3.unwrap_or("").to_string());
            }
            "POST" => {
                address.postal_code = Some(line.3.unwrap_or("").to_string());
            }
            "CTRY" => {
                address.country = Some(line.3.unwrap_or("").to_string());
            }
            "PHON" => {
                address.phone.push(line.3.unwrap_or("").to_string());
            }
            "EMAIL" => {
                address.email.push(line.3.unwrap_or("").to_string());
            }
            "FAX" => {
                address.fax.push(line.3.unwrap_or("").to_string());
            }
            "WWW" => {
                address.www.push(line.3.unwrap_or("").to_string());
            }
            _ => {}
        }
        // Update the buffer with the remainder of data
        // TODO: Clean this up. It's hella fugly.
        // buffer = str;

        // Grab the next level, if there is one, or short-circuit the loop
        (_, lvl) = parse::peek_level(buffer).unwrap_or(("", 0));
    }
    (buffer, Some(address))
}