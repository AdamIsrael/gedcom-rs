use super::{Line, Note, SourceCitation, UserReference, Xref};
use crate::parse;
use winnow::prelude::*;

// MULTIMEDIA_RECORD:=
//
// n @<XREF:OBJE>@ OBJE {1:1}
//   +1 FILE <MULTIMEDIA_FILE_REFERENCE> {1:M}
//     +2 FORM <MULTIMEDIA_FORMAT> {1:1}
//       +3 TYPE <SOURCE_MEDIA_TYPE> {0:1}
//     +2 TITL <DESCRIPTIVE_TITLE> {0:1}
//   +1 REFN <USER_REFERENCE_NUMBER> {0:M}
//     +2 TYPE <USER_REFERENCE_TYPE> {0:1}
//   +1 RIN <AUTOMATED_RECORD_ID> {0:1}
//   +1 <<NOTE_STRUCTURE>> {0:M}
//   +1 <<SOURCE_CITATION>> {0:M}
//   +1 <<CHANGE_DATE>> {0:1}

/// Represents a MULTIMEDIA_RECORD at level 0 in a GEDCOM file
///
/// MULTIMEDIA records contain information about photos, videos, audio files,
/// documents, and other media files associated with individuals, families, etc.
#[derive(Clone, Debug, Default)]
pub struct MultimediaRecord {
    /// Cross-reference identifier for this multimedia object
    pub xref: Option<Xref>,

    /// List of multimedia files (can have multiple files)
    pub files: Vec<MultimediaFile>,

    /// User reference numbers
    pub user_reference_numbers: Vec<UserReference>,

    /// Automated record ID
    pub automated_record_id: Option<String>,

    /// Notes about this multimedia object
    pub notes: Vec<Note>,

    /// Source citations for this multimedia object
    pub source_citations: Vec<SourceCitation>,

    /// Change date (basic parsing, skips structure for now)
    pub change_date: Option<String>,
}

/// Represents a multimedia file within a MULTIMEDIA_RECORD
#[derive(Clone, Debug, Default)]
pub struct MultimediaFile {
    /// File reference/path
    pub file_reference: String,

    /// File format (e.g., JPEG, PNG, MP4, etc.)
    pub format: Option<String>,

    /// Source media type (e.g., photo, video, audio, document)
    pub media_type: Option<String>,

    /// Descriptive title for the file
    pub title: Option<String>,
}

