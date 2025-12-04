/// This is a template of a Type
use crate::parse;
use crate::types::{Address, Line, Object, Place, SourceCitation, Spouse};

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

#[derive(Clone, Debug, Default)]
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
        let mut event = EventDetail::default();

        let Ok(mut line) = Line::peek(record) else {
            return Ok(event);
        };

        // Check if we've received a top-level event tag, which we want to skip over.
        match line.tag {
            "ADOP" | "BAPM" | "BARM" | "BASM" | "BIRT" | "BLES" | "BURI" | "CENS" | "CHR"
            | "CHRA" | "CONF" | "CREM" | "DEAT" | "EMIG" | "EVEN" | "FCOM" | "GRAD" | "IMMI"
            | "ORDN" | "PROB" | "NATU" | "RETI" | "WILL" => {
                // Consume the current line
                let _ = Line::parse(record);
                // Get the next line
                if let Ok(next_line) = Line::peek(record) {
                    line = next_line;
                } else {
                    return Ok(event);
                }
            }
            _ => {}
        }

        let level = line.level;

        while !record.is_empty() {
            let mut parse = true;
            match line.tag {
                "ADDR" => {
                    if let Ok(addr) = Address::parse(record) {
                        event.address = Some(addr);
                    }
                    parse = false;
                }
                // "AGE" => {
                //     event.age = Some(line.value.to_string());
                // }
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
                    event.note = parse::get_tag_value(record).ok().flatten();
                    parse = false;
                }
                "OBJE" => {
                    let obj = Object {
                        xref: Some(line.value.to_string()),
                    };
                    event.media.push(obj);
                }
                "PLAC" => {
                    if let Ok(place) = Place::parse(record) {
                        event.place = Some(place);
                    }
                    parse = false;
                }
                "RELI" => {
                    event.religion = Some(line.value.to_string());
                }
                "SOUR" => {
                    if let Ok(sc) = SourceCitation::parse(record) {
                        event.sources.push(sc);
                    }
                    parse = false;
                }
                "TYPE" => {
                    event.r#type = Some(line.value.to_string());
                }
                _ => {
                    // TODO: Need to collect and parse these lines. They seem to
                    // correspond to INDIVIDUAL_ATTRIBUTE_STRUCTURE.
                    // Not sure _where_ to parse them to, though.

                    // println!("Skipping unknown tag: {}", line.tag);
                }
            }

            if parse {
                let _ = Line::parse(record);
            }

            let Ok(next_line) = Line::peek(record) else {
                break;
            };
            line = next_line;
            if line.level < level {
                break;
            }
        }

        Ok(event)
    }
}

// FAMILY_EVENT_DETAIL:=
// n HUSB
// +1 AGE <AGE_AT_EVENT>
// n WIFE
// +1 AGE <AGE_AT_EVENT>
// n <<EVENT_DETAIL>>

#[derive(Clone, Debug, Default)]
pub struct FamilyEventDetail {
    // Xref of husband
    pub husband: Option<Spouse>,
    // Xref of wife
    pub wife: Option<Spouse>,
    pub detail: Option<EventDetail>,
}

impl FamilyEventDetail {
    pub fn parse(record: &mut &str) -> PResult<FamilyEventDetail> {
        let mut event = FamilyEventDetail::default();

        // TODO: Check the first line for and see if it's a top-level event tag

        let mut events: Vec<String> = vec![];

        while !record.is_empty() {
            // let mut parse = true;
            let Ok(line) = Line::peek(record) else {
                break;
            };
            match line.tag {
                "HUSB" => {
                    if let Ok(spouse) = Spouse::parse(record) {
                        event.husband = Some(spouse);
                    }
                }
                "WIFE" => {
                    if let Ok(spouse) = Spouse::parse(record) {
                        event.wife = Some(spouse);
                    }
                }
                _ => {
                    // Store the remaining lines to be parsed as EventDetail
                    events.push(line.to_string());
                    let _ = Line::parse(record);
                }
            }
        }
        if let Ok(detail) = EventDetail::parse(&mut events.join("\n").as_str()) {
            event.detail = Some(detail);
        }

        Ok(event)
    }
}

