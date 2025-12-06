use super::{Line, Note, Object, Xref};
use crate::parse;
use winnow::prelude::*;

// SOURCE_RECORD:=
//
// n @XREF:SOUR@ SOUR {1:1}
//   +1 DATA {0:1}
//     +2 EVEN <EVENTS_RECORDED> {0:M}
//       +3 DATE <DATE_PERIOD> {0:1}
//       +3 PLAC <SOURCE_JURISDICTION_PLACE> {0:1}
//     +2 AGNC <RESPONSIBLE_AGENCY> {0:1}
//     +2 <<NOTE_STRUCTURE>> {0:M}
//   +1 AUTH <SOURCE_ORIGINATOR> {0:1}
//     +2 [CONC|CONT] <SOURCE_ORIGINATOR> {0:M}
//   +1 TITL <SOURCE_DESCRIPTIVE_TITLE> {0:1}
//     +2 [CONC|CONT] <SOURCE_DESCRIPTIVE_TITLE> {0:M}
//   +1 ABBR <SOURCE_FILED_BY_ENTRY> {0:1}
//   +1 PUBL <SOURCE_PUBLICATION_FACTS> {0:1}
//     +2 [CONC|CONT] <SOURCE_PUBLICATION_FACTS> {0:M}
//   +1 TEXT <TEXT_FROM_SOURCE> {0:1}
//     +2 [CONC|CONT] <TEXT_FROM_SOURCE> {0:M}
//   +1 <<SOURCE_REPOSITORY_CITATION>> {0:1}
//   +1 REFN <USER_REFERENCE_NUMBER> {0:M}
//     +2 TYPE <USER_REFERENCE_TYPE> {0:1}
//   +1 RIN <AUTOMATED_RECORD_ID> {0:1}
//   +1 <<CHANGE_DATE>> {0:1}
//   +1 <<NOTE_STRUCTURE>> {0:M}
//   +1 <<MULTIMEDIA_LINK>> {0:M}

/// Represents a SOURCE_RECORD at level 0 in a GEDCOM file
#[derive(Clone, Debug, Default)]
pub struct SourceRecord {
    /// Cross-reference identifier for this source
    pub xref: Option<Xref>,

    /// Title of the source
    pub title: Option<String>,

    /// Abbreviated title
    pub abbreviation: Option<String>,

    /// Author or originator of the source
    pub author: Option<String>,

    /// Publication facts for the source
    pub publication: Option<String>,

    /// Text from the source
    pub text: Option<String>,

    /// Repository where the source is stored
    pub repository_xref: Option<Xref>,

    /// Repository call number
    pub call_number: Option<String>,

    /// Source data events recorded
    pub data: Option<SourceRecordData>,

    /// Notes about this source
    pub notes: Vec<Note>,

    /// Multimedia objects linked to this source
    pub media: Vec<Object>,

    /// User reference numbers
    pub user_reference_numbers: Vec<UserReference>,

    /// Automated record ID
    pub automated_record_id: Option<String>,

    /// Change date (not yet fully parsed)
    pub change_date: Option<String>,
}

/// Data about events recorded in a source
#[derive(Clone, Debug, Default)]
pub struct SourceRecordData {
    /// Events recorded in this source
    pub events: Vec<SourceDataEvent>,

    /// Responsible agency
    pub agency: Option<String>,

    /// Notes about the data
    pub notes: Vec<Note>,
}

/// An event recorded in source data
#[derive(Clone, Debug, Default)]
pub struct SourceDataEvent {
    /// Type of event(s) recorded (e.g., "BIRT, CHR")
    pub events_recorded: String,

    /// Date period for these events
    pub date_period: Option<String>,

    /// Place jurisdiction for these events
    pub place: Option<String>,
}

/// User reference number with optional type
#[derive(Clone, Debug, Default)]
pub struct UserReference {
    pub number: String,
    pub ref_type: Option<String>,
}

