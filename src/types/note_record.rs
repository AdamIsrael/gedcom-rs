use super::{Line, SourceCitation, UserReference, Xref};
use winnow::prelude::*;

// NOTE_RECORD:=
//
// n @<XREF:NOTE>@ NOTE <SUBMITTER_TEXT> {1:1}
//   +1 [ CONC | CONT ] <SUBMITTER_TEXT> {0:M}
//   +1 <<SOURCE_CITATION>> {0:M}
//   +1 REFN <USER_REFERENCE_NUMBER> {0:M}
//     +2 TYPE <USER_REFERENCE_TYPE> {0:1}
//   +1 RIN <AUTOMATED_RECORD_ID> {0:1}
//   +1 <<CHANGE_DATE>> {0:1}

/// Represents a NOTE_RECORD at level 0 in a GEDCOM file
///
/// NOTE records contain shared notes that can be referenced from multiple
/// other records (individuals, families, sources, etc.)
#[derive(Clone, Debug, Default)]
pub struct NoteRecord {
    /// Cross-reference identifier for this note
    pub xref: Option<Xref>,

    /// The note text content (supports CONC/CONT for multi-line notes)
    pub note: String,

    /// Source citations for this note
    pub source_citations: Vec<SourceCitation>,

    /// User reference numbers
    pub user_reference_numbers: Vec<UserReference>,

    /// Automated record ID
    pub automated_record_id: Option<String>,

    /// Change date (basic parsing, skips structure for now)
    pub change_date: Option<String>,
}

impl NoteRecord {
    /// Parse a NOTE_RECORD starting at level 0
    pub fn parse(input: &mut &str) -> PResult<NoteRecord> {
        let mut note_record = NoteRecord::default();

        // Parse the level 0 line to get xref and initial note text
        let Ok(level_line) = Line::parse(input) else {
            return Ok(note_record);
        };

        if level_line.tag != "NOTE" {
            return Ok(note_record);
        }

        // Extract xref from level_line.xref (format: @N1@)
        if !level_line.xref.is_empty() {
            note_record.xref = Some(Xref::new(level_line.xref));
        }

        // Initial note text is in the value of the NOTE line
        note_record.note = level_line.value.to_string();

        // Parse level 1+ tags
        let Ok(mut line) = Line::peek(input) else {
            return Ok(note_record);
        };

        while !input.is_empty() && line.level > 0 {
            let mut consume = true;

            match line.tag {
                "CONC" => {
                    // Concatenate to the same line (no newline)
                    note_record.note.push_str(line.value);
                }
                "CONT" => {
                    // Continue on a new line (add newline)
                    note_record.note.push('\n');
                    note_record.note.push_str(line.value);
                }
                "SOUR" => {
                    if let Ok(citation) = SourceCitation::parse(input) {
                        note_record.source_citations.push(citation);
                    }
                    consume = false;
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

                    note_record
                        .user_reference_numbers
                        .push(UserReference { number, ref_type });
                    consume = false;
                }
                "RIN" => {
                    note_record.automated_record_id = Some(line.value.to_string());
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

        Ok(note_record)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_note_record() {
        let data = vec!["0 @N1@ NOTE This is a basic note.", "0 @N2@ NOTE"].join("\n");

        let mut input = data.as_str();
        let note = NoteRecord::parse(&mut input).unwrap();

        assert!(note.xref.is_some());
        assert_eq!(note.xref.unwrap().to_string(), "@N1@");
        assert_eq!(note.note, "This is a basic note.");
    }

    #[test]
    fn parse_note_with_conc_cont() {
        let data = vec![
            "0 @N1@ NOTE This is the first line of a note.",
            "1 CONT This is the second line of a note.",
            "1 CONC This is also on the second line.",
            "1 CONT This line should be the last line.",
        ]
        .join("\n");

        let mut input = data.as_str();
        let note = NoteRecord::parse(&mut input).unwrap();

        assert_eq!(note.xref.unwrap().to_string(), "@N1@");
        assert_eq!(
            note.note,
            "This is the first line of a note.\nThis is the second line of a note.This is also on the second line.\nThis line should be the last line."
        );
    }

    #[test]
    fn parse_note_with_source_citation() {
        let data = vec![
            "0 @N2@ NOTE Comments on Family History Library.",
            "1 SOUR @S1@",
            "2 PAGE 1",
            "2 QUAY 3",
        ]
        .join("\n");

        let mut input = data.as_str();
        let note = NoteRecord::parse(&mut input).unwrap();

        assert_eq!(note.note, "Comments on Family History Library.");
        assert_eq!(note.source_citations.len(), 1);
        assert_eq!(note.source_citations[0].xref, Some("@S1@".to_string()));
        assert_eq!(note.source_citations[0].page, Some(1));
    }

    #[test]
    fn parse_note_with_refn() {
        let data = vec![
            "0 @N1@ NOTE Test note",
            "1 REFN USER123",
            "2 TYPE custom",
            "1 RIN AUTO456",
        ]
        .join("\n");

        let mut input = data.as_str();
        let note = NoteRecord::parse(&mut input).unwrap();

        assert_eq!(note.user_reference_numbers.len(), 1);
        assert_eq!(note.user_reference_numbers[0].number, "USER123");
        assert_eq!(
            note.user_reference_numbers[0].ref_type,
            Some("custom".to_string())
        );
        assert_eq!(note.automated_record_id, Some("AUTO456".to_string()));
    }

    #[test]
    fn parse_note_with_change_date() {
        let data = vec![
            "0 @N1@ NOTE Test note",
            "1 CHAN",
            "2 DATE 24 MAY 1999",
            "3 TIME 16:39:55",
        ]
        .join("\n");

        let mut input = data.as_str();
        let note = NoteRecord::parse(&mut input).unwrap();

        assert_eq!(note.note, "Test note");
        // CHAN is parsed but not fully stored yet
    }

    #[test]
    fn parse_complete_note_record() {
        let data = vec![
            "0 @N2@ NOTE Comments on Family History Library REPOSITORY Record.",
            "1 CONT",
            "1 CONT This record uses all possible GEDCOM tags for a REPOSITORY record. Some",
            "1 CONC things to look for are:",
            "1 CONT",
            "1 CONT 1. The address is specified twice. Once in a multi-line address record and once in",
            "1 CONC separate lines. The first method is usually enough. The second method is to be more",
            "1 CONC specific about parts of the address. Is everything imported?",
            "1 CONT",
            "1 CONT 2. There are multiple phone numbers. Are they all imported?",
            "1 SOUR @S1@",
            "2 PAGE 1",
            "2 DATA",
            "3 DATE 1 MAY 1999",
            "3 TEXT Text from the source about this repository.",
            "2 QUAY 3",
            "1 CHAN",
            "2 DATE 12 MAR 2000",
            "3 TIME 11:44:05",
        ]
        .join("\n");

        let mut input = data.as_str();
        let note = NoteRecord::parse(&mut input).unwrap();

        assert!(note.xref.is_some());
        assert_eq!(note.xref.unwrap().to_string(), "@N2@");
        assert!(note
            .note
            .starts_with("Comments on Family History Library REPOSITORY Record."));
        assert!(note.note.contains("things to look for are:"));
        assert!(note.note.contains("Are they all imported?"));
        assert_eq!(note.source_citations.len(), 1);
    }
}
