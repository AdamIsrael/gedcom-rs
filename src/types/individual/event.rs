// Individual Event Detail
// This is specific to Individual-related events such as birth or death,
// building off of types.EventDetail

use winnow::prelude::*;

use crate::types::{EventDetail, Line};

// INDIVIDUAL_EVENT_DETAIL:=
// n <<EVENT_DETAIL>> {1:1} p.32
// n AGE <AGE_AT_EVENT> {0:1} p.42

#[derive(Clone, Debug, Default)]
pub struct IndividualEventDetail {
    pub age: Option<String>,
    pub detail: EventDetail,
}

impl IndividualEventDetail {
    /// Initialize an empty Individual Event Detail
    pub fn new() -> IndividualEventDetail {
        IndividualEventDetail {
            age: None,
            detail: EventDetail {
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
            },
        }
    }

    pub fn parse(record: &mut &str) -> PResult<IndividualEventDetail> {
        let mut event = IndividualEventDetail {
            age: None,
            detail: EventDetail {
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
            },
        };

        let line = Line::parse(record).unwrap();
        let level = line.level;
        let mut events: Vec<String> = vec![];

        // Add the first line so EventDetails will parse cleanly
        events.push(line.to_string());

        while !record.is_empty() {
            let line = Line::peek(record).unwrap();
            if line.level <= level {
                break;
            }

            match line.tag {
                "AGE" => {
                    event.age = Some(line.value.to_string());
                }
                _ => {
                    // This works right now, in this use-case, but what if a struct
                    // composites more than one structure?

                    // add the line to events, so we can parse them all at once
                    // as part of the Event Detail
                    events.push(line.to_string());
                }
            }
            Line::parse(record).unwrap();
        }

        // Now parse the events
        if !events.is_empty() {
            // Remove the last line; it belongs to the next record
            let event_s = events.join("\n");
            let mut event_str = event_s.as_str();
            event.detail = EventDetail::parse(&mut event_str).unwrap();
        }

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use super::IndividualEventDetail;

    #[test]
    // Parse a Individual Event Detail that's not part of an ADOP record
    fn parse_individual_event_detail() {
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
            "3 NOTE Some notes about this adoption source citation which are embedded in the citation ",
            "4 CONC structure itself.",
            "3 QUAY 2",
            "2 OBJE @M15@",
            "2 AGE 0y",
            "2 FAMC @F2@",
        ].join("\n");

        let mut record = data.as_str();

        let e = IndividualEventDetail::parse(&mut record);

        assert!(e.is_ok());
        let event = e.unwrap();

        assert!(event.age == Some("0y".to_string()));

        let mut detail = event.detail;

        assert!(detail.r#type.is_some());
        assert!(detail.r#type.unwrap() == "Normal");

        assert!(detail.date.is_some());
        assert!(detail.date.unwrap() == "31 DEC 1965");

        assert!(detail.place.is_some());
        assert!(detail.place.unwrap().name == Some("Salt Lake City, UT, USA".to_string()));

        let addr = detail.address.unwrap();
        assert!(addr.addr1.is_some());
        assert!(addr.city.is_some());
        assert!(addr.state.is_some());

        assert!(detail.agency.is_some());
        assert!(detail.agency.unwrap() == "none");

        assert!(detail.religion.is_some());
        assert!(detail.religion.unwrap() == "Religion");

        assert!(detail.cause.is_some());
        assert!(detail.cause.unwrap() == "Conception");

        assert!(detail.note.is_some());
        assert!(detail.note == Some("Some notes.".to_string()));

        assert!(detail.media.len() == 1);
        let obje = detail.media.pop().unwrap();
        assert!(obje.xref == Some("@M15@".to_string()));

    }
}
