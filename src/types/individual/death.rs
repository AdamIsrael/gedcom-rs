use crate::types::{EventDetail, Family, Line};

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
pub struct Death {
    pub age: Option<String>,
    pub event: Option<EventDetail>,
    pub family: Option<Family>,
}

impl Death {
    pub fn parse(record: &mut &str) -> PResult<Death> {
        let mut death = Death {
            age: None,
            event: None,
            family: None,
        };

        let line = Line::parse(record).unwrap();
        // TODO: This implies a death is known but the date is not.
        // Is this effective as-is? It'll create an empty death record, so
        // we have Some() in place, where if there is no death tag we would
        // have None()
        // 1 DEAT Y
        let mut events: Vec<String> = vec![];

        // Add the first line so EventDetails will parse cleanly
        events.push(line.to_string());

        while !record.is_empty() {
            let line = Line::peek(record).unwrap();
            if line.level == 1 {
                break;
            }
            match line.tag {
                "AGE" => {
                    death.age = Some(line.value.to_string());
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
            let event = events.join("\n");
            let mut event_str = event.as_str();
            death.event = Some(EventDetail::parse(&mut event_str).unwrap());
        }

        Ok(death)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests a possible bug in Ancestry's format, if a line break is embedded within the content of a note
    /// As far as I can tell, it's a \n embedded into the note, at least, from a hex dump of that content.
    fn parse_death() {
        let data = vec![
            "1 DEAT",
            "2 DATE ABT 15 JAN 2001",
            "2 PLAC New York, New York, USA",
            "3 NOTE The place structure has more detail than usually used for places",
            "2 AGE 76y",
            "2 TYPE slow",
            "2 ADDR",
            "3 ADR1 at Home",
            "2 CAUS Cancer",
            "2 AGNC none",
            "2 OBJE @M8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some death source text.",
            "3 QUAY 3",
            "3 NOTE A death source note.",
            "2 NOTE A death event note.",
        ]
        .join("\n");

        let mut record = data.as_str();
        let death = Death::parse(&mut record).unwrap();

        let mut event = death.event.unwrap();
        assert!(event.date.is_some());
        assert!(event.r#type.is_some());

        let place = event.place.unwrap();
        assert!(place.name.is_some());
        assert!(place.note.is_some());
        assert!(
            place.note.unwrap().note.unwrap()
                == "The place structure has more detail than usually used for places"
        );

        let addr = event.address.unwrap();
        assert!(addr.addr1.is_some());
        assert!(addr.city.is_none());
        assert!(addr.state.is_none());

        assert!(event.agency.is_some());
        assert!(event.agency.unwrap() == "none");

        assert!(event.religion.is_none());

        assert!(event.cause.is_some());
        assert!(event.cause.unwrap() == "Cancer");

        assert!(event.note.is_some());
        assert!(event.note.unwrap() == "A death event note.");

        assert!(event.media.len() == 1);
        let obje = event.media.pop().unwrap();
        assert!(obje.xref == Some("@M8@".to_string()));

        assert!(death.age.unwrap() == "76y");

        assert!(death.family.is_none());
    }
}