// +1 EVEN <EVENT_TYPE_CITED_FROM>
// +2 ROLE <ROLE_IN_EVENT>
//
// "3 EVEN BIRT",
// "4 ROLE CHIL",

#[derive(Clone, Debug, Default)]
pub struct EventTypeCitedFrom {
    pub r#type: Option<String>,
    pub role: Option<String>,
}

impl EventTypeCitedFrom {
    /// Parse
    pub fn parse(record: &mut &str) -> PResult<EventTypeCitedFrom> {
        let mut event = EventTypeCitedFrom::default();
        let Ok(level_line) = Line::peek(record) else {
            return Ok(event);
        };
        let level = level_line.level;

        while !record.is_empty() {
            let Ok(mut line) = Line::parse(record) else {
                break;
            };
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
            let Ok(peek_line) = Line::peek(record) else {
                break;
            };
            line = peek_line;
            if line.level == level {
                break;
            }
        }

        Ok(event)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_event_detail() {
        let data = vec![
            "1 BAPM",
            "2 DATE ABT 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE BAPM",
            "2 ADDR",
            "3 ADR1 Church Name",
            "3 ADR2 Street Address",
            "3 CITY City Name",
            "3 POST zip",
            "3 CTRY Country",
            "2 CAUS Birth",
            "2 AGNC The Church",
            "2 OBJE @M8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Sample baptism Source text.",
            "3 QUAY 3",
            "3 NOTE A baptism source note.",
            "2 NOTE A baptism event note (the event of baptism (not LDS), performed in infancy or later. See also BAPL and CHR).",
        ].join("\n");
        let mut record = data.as_str();
        let detail = EventDetail::parse(&mut record).unwrap();

        assert!(detail.date.is_some());
        assert!(detail.date.unwrap() == "ABT 31 DEC 1997");

        assert!(detail.place.is_some());
        assert!(detail.place.unwrap().name.unwrap() == "The place");
        assert!(detail.r#type.is_some());
        assert!(detail.address.is_some());
        assert!(detail.agency.is_some());
        assert!(detail.media.len() == 1);
        assert!(detail.sources.len() == 1);
        assert!(detail.note.is_some());

        // assert!(detail.age.is_some());
    }

    #[test]
    fn parse_family_event_detail() {
        let data = vec![
            "1 MARR",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE Man and Wife",
            "2 ADDR",
            "3 ADR1 A Church",
            "3 ADR2 Main Street",
            "3 CTRY USA",
            "2 CAUS Love",
            "2 AGNC Catholic Church",
            "2 HUSB",
            "3 AGE 42y",
            "2 WIFE",
            "3 AGE 42y 6m",
            "2 OBJE @M8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Text from marriage source.",
            "3 QUAY 3",
            "3 NOTE A note about the marriage source.",
            "2 NOTE Marriage event note (a legal, common-law, or customary event of creating a family",
            "3 CONC unit of a man and a woman as husband and wife).",
        ].join("\n");
        let mut record = data.as_str();
        let event = FamilyEventDetail::parse(&mut record).unwrap();
        // println!("Event: {:?}", event);

        assert!(event.husband.is_some());
        let husband = event.husband.unwrap();
        assert!(husband.age.is_some());
        assert!(husband.age.unwrap() == "42y");

        assert!(event.wife.is_some());
        let wife = event.wife.unwrap();
        assert!(wife.age.is_some());
        assert!(wife.age.unwrap() == "42y 6m");

        assert!(event.detail.is_some());
        // assert!(event.wife.is_some());

        // assert!(event.detail.date.is_some());
        // assert!(event.detail.date.unwrap() == "31 DEC 1900");
    }

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
