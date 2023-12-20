use crate::{
    parse,
    types::{Address, EventTypeCitedFrom, Family, Line, Object, Place, SourceCitation},
};

use winnow::prelude::*;

// n [ BIRT | CHR ] [Y|<NULL>] {1:1}
// +1 <<INDIVIDUAL_EVENT_DETAIL>> {0:1}* p.34
// +1 FAMC @<XREF:FAM>@

// EVENT_DETAIL:=
// n TYPE <EVENT_OR_FACT_CLASSIFICATION> {0:1} p.49
// n DATE <DATE_VALUE> {0:1} p.47, 46
// n <<PLACE_STRUCTURE>> {0:1} p.38
// n <<ADDRESS_STRUCTURE>> {0:1} p.31
// n AGNC <RESPONSIBLE_AGENCY> {0:1} p.60
// n RELI <RELIGIOUS_AFFILIATION> {0:1} p.60
// n CAUS <CAUSE_OF_EVENT> {0:1} p.43
// n RESN <RESTRICTION_NOTICE> {0:1} p.60
// n <<NOTE_STRUCTURE>> {0:M} p.37
// n <<SOURCE_CITATION>> {0:M} p.39
// n <<MULTIMEDIA_LINK>> {0:M} p.37, 26
// FAMILY

#[derive(Debug, Default)]
pub struct Birth {
    pub age: Option<String>,
    pub date: Option<String>,
    pub r#type: Option<String>,
    pub place: Option<Place>,
    pub sources: Vec<SourceCitation>,
    pub address: Option<Address>,
    pub agency: Option<String>,
    pub religion: Option<String>,
    pub cause: Option<String>,
    pub note: Option<String>,
    pub event_type_cited_from: Option<EventTypeCitedFrom>,
    pub media: Vec<Object>,
    pub family: Option<Family>,
}

impl Birth {
    pub fn parse(record: &mut &str) -> PResult<Birth> {
        let mut birth = Birth {
            address: None,
            age: None,
            agency: None,
            cause: None,
            date: None,
            event_type_cited_from: None,
            family: None,
            media: vec![],
            note: None,
            place: None,
            religion: None,
            sources: vec![],
            r#type: None,
        };

        let tag = Line::peek(record).unwrap().tag;
        if tag == "BIRT" {
            Line::parse(record).unwrap();
        }

        while !record.is_empty() {
            let mut parse = true;
            let mut line = Line::peek(record).unwrap();
            match line.tag {
                "ADDR" => {
                    birth.address = Some(Address::parse(record).unwrap());
                    parse = false;
                }
                "AGE" => {
                    birth.age = Some(line.value.to_string());
                }
                "AGNC" => {
                    birth.agency = Some(line.value.to_string());
                }
                "CAUS" => {
                    birth.cause = Some(line.value.to_string());
                }
                "DATE" => {
                    birth.date = Some(line.value.to_string());
                }
                "FAMC" => {
                    let famc = Family {
                        xref: line.value.to_string(),
                    };
                    birth.family = Some(famc);
                }
                "NOTE" => {
                    birth.note = parse::get_tag_value(record).unwrap();
                    parse = false;
                }
                "OBJE" => {
                    let obj = Object {
                        xref: line.value.to_string(),
                    };
                    birth.media.push(obj);
                }
                "PLAC" => {
                    birth.place = Some(Place::parse(record).unwrap());
                    parse = false;
                }
                "RELI" => {
                    birth.religion = Some(line.value.to_string());
                }
                "SOUR" => {
                    let sc = SourceCitation::parse(record).unwrap();
                    birth.sources.push(sc);
                    parse = false;
                }
                "TYPE" => {
                    birth.r#type = Some(line.value.to_string());
                }
                _ => {}
            }

            if parse {
                line = Line::parse(record).unwrap();
            }
            if line.level == 1 {
                break;
            }
        }

        Ok(birth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests a possible bug in Ancestry's format, if a line break is embedded within the content of a note
    /// As far as I can tell, it's a \n embedded into the note, at least, from a hex dump of that content.
    fn parse_birth() {
        let data = vec![
            "1 BIRT",
            "2 TYPE Normal",
            "2 DATE 31 DEC 1965",
            "2 PLAC Salt Lake City, UT, USA",
            "3 FONE Salt Lake City, UT, USA",
            "4 TYPE user defined",
            "3 ROMN Salt Lake City, UT, USA",
            "4 TYPE user defined",
            "3 MAP",
            "4 LATI N0",
            "4 LONG E0",
            "3 NOTE Some place notes.",
            "2 ADDR",
            "3 ADR1 St. Marks Hospital",
            "3 CITY Salt Lake City",
            "3 STAE UT",
            "3 POST 84121",
            "3 CTRY USA",
            "2 AGNC none",
            "2 RELI Religion",
            "2 CAUS Conception",
            "2 NOTE Some notes.",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 EVEN BIRT",
            "4 ROLE CHIL",
            "3 DATA",
            "4 DATE 1 JAN 1900",
            "4 TEXT Here is some text from the source specific to this source ",
            "5 CONC citation.",
            "5 CONT Here is more text but on a new line.",
            "3 OBJE @M8@",
            "3 NOTE Some notes about this birth source citation which are embedded in the citation ",
            "4 CONC structure itself.",
            "3 QUAY 2",
            "2 OBJE @M15@",
            "2 AGE 0y",
            "2 FAMC @F2@",
        ].join("\n");

        let mut record = data.as_str();
        let mut birth = Birth::parse(&mut record).unwrap();

        assert!(birth.date.is_some());
        assert!(birth.r#type.is_some());

        let place = birth.place.unwrap();
        assert!(place.name.is_some());
        assert!(place.note.is_some());
        assert!(place.note.unwrap().note.unwrap() == "Some place notes.");

        let addr = birth.address.unwrap();
        assert!(addr.addr1.is_some());
        assert!(addr.city.is_some());
        assert!(addr.state.is_some());

        assert!(birth.agency.is_some());
        assert!(birth.agency.unwrap() == "none");

        assert!(birth.religion.is_some());
        assert!(birth.religion.unwrap() == "Religion");

        assert!(birth.cause.is_some());
        assert!(birth.cause.unwrap() == "Conception");

        // assert!(birth.event_type_cited_from.is_some());
        // let event_type = birth.event_type_cited_from.unwrap();
        // assert!(event_type.r#type.unwrap() == "BIRT");
        // assert!(event_type.role.unwrap() == "CHIL");

        assert!(birth.note.is_some());
        assert!(birth.note.unwrap() == "Some notes.");

        // assert!(place.name.unwrap() == "");

        assert!(birth.media.len() == 1);
        let obje = birth.media.pop().unwrap();
        assert!(obje.xref == "@M15@");

        assert!(birth.age.unwrap() == "0y");

        assert!(birth.family.unwrap().xref == "@F2@");
    }
}
