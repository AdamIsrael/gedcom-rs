use crate::types::{Family, Line};

use winnow::prelude::*;

use super::IndividualEventDetail;

// n ADOP {1:1}
// +1 <<INDIVIDUAL_EVENT_DETAIL>> {0:1}* p.34
// +1 FAMC @<XREF:FAM>@ {0:1} p.24
//    +2 ADOP <ADOPTED_BY_WHICH_PARENT> {0:1} p.42

#[derive(Clone, Debug, Default)]
pub struct Adoption {
    pub event: IndividualEventDetail,
    pub family: Option<Family>,
}

impl Adoption {
    pub fn parse(record: &mut &str) -> PResult<Adoption> {
        let mut adoption = Adoption {
            event: IndividualEventDetail::new(),
            family: None,
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
            let mut consume = true;
            match line.tag {
                "FAMC" => {
                    let famc = Family::parse(record);
                    adoption.family = Some(famc);
                    consume = false;
                }
                _ => {
                    // This works right now, in this use-case, but what if a struct
                    // composites more than one structure?

                    // add the line to events, so we can parse them all at once
                    // as part of the Event Detail
                    events.push(line.to_string());
                }
            }
            // line = Line::parse(record).unwrap();
            if consume {
                Line::parse(record).unwrap();
            }
        }

        // Now parse the events
        if !events.is_empty() {
            // Remove the last line; it belongs to the next record
            // println!("DELETE: {:?}", events.pop());
            let event = events.join("\n");
            let mut event_str = event.as_str();
            // println!("parsing --\n{}", event_str);
            adoption.event = IndividualEventDetail::parse(&mut event_str).unwrap();
        }

        Ok(adoption)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests a possible bug in Ancestry's format, if a line break is embedded within the content of a note
    /// As far as I can tell, it's a \n embedded into the note, at least, from a hex dump of that content.
    fn parse_adoption() {
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
        let adoption = Adoption::parse(&mut record).unwrap();

        let mut event = adoption.event;
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

        // assert!(adoption.event_type_cited_from.is_some());
        // let event_type = adoption.event_type_cited_from.unwrap();
        // assert!(event_type.r#type.unwrap() == "BIRT");
        // assert!(event_type.role.unwrap() == "CHIL");

        assert!(event.detail.note.is_some());
        assert!(event.detail.note.unwrap() == "Some notes.");

        // assert!(place.name.unwrap() == "");

        assert!(event.detail.media.len() == 1);
        let obje = event.detail.media.pop().unwrap();
        assert!(obje.xref == "@M15@");

        assert!(event.age.unwrap() == "0y");

        assert!(adoption.family.unwrap().xref == "@F2@");
    }
}
