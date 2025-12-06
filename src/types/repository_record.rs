use super::{Address, Line, Note, UserReference, Xref};
use crate::parse;
use winnow::prelude::*;

// REPOSITORY_RECORD:=
//
// n @<XREF:REPO>@ REPO {1:1}
//   +1 NAME <NAME_OF_REPOSITORY> {0:1}
//   +1 <<ADDRESS_STRUCTURE>> {0:1}
//   +1 <<NOTE_STRUCTURE>> {0:M}
//   +1 REFN <USER_REFERENCE_NUMBER> {0:M}
//     +2 TYPE <USER_REFERENCE_TYPE> {0:1}
//   +1 RIN <AUTOMATED_RECORD_ID> {0:1}
//   +1 <<CHANGE_DATE>> {0:1}

/// Represents a REPOSITORY_RECORD at level 0 in a GEDCOM file
///
/// REPOSITORY records contain information about archives, libraries, and other
/// institutions that hold genealogical records and source materials.
#[derive(Clone, Debug, Default)]
pub struct RepositoryRecord {
    /// Cross-reference identifier for this repository
    pub xref: Option<Xref>,

    /// Name of the repository (e.g., "Family History Library")
    pub name: Option<String>,

    /// Address of the repository
    pub address: Option<Address>,

    /// Notes about this repository
    pub notes: Vec<Note>,

    /// User reference numbers
    pub user_reference_numbers: Vec<UserReference>,

    /// Automated record ID
    pub automated_record_id: Option<String>,

    /// Change date - stores the DATE value from CHAN/DATE (skips TIME and other structure)
    pub change_date: Option<String>,
}

