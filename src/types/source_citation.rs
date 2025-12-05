use std::str::FromStr;

use crate::parse;

use super::{EventTypeCitedFrom, Line, Note, Object, Quay};

use winnow::prelude::*;

// [ /* pointer to source record (preferred)*/
// n SOUR @<XREF:SOUR>@ {1:1} p.27
// +1 PAGE <WHERE_WITHIN_SOURCE> {0:1} p.64
// +1 EVEN <EVENT_TYPE_CITED_FROM> {0:1} p.49
// +2 ROLE <ROLE_IN_EVENT> {0:1} p.61
// +1 DATA {0:1}
// +2 DATE <ENTRY_RECORDING_DATE> {0:1} p.48
// +2 TEXT <TEXT_FROM_SOURCE> {0:M} p.63
// +3 [CONC|CONT] <TEXT_FROM_SOURCE> {0:M}
// +1 <<MULTIMEDIA_LINK>> {0:M} p.37, 26
// +1 <<NOTE_STRUCTURE>> {0:M} p.37
// +1 QUAY <CERTAINTY_ASSESSMENT> {0:1} p.43

// | /* Systems not using source records */
// n SOUR <SOURCE_DESCRIPTION> {1:1} p.61
// +1 [CONC|CONT] <SOURCE_DESCRIPTION> {0:M}
// +1 TEXT <TEXT_FROM_SOURCE> {0:M} p.63
// +2 [CONC|CONT] <TEXT_FROM_SOURCE> {0:M}
// +1 <<MULTIMEDIA_LINK>> {0:M} p.37, 26
// +1 <<NOTE_STRUCTURE>> {0:M} p.37
// +1 QUAY <CERTAINTY_ASSESSMENT> {0:1} p.43
// ]

#[derive(Clone, Debug, Default)]
pub struct SourceCitation {
    pub xref: Option<String>,
    pub page: Option<i32>,
    pub event: Option<EventTypeCitedFrom>,
    pub data: Option<SourceCitationData>,
    pub media: Vec<Object>,
    pub note: Option<Note>,
    pub quay: Option<Quay>,
}

impl SourceCitation {
    pub fn parse(record: &mut &str) -> PResult<SourceCitation> {
        let mut sc = SourceCitation::default();

        let Ok(level_line) = Line::peek(record) else {
            return Ok(sc);
        };
        let level = level_line.level;
        let Ok(mut line) = Line::peek(record) else {
            return Ok(sc);
        };

        while !record.is_empty() {
            let mut consume = true;
            match line.tag {
                "DATA" => {
                    if let Ok(data) = SourceCitationData::parse(record) {
                        sc.data = Some(data);
                    }
                    consume = false;
                }
                "EVEN" => {
                    if let Ok(event) = EventTypeCitedFrom::parse(record) {
                        sc.event = Some(event);
                    }
                    consume = false;
                }
                "NOTE" => {
                    if let Ok(note) = parse::get_tag_value(record) {
                        sc.note = Some(Note { note });
                    }
                    consume = false;
                }
                "OBJE" => {
                    let obj = Object {
                        xref: Some(line.value.to_string()),
                    };
                    sc.media.push(obj);
                }
                "PAGE" => {
                    if let Ok(page) = line.value.parse() {
                        sc.page = Some(page);
                    }
                }
                "QUAY" => {
                    if let Ok(quay) = Quay::from_str(line.value) {
                        sc.quay = Some(quay);
                    }
                }
                "SOUR" => {
                    sc.xref = Some(line.value.to_string());
                }
                _ => {}
            }

            if consume {
                let _ = Line::parse(record);
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

        Ok(sc)
    }
}

#[derive(Clone, Debug, Default)]
pub struct SourceCitationData {
    pub date: Option<String>,
    pub text: Option<Note>,
}
impl SourceCitationData {
    pub fn parse(record: &mut &str) -> PResult<SourceCitationData> {
        let mut data = SourceCitationData::default();

        let Ok(level_line) = Line::peek(record) else {
            return Ok(data);
        };
        let level = level_line.level;
        let Ok(mut line) = Line::peek(record) else {
            return Ok(data);
        };

        while !record.is_empty() {
            let mut consume = true;
            match line.tag {
                "DATE" => {
                    data.date = Some(line.value.to_string());
                }
                "TEXT" => {
                    if let Ok(text) = parse::get_tag_value(record) {
                        let note = Note { note: text };
                        data.text = Some(note);
                    }
                    consume = false;
                }
                _ => {}
            }

            if consume {
                let _ = Line::parse(record);
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
        Ok(data)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_source_citation() {
        let data = vec![
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
        ].join("\n");

        let mut record = data.as_str();
        let detail = SourceCitation::parse(&mut record).unwrap();

        assert!(detail.data.is_some());
        let sc = detail.data.unwrap();
        assert!(sc.date.unwrap() == "1 JAN 1900");
        assert!(sc.text.unwrap().note.unwrap() == "Here is some text from the source specific to this source citation.\nHere is more text but on a new line.");

        assert!(detail.xref.is_some());
        assert!(detail.xref.unwrap() == "@S1@");

        assert!(detail.event.is_some());
        let event = detail.event.unwrap();
        assert!(event.r#type.unwrap() == "BIRT");
        assert!(event.role.unwrap() == "CHIL");

        assert!(detail.page.is_some());
        assert!(detail.page.unwrap() == 42);
    }
}
