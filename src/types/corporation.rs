use crate::parse;
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

#[derive(Debug, Default)]
pub struct Corporation {
    pub name: Option<String>,
    pub address: Option<Address>,
}

pub fn parse_corporation(mut buffer: &str) -> (&str, Option<Corporation>) {
    let mut corp: Corporation = Corporation {
        name: None,
        address: None,
    };

    let (_, lvl) = parse::peek_level(buffer).unwrap();
    let (_, tag) = parse::peek_tag(buffer).unwrap();

    // Verify we have a CORP record
    if lvl == 2 && tag == "CORP" {
        let line: (u8, Option<&str>, Option<&str>, Option<&str>);

        (buffer, line) = parse::line(buffer).unwrap();
        corp.name = Some(line.3.unwrap_or("").to_string());

        // Check if the next line contains an address struct
        let (_, lvl) = parse::peek_level(buffer).unwrap();
        if lvl == 3 {
            //corp.address = addrss::parse_address(buffer).unwrap();
        }
    }

    (buffer, Some(corp))
}
