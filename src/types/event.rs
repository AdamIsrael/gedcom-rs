/// This is a template of a Type
use crate::types::Line;

use winnow::prelude::*;

// The GEDCOM specification of this type
//
// PLACE_STRUCTURE:=
// n PLAC <PLACE_NAME> {1:1} p.58
// +1 FORM <PLACE_HIERARCHY> {0:1} p.58
// 39
// +1 FONE <PLACE_PHONETIC_VARIATION> {0:M} p.59
// +2 TYPE <PHONETIC_TYPE> {1:1} p.57
// +1 ROMN <PLACE_ROMANIZED_VARIATION> {0:M} p.59
// +2 TYPE <ROMANIZED_TYPE> {1:1} p.61
// +1 MAP {0:1}
// +2 LATI <PLACE_LATITUDE> {1:1} p.58
// +2 LONG <PLACE_LONGITUDE> {1:1} p.58
// +1 <<NOTE_STRUCTURE>> {0:M} p.37

// #[derive(Debug, Default)]
// pub struct EventDetail {
//     pub name: Option<String>,
// }

// impl EventDetail {
//     /// Parse
//     pub fn parse(record: &mut &str) -> PResult<EventDetail> {
//         let mut detail = EventDetail { name: None };
//         let level = Line::peek(record).unwrap().level;

//         while !record.is_empty() {
//             let mut line = Line::parse(record).unwrap();
//             match line.tag {
//                 "NAME" => {
//                     detail.name = Some(line.value.to_string());
//                 }
//                 _ => {}
//             }

//             // If the next level matches our initial level, we're done parsing
//             // this structure.
//             line = Line::peek(record).unwrap();
//             if line.level == level {
//                 break;
//             }
//         }

//         Ok(detail)
//     }
// }

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