impl SourceRecord {
    /// Parse a SOURCE_RECORD starting at level 0
    pub fn parse(input: &mut &str) -> PResult<SourceRecord> {
        let mut source = SourceRecord::default();

        // Parse the level 0 line to get xref
        let Ok(level_line) = Line::parse(input) else {
            return Ok(source);
        };

        if level_line.tag != "SOUR" {
            return Ok(source);
        }

        // Extract xref from level_line.xref (format: @S1@)
        if !level_line.xref.is_empty() {
            source.xref = Some(Xref::new(level_line.xref));
        }

        // Parse level 1+ tags
        let Ok(mut line) = Line::peek(input) else {
            return Ok(source);
        };

        while !input.is_empty() && line.level > 0 {
            let mut consume = true;

            match line.tag {
                "TITL" => {
                    if let Ok(title) = parse::get_tag_value(input) {
                        source.title = title;
                    }
                    consume = false;
                }
                "ABBR" => {
                    source.abbreviation = Some(line.value.to_string());
                }
                "AUTH" => {
                    if let Ok(author) = parse::get_tag_value(input) {
                        source.author = author;
                    }
                    consume = false;
                }
                "PUBL" => {
                    if let Ok(publ) = parse::get_tag_value(input) {
                        source.publication = publ;
                    }
                    consume = false;
                }
                "TEXT" => {
                    if let Ok(text) = parse::get_tag_value(input) {
                        source.text = text;
                    }
                    consume = false;
                }
                "REPO" => {
                    // Repository reference
                    source.repository_xref = Some(Xref::new(line.value));
                    // Consume this line
                    let _ = Line::parse(input);

                    // Check for CALN (call number) at next level
                    if let Ok(next_line) = Line::peek(input) {
                        if next_line.tag == "CALN" && next_line.level > line.level {
                            let _ = Line::parse(input);
                            source.call_number = Some(next_line.value.to_string());
                        }
                    }
                    consume = false;
                }
                "DATA" => {
                    if let Ok(data) = SourceRecordData::parse(input) {
                        source.data = Some(data);
                    }
                    consume = false;
                }
                "NOTE" => {
                    if let Ok(Some(text)) = parse::get_tag_value(input) {
                        source.notes.push(Note { note: Some(text) });
                    }
                    consume = false;
                }
                "OBJE" => {
                    // Multimedia link
                    let obj = Object {
                        xref: Some(line.value.to_string()),
                    };
                    source.media.push(obj);
                }
                "REFN" => {
                    let number = line.value.to_string();
                    let _ = Line::parse(input);

                    // Check for TYPE at next level
                    let mut ref_type = None;
                    if let Ok(next_line) = Line::peek(input) {
                        if next_line.tag == "TYPE" && next_line.level > line.level {
                            let _ = Line::parse(input);
                            ref_type = Some(next_line.value.to_string());
                        }
                    }

                    source
                        .user_reference_numbers
                        .push(UserReference { number, ref_type });
                    consume = false;
                }
                "RIN" => {
                    source.automated_record_id = Some(line.value.to_string());
                }
                "CHAN" => {
                    // For now, just skip CHAN and its children
                    // TODO: Implement full CHANGE_DATE parsing
                    let chan_level = line.level;
                    let _ = Line::parse(input);

                    // Skip all child tags
                    while let Ok(peek) = Line::peek(input) {
                        if peek.level <= chan_level {
                            break;
                        }
                        let _ = Line::parse(input);
                    }
                    consume = false;
                }
                _ => {
                    // Unknown tag - skip it
                }
            }

            if consume {
                let _ = Line::parse(input);
            }

            // Peek at next line
            let Ok(peek_line) = Line::peek(input) else {
                break;
            };
            line = peek_line;

            // Stop if we've reached the next level 0 record
            if line.level == 0 {
                break;
            }
        }

        Ok(source)
    }
}

impl SourceRecordData {
    /// Parse DATA tag and its children
    fn parse(input: &mut &str) -> PResult<SourceRecordData> {
        let mut data = SourceRecordData::default();

        let Ok(level_line) = Line::peek(input) else {
            return Ok(data);
        };

        if level_line.tag != "DATA" {
            return Ok(data);
        }

        let data_level = level_line.level;
        let _ = Line::parse(input); // Consume DATA line

        let Ok(mut line) = Line::peek(input) else {
            return Ok(data);
        };

        while !input.is_empty() && line.level > data_level {
            let mut consume = true;

            match line.tag {
                "EVEN" => {
                    if let Ok(event) = SourceDataEvent::parse(input) {
                        data.events.push(event);
                    }
                    consume = false;
                }
                "AGNC" => {
                    data.agency = Some(line.value.to_string());
                }
                "NOTE" => {
                    if let Ok(Some(text)) = parse::get_tag_value(input) {
                        data.notes.push(Note { note: Some(text) });
                    }
                    consume = false;
                }
                _ => {}
            }

            if consume {
                let _ = Line::parse(input);
            }

            let Ok(peek_line) = Line::peek(input) else {
                break;
            };
            line = peek_line;

            if line.level <= data_level {
                break;
            }
        }

        Ok(data)
    }
}