impl RepositoryRecord {
    /// Parse a REPOSITORY_RECORD starting at level 0
    pub fn parse(input: &mut &str) -> PResult<RepositoryRecord> {
        let mut repository = RepositoryRecord::default();

        // Parse the level 0 line to get xref
        let Ok(level_line) = Line::parse(input) else {
            return Ok(repository);
        };

        if level_line.tag != "REPO" {
            return Ok(repository);
        }

        // Extract xref from level_line.xref (format: @R1@)
        if !level_line.xref.is_empty() {
            repository.xref = Some(Xref::new(level_line.xref));
        }

        // Parse level 1+ tags
        let Ok(mut line) = Line::peek(input) else {
            return Ok(repository);
        };

        while !input.is_empty() && line.level > 0 {
            let mut consume = true;

            match line.tag {
                "NAME" => {
                    repository.name = Some(line.value.to_string());
                }
                "ADDR" => {
                    if let Ok(address) = Address::parse(input) {
                        repository.address = Some(address);
                    }
                    consume = false;
                }
                "NOTE" => {
                    // NOTE can be either inline text or a reference (@N1@)
                    if line.value.starts_with('@') && line.value.ends_with('@') {
                        // It's a note reference - store as-is
                        let _ = Line::parse(input);
                        repository.notes.push(Note {
                            note: Some(line.value.to_string()),
                        });
                    } else {
                        // It's inline text - use get_tag_value to handle CONC/CONT
                        if let Ok(Some(text)) = parse::get_tag_value(input) {
                            repository.notes.push(Note { note: Some(text) });
                        }
                    }
                    consume = false;
                }
                "REFN" => {
                    let number = line.value.to_string();
                    let refn_level = line.level;
                    let _ = Line::parse(input);

                    // Check for TYPE at next level
                    let mut ref_type = None;
                    if let Ok(next_line) = Line::peek(input) {
                        if next_line.tag == "TYPE" && next_line.level > refn_level {
                            let _ = Line::parse(input);
                            ref_type = Some(next_line.value.to_string());
                        }
                    }

                    repository
                        .user_reference_numbers
                        .push(UserReference { number, ref_type });
                    consume = false;
                }
                "RIN" => {
                    repository.automated_record_id = Some(line.value.to_string());
                }
                "CHAN" => {
                    // Basic parsing - extract DATE value and skip rest of structure
                    let chan_level = line.level;
                    let _ = Line::parse(input);

                    // Look for DATE tag at next level
                    while let Ok(peek) = Line::peek(input) {
                        if peek.level <= chan_level {
                            break;
                        }
                        if peek.tag == "DATE" && peek.level == chan_level + 1 {
                            repository.change_date = Some(peek.value.to_string());
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

        Ok(repository)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_repository_record() {
        let data = vec!["0 @R1@ REPO", "1 NAME Family History Library"].join("\n");

        let mut input = data.as_str();
        let repository = RepositoryRecord::parse(&mut input).unwrap();

        assert!(repository.xref.is_some());
        assert_eq!(repository.xref.unwrap().to_string(), "@R1@");
        assert_eq!(repository.name, Some("Family History Library".to_string()));
    }

    #[test]
    fn parse_repository_with_address() {
        let data = vec![
            "0 @R1@ REPO",
            "1 NAME Family History Library",
            "1 ADDR",
            "2 ADR1 35 North West Temple",
            "2 CITY Salt Lake City",
            "2 STAE UT",
            "2 POST 84111",
            "2 CTRY USA",
        ]
        .join("\n");

        let mut input = data.as_str();
        let repository = RepositoryRecord::parse(&mut input).unwrap();

        assert!(repository.address.is_some());
        let addr = repository.address.unwrap();
        assert_eq!(addr.addr1, Some("35 North West Temple".to_string()));
        assert_eq!(addr.city, Some("Salt Lake City".to_string()));
        assert_eq!(addr.state, Some("UT".to_string()));
        assert_eq!(addr.postal_code, Some("84111".to_string()));
        assert_eq!(addr.country, Some("USA".to_string()));
    }

    #[test]
    fn parse_repository_with_phones() {
        let data = vec![
            "0 @R1@ REPO",
            "1 NAME Family History Library",
            "1 ADDR",
            "2 CITY Salt Lake City",
            "1 PHON +1-801-240-2331",
            "1 PHON +1-801-240-1278",
            "1 PHON +1-801-240-2584",
        ]
        .join("\n");

        let mut input = data.as_str();
        let repository = RepositoryRecord::parse(&mut input).unwrap();

        assert!(repository.address.is_some());
        let addr = repository.address.unwrap();
        assert_eq!(addr.phone.len(), 3);
        assert_eq!(addr.phone[0], "+1-801-240-2331");
        assert_eq!(addr.phone[1], "+1-801-240-1278");
        assert_eq!(addr.phone[2], "+1-801-240-2584");
    }

    #[test]
    fn parse_repository_with_note_reference() {
        let data = vec![
            "0 @R1@ REPO",
            "1 NAME Family History Library",
            "1 NOTE @N2@",
        ]
        .join("\n");

        let mut input = data.as_str();
        let repository = RepositoryRecord::parse(&mut input).unwrap();

        assert_eq!(repository.notes.len(), 1);
        assert_eq!(repository.notes[0].note, Some("@N2@".to_string()));
    }

    #[test]
    fn parse_repository_with_refn() {
        let data = vec![
            "0 @R1@ REPO",
            "1 NAME Family History Library",
            "1 REFN 01234567890123456789",
            "2 TYPE reference",
        ]
        .join("\n");

        let mut input = data.as_str();
        let repository = RepositoryRecord::parse(&mut input).unwrap();

        assert_eq!(repository.user_reference_numbers.len(), 1);
        assert_eq!(
            repository.user_reference_numbers[0].number,
            "01234567890123456789"
        );
        assert_eq!(
            repository.user_reference_numbers[0].ref_type,
            Some("reference".to_string())
        );
    }

    #[test]
    fn parse_repository_with_change_date() {
        let data = vec![
            "0 @R1@ REPO",
            "1 NAME Family History Library",
            "1 CHAN",
            "2 DATE 12 MAR 2000",
            "3 TIME 10:36:02",
        ]
        .join("\n");

        let mut input = data.as_str();
        let repository = RepositoryRecord::parse(&mut input).unwrap();

        assert_eq!(repository.change_date, Some("12 MAR 2000".to_string()));
    }

    #[test]
    fn parse_complete_repository_record() {
        let data = vec![
            "0 @R1@ REPO",
            "1 NAME Family History Library",
            "1 ADDR",
            "2 ADR1 35 North West Temple",
            "2 ADR2 Across the street from Temple Square",
            "2 ADR3 Ste 1",
            "2 CITY Salt Lake City",
            "2 STAE UT",
            "2 POST 84111",
            "2 CTRY USA",
            "1 PHON +1-801-240-2331",
            "1 PHON +1-801-240-1278",
            "1 PHON +1-801-240-2584",
            "1 NOTE @N2@",
            "1 REFN 01234567890123456789",
            "2 TYPE reference",
            "1 RIN 1",
            "1 CHAN",
            "2 DATE 12 MAR 2000",
            "3 TIME 10:36:02",
        ]
        .join("\n");

        let mut input = data.as_str();
        let repository = RepositoryRecord::parse(&mut input).unwrap();

        assert_eq!(repository.xref.unwrap().to_string(), "@R1@");
        assert_eq!(repository.name, Some("Family History Library".to_string()));
        assert!(repository.address.is_some());
        let addr = repository.address.unwrap();
        assert_eq!(addr.phone.len(), 3);
        assert_eq!(repository.notes.len(), 1);
        assert_eq!(repository.user_reference_numbers.len(), 1);
        assert_eq!(repository.automated_record_id, Some("1".to_string()));
        assert_eq!(repository.change_date, Some("12 MAR 2000".to_string()));
    }
}
