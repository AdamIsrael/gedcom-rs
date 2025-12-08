use super::{Line, Note};
use winnow::prelude::*;

// CHANGE_DATE:=
//
// n CHAN                           {0:1}
//   +1 DATE <CHANGE_DATE>          {1:1}
//     +2 TIME <TIME_VALUE>         {0:1}
//   +1 NOTE <NOTE_STRUCTURE>       {0:M}

/// Represents a CHANGE_DATE structure in GEDCOM
///
/// This structure records when a record was last changed, including
/// the date, optional time, and optional notes about the change.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeDate {
    /// Date of the change (required if CHAN present)
    pub date: Option<String>,

    /// Time of the change (format: HH:MM:SS or HH:MM:SS.fraction)
    pub time: Option<String>,

    /// Notes about the change
    pub notes: Vec<Note>,
}

impl ChangeDate {
    /// Parse a CHANGE_DATE structure (CHAN tag)
    ///
    /// Expects input positioned at the CHAN line and consumes the entire structure.
    pub fn parse(input: &mut &str) -> PResult<ChangeDate> {
        let mut change_date = ChangeDate::default();

        // Parse the CHAN line
        let Ok(chan_line) = Line::peek(input) else {
            return Ok(change_date);
        };

        if chan_line.tag != "CHAN" {
            return Ok(change_date);
        }

        let chan_level = chan_line.level;
        let _ = Line::parse(input);

        // Parse child tags (DATE, TIME, NOTE)
        while let Ok(line) = Line::peek(input) {
            // Stop if we've gone back to the same level or higher
            if line.level <= chan_level {
                break;
            }

            let mut consume = true;

            match line.tag {
                "DATE" if line.level == chan_level + 1 => {
                    change_date.date = Some(line.value.to_string());

                    // After parsing DATE, check for TIME at the next level
                    let _ = Line::parse(input);
                    consume = false;

                    if let Ok(time_line) = Line::peek(input) {
                        if time_line.tag == "TIME" && time_line.level == chan_level + 2 {
                            change_date.time = Some(time_line.value.to_string());
                            let _ = Line::parse(input);
                        }
                    }
                }
                "NOTE" if line.level == chan_level + 1 => {
                    if let Ok(note) = Note::parse(input) {
                        change_date.notes.push(note);
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
        }

        Ok(change_date)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_date_with_date_only() {
        let data = vec!["1 CHAN", "2 DATE 13 JUN 2000"].join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, Some("13 JUN 2000".to_string()));
        assert_eq!(change_date.time, None);
        assert_eq!(change_date.notes.len(), 0);
    }

    #[test]
    fn test_change_date_with_date_and_time() {
        let data = vec!["1 CHAN", "2 DATE 7 SEP 2000", "3 TIME 8:35:36"].join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, Some("7 SEP 2000".to_string()));
        assert_eq!(change_date.time, Some("8:35:36".to_string()));
        assert_eq!(change_date.notes.len(), 0);
    }

    #[test]
    fn test_change_date_with_date_time_and_note() {
        let data = vec![
            "1 CHAN",
            "2 DATE 17 Feb 2003",
            "3 TIME 9:55:13",
            "2 NOTE Updated to fix typo",
        ]
        .join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, Some("17 Feb 2003".to_string()));
        assert_eq!(change_date.time, Some("9:55:13".to_string()));
        assert_eq!(change_date.notes.len(), 1);
        assert_eq!(
            change_date.notes[0].note,
            Some("Updated to fix typo".to_string())
        );
    }

    #[test]
    fn test_change_date_with_multiple_notes() {
        let data = vec![
            "1 CHAN",
            "2 DATE 11 Jan 2001",
            "3 TIME 16:00:06",
            "2 NOTE First change note",
            "2 NOTE Second change note",
        ]
        .join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, Some("11 Jan 2001".to_string()));
        assert_eq!(change_date.time, Some("16:00:06".to_string()));
        assert_eq!(change_date.notes.len(), 2);
        assert_eq!(
            change_date.notes[0].note,
            Some("First change note".to_string())
        );
        assert_eq!(
            change_date.notes[1].note,
            Some("Second change note".to_string())
        );
    }

    #[test]
    fn test_change_date_with_note_continuation() {
        let data = vec![
            "1 CHAN",
            "2 DATE 11 Jan 2001",
            "2 NOTE This is a long note",
            "3 CONT that continues on the next line",
        ]
        .join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, Some("11 Jan 2001".to_string()));
        assert_eq!(change_date.notes.len(), 1);
        assert!(change_date.notes[0]
            .note
            .as_ref()
            .unwrap()
            .contains("This is a long note"));
        assert!(change_date.notes[0]
            .note
            .as_ref()
            .unwrap()
            .contains("that continues on the next line"));
    }

    #[test]
    fn test_change_date_empty() {
        let data = vec!["1 CHAN"].join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, None);
        assert_eq!(change_date.time, None);
        assert_eq!(change_date.notes.len(), 0);
    }

    #[test]
    fn test_change_date_with_fractional_seconds() {
        let data = vec!["1 CHAN", "2 DATE 1 JAN 1998", "3 TIME 13:57:24.80"].join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, Some("1 JAN 1998".to_string()));
        assert_eq!(change_date.time, Some("13:57:24.80".to_string()));
    }

    #[test]
    fn test_change_date_at_different_levels() {
        // Test parsing at level 2 (common for nested structures)
        let data = vec!["2 CHAN", "3 DATE 12 MAR 2000", "4 TIME 10:36:02"].join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, Some("12 MAR 2000".to_string()));
        assert_eq!(change_date.time, Some("10:36:02".to_string()));
    }

    #[test]
    fn test_change_date_stops_at_next_record() {
        let data = vec!["1 CHAN", "2 DATE 13 JUN 2000", "3 TIME 17:07:32", "1 RIN 3"].join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, Some("13 JUN 2000".to_string()));
        assert_eq!(change_date.time, Some("17:07:32".to_string()));

        // Verify that RIN line is still available to parse
        let next_line = Line::peek(&mut input).unwrap();
        assert_eq!(next_line.tag, "RIN");
    }

    #[test]
    fn test_change_date_with_note_reference() {
        let data = vec!["1 CHAN", "2 DATE 7 SEP 2000", "2 NOTE @N1@"].join("\n");

        let mut input = data.as_str();
        let change_date = ChangeDate::parse(&mut input).unwrap();

        assert_eq!(change_date.date, Some("7 SEP 2000".to_string()));
        assert_eq!(change_date.notes.len(), 1);
        assert_eq!(change_date.notes[0].note, Some("@N1@".to_string()));
    }
}