impl SourceDataEvent {
    /// Parse EVEN tag and its children
    fn parse(input: &mut &str) -> PResult<SourceDataEvent> {
        let mut event = SourceDataEvent::default();

        let Ok(level_line) = Line::parse(input) else {
            return Ok(event);
        };

        if level_line.tag != "EVEN" {
            return Ok(event);
        }

        event.events_recorded = level_line.value.to_string();
        let even_level = level_line.level;

        let Ok(mut line) = Line::peek(input) else {
            return Ok(event);
        };

        while !input.is_empty() && line.level > even_level {
            match line.tag {
                "DATE" => {
                    event.date_period = Some(line.value.to_string());
                }
                "PLAC" => {
                    event.place = Some(line.value.to_string());
                }
                _ => {}
            }

            let _ = Line::parse(input);

            let Ok(peek_line) = Line::peek(input) else {
                break;
            };
            line = peek_line;

            if line.level <= even_level {
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
    fn parse_basic_source_record() {
        let data = vec![
            "0 @S1@ SOUR",
            "1 TITL Everything You Every Wanted to Know about GEDCOM Tags",
            "1 ABBR All About GEDCOM Tags",
            "1 AUTH Author Name",
        ]
        .join("\n");

        let mut input = data.as_str();
        let source = SourceRecord::parse(&mut input).unwrap();

        assert!(source.xref.is_some());
        assert_eq!(source.xref.unwrap().to_string(), "@S1@");
        assert_eq!(
            source.title,
            Some("Everything You Every Wanted to Know about GEDCOM Tags".to_string())
        );
        assert_eq!(
            source.abbreviation,
            Some("All About GEDCOM Tags".to_string())
        );
        assert_eq!(source.author, Some("Author Name".to_string()));
    }

    #[test]
    fn parse_source_with_conc_cont() {
        let data = vec![
            "0 @S1@ SOUR",
            "1 TITL Everything You Every Wanted to Know about GEDCOM Tags, But",
            "2 CONC Were Afraid to Ask!",
            "2 CONT You can start new lines in this field too.",
            "1 AUTH Author or Authors of this Source using multiple lines if",
            "2 CONC necessary.",
            "2 CONT Here is a new line in this field",
        ]
        .join("\n");

        let mut input = data.as_str();
        let source = SourceRecord::parse(&mut input).unwrap();

        assert_eq!(
            source.title,
            Some("Everything You Every Wanted to Know about GEDCOM Tags, ButWere Afraid to Ask!\nYou can start new lines in this field too.".to_string())
        );
        assert_eq!(
            source.author,
            Some("Author or Authors of this Source using multiple lines ifnecessary.\nHere is a new line in this field".to_string())
        );
    }

    #[test]
    fn parse_source_with_repo() {
        let data = vec![
            "0 @S1@ SOUR",
            "1 TITL Source Title",
            "1 REPO @R1@",
            "2 CALN 920.23",
        ]
        .join("\n");

        let mut input = data.as_str();
        let source = SourceRecord::parse(&mut input).unwrap();

        assert!(source.repository_xref.is_some());
        assert_eq!(source.repository_xref.unwrap().to_string(), "@R1@");
        assert_eq!(source.call_number, Some("920.23".to_string()));
    }

    #[test]
    fn parse_source_with_data() {
        let data = vec![
            "0 @S1@ SOUR",
            "1 TITL Source Title",
            "1 DATA",
            "2 EVEN BIRT, CHR",
            "3 DATE FROM 1 JAN 1980 TO 1 FEB 1982",
            "3 PLAC Anytown, Anycounty, USA",
            "2 EVEN DEAT",
            "3 DATE FROM 1 JAN 1980 TO 1 FEB 1982",
            "3 PLAC County Some, Ireland",
            "2 AGNC Responsible agency for data in this source",
        ]
        .join("\n");

        let mut input = data.as_str();
        let source = SourceRecord::parse(&mut input).unwrap();

        assert!(source.data.is_some());
        let data = source.data.unwrap();
        assert_eq!(data.events.len(), 2);
        assert_eq!(data.events[0].events_recorded, "BIRT, CHR");
        assert_eq!(
            data.events[0].date_period,
            Some("FROM 1 JAN 1980 TO 1 FEB 1982".to_string())
        );
        assert_eq!(
            data.events[0].place,
            Some("Anytown, Anycounty, USA".to_string())
        );
        assert_eq!(data.events[1].events_recorded, "DEAT");
        assert_eq!(
            data.agency,
            Some("Responsible agency for data in this source".to_string())
        );
    }

    #[test]
    fn parse_source_with_notes_and_media() {
        let data = vec![
            "0 @S1@ SOUR",
            "1 TITL Source Title",
            "1 NOTE This is a note about the source",
            "1 OBJE @M8@",
            "1 REFN 01234567890123456789",
            "2 TYPE reference",
            "1 RIN 1",
        ]
        .join("\n");

        let mut input = data.as_str();
        let source = SourceRecord::parse(&mut input).unwrap();

        assert_eq!(source.notes.len(), 1);
        assert_eq!(
            source.notes[0].note,
            Some("This is a note about the source".to_string())
        );
        assert_eq!(source.media.len(), 1);
        assert_eq!(source.media[0].xref, Some("@M8@".to_string()));
        assert_eq!(source.user_reference_numbers.len(), 1);
        assert_eq!(
            source.user_reference_numbers[0].number,
            "01234567890123456789"
        );
        assert_eq!(
            source.user_reference_numbers[0].ref_type,
            Some("reference".to_string())
        );
        assert_eq!(source.automated_record_id, Some("1".to_string()));
    }

    #[test]
    fn parse_complete_source_record() {
        let data = vec![
            "0 @S1@ SOUR",
            "1 TITL Everything You Every Wanted to Know about GEDCOM Tags, But",
            "2 CONC Were Afraid to Ask!",
            "2 CONT You can start new lines in this field too.",
            "1 ABBR All About GEDCOM Tags",
            "1 AUTH Author or Authors of this Source using multiple lines if",
            "2 CONC necessary.",
            "2 CONT Here is a new line in this field",
            "1 PUBL Details of the publisher of this source using multiple lines",
            "2 CONC if necessary.",
            "2 CONT Here is a new line in this field",
            "1 REPO @R1@",
            "2 CALN 920.23",
            "1 TEXT This section is used to generic text from the course. It will usually be a",
            "2 CONC quote from the text that is relevant to the use of this source in the current",
            "2 CONC GEDCOM file.",
            "2 CONT",
            "2 CONT It may use as many lines as needed.",
            "1 DATA",
            "2 EVEN BIRT, CHR",
            "3 DATE FROM 1 JAN 1980 TO 1 FEB 1982",
            "3 PLAC Anytown, Anycounty, USA",
            "2 EVEN DEAT",
            "3 DATE FROM 1 JAN 1980 TO 1 FEB 1982",
            "3 PLAC County Some, Ireland",
            "2 AGNC Responsible agency for data in this source",
            "2 NOTE A note about data in source.",
            "3 CONT",
            "3 CONT This note includes a blank line before this text. These notes are used to describe the",
            "3 CONC data in this source. Notes about the source itself are usually entered in a different set",
            "3 CONC of notes.",
            "1 NOTE These are notes embedded in the SOURCE Record instead of in a separate NOTE",
            "2 CONC RECORD.",
            "1 OBJE @M8@",
            "1 REFN 01234567890123456789",
            "2 TYPE reference",
            "1 RIN 1",
            "1 CHAN",
            "2 DATE 14 JAN 2001",
            "3 TIME 14:29:25",
        ]
        .join("\n");

        let mut input = data.as_str();
        let source = SourceRecord::parse(&mut input).unwrap();

        assert!(source.xref.is_some());
        assert!(source.title.is_some());
        assert!(source.abbreviation.is_some());
        assert!(source.author.is_some());
        assert!(source.publication.is_some());
        assert!(source.text.is_some());
        assert!(source.repository_xref.is_some());
        assert!(source.call_number.is_some());
        assert!(source.data.is_some());
        assert_eq!(source.notes.len(), 1);
        assert_eq!(source.media.len(), 1);
        assert_eq!(source.user_reference_numbers.len(), 1);
        assert!(source.automated_record_id.is_some());
    }
}
