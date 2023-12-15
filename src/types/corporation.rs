use super::Line;
use crate::types::Address;

// +1 SOUR <APPROVED_SYSTEM_ID>
//     +2 VERS <VERSION_NUMBER>
//     +2 NAME <NAME_OF_PRODUCT>
//     +2 CORP <NAME_OF_BUSINESS>
//         +3 <<ADDRESS_STRUCTURE>>
//     +2 DATA <NAME_OF_SOURCE_DATA>
//         +3 DATE <PUBLICATION_DATE>
//         +3 COPR <COPYRIGHT_SOURCE_DATA>
//         +4 [CONT|CONC]<COPYRIGHT_SOURCE_DATA>

#[derive(Debug, Default, PartialEq)]
pub struct Corporation {
    pub name: Option<String>,
    pub address: Option<Address>,
}

impl Corporation {
    pub fn parse(mut buffer: &str) -> (&str, Option<Corporation>) {
        let mut corp: Corporation = Corporation {
            name: None,
            address: None,
        };

        let mut line: Line = Line::peek(&mut buffer).unwrap();

        // Verify we have a CORP record
        // line = Line::peek(&mut buffer).unwrap();
        if line.level == 2 && line.tag == "CORP" {
            line = Line::parse(&mut buffer).unwrap();
            // (buffer, line) = Line::parse(buffer).unwrap();
            corp.name = Some(line.value.to_string());

            // Check if the next line contains an address struct
            line = Line::peek(&mut buffer).unwrap();

            if line.level == 3 && line.tag == "ADDR" {
                (buffer, corp.address) = Address::parse(buffer);
            }
        }

        (buffer, Some(corp))
    }
}

#[cfg(test)]
mod tests {
    use crate::types::corporation::Corporation;

    #[test]
    fn parse_corp() {
        let data = vec![
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
        ];

        let (_data, _corp) = Corporation::parse(data.join("\n").as_str());
        let corp = _corp.unwrap();

        assert!(Some("RSAC Software".to_string()) == corp.name);

        let addr = corp.address.unwrap();

        assert!(Some("RSAC Software".to_string()) == addr.addr1);
        assert!(Some("7108 South Pine Cone Street".to_string()) == addr.addr2);
        assert!(Some("Ste 1".to_string()) == addr.addr3);
        assert!(Some("Salt Lake City".to_string()) == addr.city);
        assert!(Some("UT".to_string()) == addr.state);
        assert!(Some("84121".to_string()) == addr.postal_code);
        assert!(Some("USA".to_string()) == addr.country);
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
}
