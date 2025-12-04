use crate::types::{Family, Line};

use winnow::prelude::*;

use super::IndividualEventDetail;

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

#[derive(Clone, Debug, Default)]
pub struct Birth {
    pub event: IndividualEventDetail,
    pub family: Option<Family>,
}

impl Birth {
    pub fn parse(record: &mut &str) -> PResult<Birth> {
        let mut birth = Birth {
            event: IndividualEventDetail::new(),
            family: None,
        };

        let line = Line::parse(record)?;
        let level = line.level;
        // Pre-allocate capacity for typical event detail (avg ~10-15 lines)
        let mut events: Vec<String> = Vec::with_capacity(16);

        // Add the first line so EventDetails will parse cleanly
        events.push(line.to_string());

        while !record.is_empty() {
            let line = Line::peek(record)?;
            if line.level <= level {
                break;
            }

            match line.tag {
                "FAMC" => {
                    let famc = Family {
                        adopted_by: None,
                        husband: None,
                        wife: None,
                        xref: line.value.to_string(),
                        notes: vec![],
                        pedigree: None,
                    };
                    birth.family = Some(famc);
                }
                _ => {
                    // This works right now, in this use-case, but what if a struct
                    // composites more than one structure?

                    // add the line to events, so we can parse them all at once
                    // as part of the Event Detail
                    events.push(line.to_string());
                }
            }

            Line::parse(record)?;
        }

        // Now parse the events
        if !events.is_empty() {
            // Remove the last line; it belongs to the next record
            let event = events.join("\n");
            let mut event_str = event.as_str();
            birth.event = IndividualEventDetail::parse(&mut event_str)?;
        }

        Ok(birth)
    }
}

#[allow(clippy::unwrap_used)]
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
        let birth = Birth::parse(&mut record).unwrap();

        let mut event = birth.event;
        assert!(event.detail.date.is_some());
        assert!(event.detail.r#type.is_some());

        let place = event.detail.place.unwrap();
        assert!(place.name.is_some());
        assert!(place.note.is_some());
        assert!(place.note.unwrap().note.unwrap() == "Some place notes.");

        let addr = event.detail.address.unwrap();
        assert!(addr.addr1.is_some());
        assert!(addr.city.is_some());
        assert!(addr.state.is_some());

        assert!(event.detail.agency.is_some());
        assert!(event.detail.agency.unwrap() == "none");

        assert!(event.detail.religion.is_some());
        assert!(event.detail.religion.unwrap() == "Religion");

        assert!(event.detail.cause.is_some());
        assert!(event.detail.cause.unwrap() == "Conception");

        assert!(event.detail.note.is_some());
        assert!(event.detail.note.unwrap() == "Some notes.");

        assert!(event.detail.media.len() == 1);
        let obje = event.detail.media.pop().unwrap();
        assert!(obje.xref == Some("@M15@".to_string()));

        assert!(event.age.unwrap() == "0y");

        assert!(birth.family.unwrap().xref == "@F2@");
    }
}
