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