impl MultimediaRecord {
    /// Parse a MULTIMEDIA_RECORD starting at level 0
    pub fn parse(input: &mut &str) -> PResult<MultimediaRecord> {
        let mut multimedia = MultimediaRecord::default();

        // Parse the level 0 line to get xref
        let Ok(level_line) = Line::parse(input) else {
            return Ok(multimedia);
        };

        if level_line.tag != "OBJE" {
            return Ok(multimedia);
        }

        // Extract xref from level_line.xref (format: @M1@)
        if !level_line.xref.is_empty() {
            multimedia.xref = Some(Xref::new(level_line.xref));
        }

        // Parse level 1+ tags
        let Ok(mut line) = Line::peek(input) else {
            return Ok(multimedia);
        };

        while !input.is_empty() && line.level > 0 {
            let mut consume = true;

            match line.tag {
                "FILE" => {
                    if let Ok(file) = MultimediaFile::parse(input) {
                        multimedia.files.push(file);
                    }
                    consume = false;
                }
                "NOTE" => {
                    if let Ok(Some(text)) = parse::get_tag_value(input) {
                        multimedia.notes.push(Note { note: Some(text) });
                    }
                    consume = false;
                }
                "SOUR" => {
                    if let Ok(citation) = SourceCitation::parse(input) {
                        multimedia.source_citations.push(citation);
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

                    multimedia
                        .user_reference_numbers
                        .push(UserReference { number, ref_type });
                    consume = false;
                }
                "RIN" => {
                    multimedia.automated_record_id = Some(line.value.to_string());
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

        Ok(multimedia)
    }
}

impl MultimediaFile {
    /// Parse a FILE tag and its children
    fn parse(input: &mut &str) -> PResult<MultimediaFile> {
        let mut file = MultimediaFile::default();

        let Ok(level_line) = Line::parse(input) else {
            return Ok(file);
        };

        if level_line.tag != "FILE" {
            return Ok(file);
        }

        // File reference is the value of the FILE tag
        file.file_reference = level_line.value.to_string();
        let file_level = level_line.level;

        let Ok(mut line) = Line::peek(input) else {
            return Ok(file);
        };

        while !input.is_empty() && line.level > file_level {
            let mut consume = true;

            match line.tag {
                "FORM" => {
                    file.format = Some(line.value.to_string());
                    let _ = Line::parse(input);

                    // Check for TYPE at next level
                    if let Ok(next_line) = Line::peek(input) {
                        if next_line.tag == "TYPE" && next_line.level > line.level {
                            let _ = Line::parse(input);
                            file.media_type = Some(next_line.value.to_string());
                        }
                    }
                    consume = false;
                }
                "TITL" => {
                    file.title = Some(line.value.to_string());
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

            if line.level <= file_level {
                break;
            }
        }

        Ok(file)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_multimedia_record() {
        let data = vec![
            "0 @M1@ OBJE",
            "1 FILE photo.jpeg",
            "2 FORM JPEG",
            "0 @M2@ OBJE",
        ]
        .join("\n");

        let mut input = data.as_str();
        let multimedia = MultimediaRecord::parse(&mut input).unwrap();

        assert!(multimedia.xref.is_some());
        assert_eq!(multimedia.xref.unwrap().to_string(), "@M1@");
        assert_eq!(multimedia.files.len(), 1);
        assert_eq!(multimedia.files[0].file_reference, "photo.jpeg");
        assert_eq!(multimedia.files[0].format, Some("JPEG".to_string()));
    }

    #[test]
    fn parse_multimedia_with_type() {
        let data = vec![
            "0 @M1@ OBJE",
            "1 FILE photo.jpeg",
            "2 FORM JPEG",
            "3 TYPE photo",
            "2 TITL Picture of the book cover",
        ]
        .join("\n");

        let mut input = data.as_str();
        let multimedia = MultimediaRecord::parse(&mut input).unwrap();

        assert_eq!(multimedia.files.len(), 1);
        let file = &multimedia.files[0];
        assert_eq!(file.file_reference, "photo.jpeg");
        assert_eq!(file.format, Some("JPEG".to_string()));
        assert_eq!(file.media_type, Some("photo".to_string()));
        assert_eq!(file.title, Some("Picture of the book cover".to_string()));
    }

    #[test]
    fn parse_multimedia_with_notes() {
        let data = vec![
            "0 @M1@ OBJE",
            "1 FILE image.jpg",
            "2 FORM JPEG",
            "1 NOTE Here are some notes on this multimedia object.",
            "2 CONT If decoded it should be an image of a flower.",
        ]
        .join("\n");

        let mut input = data.as_str();
        let multimedia = MultimediaRecord::parse(&mut input).unwrap();

        assert_eq!(multimedia.notes.len(), 1);
        assert!(multimedia.notes[0]
            .note
            .as_ref()
            .unwrap()
            .contains("multimedia object"));
        assert!(multimedia.notes[0]
            .note
            .as_ref()
            .unwrap()
            .contains("image of a flower"));
    }

    #[test]
    fn parse_multimedia_with_refn() {
        let data = vec![
            "0 @M1@ OBJE",
            "1 FILE photo.jpeg",
            "2 FORM JPEG",
            "1 REFN 01234567890123456789",
            "2 TYPE reference",
            "1 RIN 1",
        ]
        .join("\n");

        let mut input = data.as_str();
        let multimedia = MultimediaRecord::parse(&mut input).unwrap();

        assert_eq!(multimedia.user_reference_numbers.len(), 1);
        assert_eq!(
            multimedia.user_reference_numbers[0].number,
            "01234567890123456789"
        );
        assert_eq!(
            multimedia.user_reference_numbers[0].ref_type,
            Some("reference".to_string())
        );
        assert_eq!(multimedia.automated_record_id, Some("1".to_string()));
    }

    #[test]
    fn parse_multimedia_with_change_date() {
        let data = vec![
            "0 @M1@ OBJE",
            "1 FILE photo.jpeg",
            "2 FORM JPEG",
            "1 CHAN",
            "2 DATE 14 JAN 2001",
            "3 TIME 14:10:31",
        ]
        .join("\n");

        let mut input = data.as_str();
        let multimedia = MultimediaRecord::parse(&mut input).unwrap();

        assert_eq!(multimedia.files.len(), 1);
        // CHAN is parsed but not fully stored yet
    }

    #[test]
    fn parse_complete_multimedia_record() {
        let data = vec![
            "0 @M1@ OBJE",
            "1 FILE photo.jpeg",
            "2 FORM JPEG",
            "3 TYPE photo",
            "2 TITL Picture of the book cover",
            "1 REFN 01234567890123456789",
            "2 TYPE reference",
            "1 RIN 1",
            "1 NOTE Here are some notes on this multimedia object.",
            "2 CONT If decoded it should be an image of a flower.",
            "1 NOTE @N1@",
            "1 CHAN",
            "2 DATE 14 JAN 2001",
            "3 TIME 14:10:31",
        ]
        .join("\n");

        let mut input = data.as_str();
        let multimedia = MultimediaRecord::parse(&mut input).unwrap();

        assert!(multimedia.xref.is_some());
        assert_eq!(multimedia.xref.unwrap().to_string(), "@M1@");
        assert_eq!(multimedia.files.len(), 1);
        assert_eq!(multimedia.files[0].file_reference, "photo.jpeg");
        assert_eq!(multimedia.files[0].format, Some("JPEG".to_string()));
        assert_eq!(multimedia.files[0].media_type, Some("photo".to_string()));
        assert_eq!(
            multimedia.files[0].title,
            Some("Picture of the book cover".to_string())
        );
        assert_eq!(multimedia.notes.len(), 2);
        assert_eq!(multimedia.user_reference_numbers.len(), 1);
        assert!(multimedia.automated_record_id.is_some());
    }

    #[test]
    fn parse_multimedia_with_multiple_files() {
        let data = vec![
            "0 @M1@ OBJE",
            "1 FILE photo.jpeg",
            "2 FORM JPEG",
            "1 FILE photo_thumb.jpeg",
            "2 FORM JPEG",
            "2 TITL Thumbnail version",
        ]
        .join("\n");

        let mut input = data.as_str();
        let multimedia = MultimediaRecord::parse(&mut input).unwrap();

        assert_eq!(multimedia.files.len(), 2);
        assert_eq!(multimedia.files[0].file_reference, "photo.jpeg");
        assert_eq!(multimedia.files[1].file_reference, "photo_thumb.jpeg");
        assert_eq!(
            multimedia.files[1].title,
            Some("Thumbnail version".to_string())
        );
    }
}
