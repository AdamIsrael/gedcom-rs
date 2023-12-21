/// This is a template of a Type
use crate::parse;
use crate::types::{Address, Line, Object, Place, SourceCitation};

use winnow::prelude::*;

// The GEDCOM specification of this type
//
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

#[derive(Debug, Default)]
pub struct EventDetail {
    pub r#type: Option<String>,
    pub date: Option<String>,
    pub place: Option<Place>,
    pub address: Option<Address>,
    pub agency: Option<String>,
    pub religion: Option<String>,
    pub cause: Option<String>,
    pub restriction_notice: Option<String>,
    pub note: Option<String>,
    pub sources: Vec<SourceCitation>,
    pub media: Vec<Object>,
}

impl EventDetail {
    /// Parse
    pub fn parse(record: &mut &str) -> PResult<EventDetail> {
        let mut event = EventDetail {
            r#type: None,
            date: None,
            place: None,
            address: None,
            agency: None,
            religion: None,
            cause: None,
            restriction_notice: None,
            note: None,
            sources: vec![],
            media: vec![],
        };

        while !record.is_empty() {
            let mut parse = true;
            let line = Line::peek(record).unwrap();
            match line.tag {
                "ADDR" => {
                    event.address = Some(Address::parse(record).unwrap());
                    parse = false;
                }
                "AGNC" => {
                    event.agency = Some(line.value.to_string());
                }
                "CAUS" => {
                    event.cause = Some(line.value.to_string());
                }
                "DATE" => {
                    event.date = Some(line.value.to_string());
                }
                "NOTE" => {
                    event.note = parse::get_tag_value(record).unwrap();
                    parse = false;
                }
                "OBJE" => {
                    let obj = Object {
                        xref: line.value.to_string(),
                    };
                    event.media.push(obj);
                }
                "PLAC" => {
                    event.place = Some(Place::parse(record).unwrap());
                    parse = false;
                }
                "RELI" => {
                    event.religion = Some(line.value.to_string());
                }
                "SOUR" => {
                    let sc = SourceCitation::parse(record).unwrap();
                    event.sources.push(sc);
                    parse = false;
                }
                "TYPE" => {
                    event.r#type = Some(line.value.to_string());
                }
                _ => {}
            }

            if parse {
                Line::parse(record).unwrap();
            }
        }

        Ok(event)
    }
}

// +1 EVEN <EVENT_TYPE_CITED_FROM>
// +2 ROLE <ROLE_IN_EVENT>
//
// "3 EVEN BIRT",
// "4 ROLE CHIL",

#[derive(Debug, Default)]
pub struct EventTypeCitedFrom {
    pub r#type: Option<String>,
    pub role: Option<String>,
}

impl EventTypeCitedFrom {
    /// Parse
    pub fn parse(record: &mut &str) -> PResult<EventTypeCitedFrom> {
        let mut event = EventTypeCitedFrom {
            r#type: None,
            role: None,
        };
        let level = Line::peek(record).unwrap().level;

        while !record.is_empty() {
            let mut line = Line::parse(record).unwrap();
            match line.tag {
                "EVEN" => {
                    event.r#type = Some(line.value.to_string());
                }
                "ROLE" => {
                    event.role = Some(line.value.to_string());
                }
                _ => {}
            }

            // If the next level matches our initial level, we're done parsing
            // this structure.
            line = Line::peek(record).unwrap();
            if line.level == level {
                break;
            }
        }

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn parse_event_detail() {
    //     let data = vec!["1 NAME This is a name."];

    //     let input = data.join("\n");
    //     let mut record = input.as_str();
    //     let detail = EventDetail::parse(&mut record).unwrap();

    //     assert!(detail.name.is_some());
    // }

    #[test]
    fn parse_event_type_cited_from() {
        let data = vec!["3 EVEN BIRT", "4 ROLE CHIL"];

        let input = data.join("\n");
        let mut record = input.as_str();
        let event_type = EventTypeCitedFrom::parse(&mut record).unwrap();

        assert!(event_type.r#type.is_some());
        assert!(event_type.r#type.unwrap() == "BIRT");

        assert!(event_type.role.is_some());
        assert!(event_type.role.unwrap() == "CHIL");
    }
}
