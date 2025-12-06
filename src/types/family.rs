use std::str::FromStr;

use crate::{
    parse,
    types::{AdoptedBy, FamilyEventDetail, Line, Note, Pedigree, Xref},
};

// TODO: implement full parsing of the family record
// TODO: Need to create a trait? to find_by_xref that can be used in these
// types of structs, to find the type of object in a vec of the types.

// FAM_RECORD:=
// n @<XREF:FAM>@ FAM {1:1}
// +1 RESN <RESTRICTION_NOTICE> {0:1) p.60
// +1 <<FAMILY_EVENT_STRUCTURE>> {0:M} p.32
// +1 HUSB @<XREF:INDI>@ {0:1} p.25
// +1 WIFE @<XREF:INDI>@ {0:1} p.25
// +1 CHIL @<XREF:INDI>@ {0:M} p.25
// +1 NCHI <COUNT_OF_CHILDREN> {0:1} p.44
// +1 SUBM @<XREF:SUBM>@ {0:M} p.28
// +1 <<LDS_SPOUSE_SEALING>> {0:M} p.36
// +1 REFN <USER_REFERENCE_NUMBER> {0:M} p.63, 64
// 25
// +2 TYPE <USER_REFERENCE_TYPE> {0:1} p.64
// +1 RIN <AUTOMATED_RECORD_ID> {0:1} p.43
// +1 <<CHANGE_DATE>> {0:1} p.31
// +1 <<NOTE_STRUCTURE>> {0:M} p.37
// +1 <<SOURCE_CITATION>> {0:M} p.39
// +1 <<MULTIMEDIA_LINK>> {0:M} p.37, 26

#[derive(Debug, Clone, Default)]
/// The Family structure representing a FAM record
///
/// Represents a family unit with husband, wife, children, and associated events.
pub struct Family {
    /// Cross-reference identifier for this family (e.g., @F1@)
    pub xref: Xref,

    /// Reference to husband individual
    pub husband: Option<Xref>,

    /// Reference to wife individual
    pub wife: Option<Xref>,

    /// References to children individuals
    pub children: Vec<Xref>,

    /// Count of children (NCHI tag)
    pub child_count: Option<u32>,

    /// Family events (marriage, engagement, divorce, etc.)
    pub events: Vec<FamilyEventDetail>,

    /// Notes about this family
    pub notes: Vec<Note>,

    /// Automated record ID
    pub automated_record_id: Option<String>,

    /// User reference number
    pub user_reference_number: Option<String>,

    /// User reference type
    pub user_reference_type: Option<String>,

    // Legacy fields used for FAMC/FAMS child-to-family links
    // These are kept for backward compatibility with Individual parsing
    pub(crate) adopted_by: Option<AdoptedBy>,
    pub(crate) pedigree: Option<Pedigree>,
}

impl Family {
    /// Create a minimal Family with just an xref (for FAMC/FAMS references)
    #[inline]
    pub(crate) fn with_xref(xref: Xref) -> Self {
        Family {
            xref,
            husband: None,
            wife: None,
            children: Vec::new(),
            child_count: None,
            events: Vec::new(),
            notes: Vec::new(),
            automated_record_id: None,
            user_reference_number: None,
            user_reference_type: None,
            adopted_by: None,
            pedigree: None,
        }
    }

    /// Parse a FAM record or FAMC/FAMS family reference
    ///
    /// This method handles two different parsing contexts:
    /// 1. Top-level FAM records (level 0): `0 @F1@ FAM`
    /// 2. Child-to-family links (level 1+): `1 FAMC @F1@` or `1 FAMS @F1@`
    pub fn parse(record: &mut &str) -> Family {
        let mut family = Family::default();

        let Ok(line) = Line::peek(record) else {
            return family;
        };
        let level = line.level;
        let tag = line.tag;

        // Handle top-level FAM record: 0 @XREF@ FAM
        if tag == "FAM" && level == 0 {
            if !line.xref.is_empty() {
                family.xref = Xref::new(line.xref.to_string());
            }
            let _ = Line::parse(record);

            // Parse FAM record fields
            while !record.is_empty() {
                let Ok(line) = Line::peek(record) else {
                    break;
                };

                // If we hit another level-0 record, stop
                if line.level == 0 {
                    break;
                }

                // Only process direct children (level 1)
                if line.level != 1 {
                    let _ = Line::parse(record);
                    continue;
                }

                match line.tag {
                    "HUSB" => {
                        family.husband = Some(Xref::new(line.value.to_string()));
                        let _ = Line::parse(record);
                    }
                    "WIFE" => {
                        family.wife = Some(Xref::new(line.value.to_string()));
                        let _ = Line::parse(record);
                    }
                    "CHIL" => {
                        family.children.push(Xref::new(line.value.to_string()));
                        let _ = Line::parse(record);
                    }
                    "NCHI" => {
                        if let Ok(count) = line.value.parse::<u32>() {
                            family.child_count = Some(count);
                        }
                        let _ = Line::parse(record);
                    }
                    "MARR" | "ENGA" | "DIV" | "ANUL" | "CENS" | "DIVF" | "EVEN" => {
                        // Consume the event tag line
                        let _ = Line::parse(record);

                        // Collect all subfields (level 2+) for this event
                        // Pre-allocate with typical event size (~5-10 lines)
                        let mut event_lines: Vec<String> = Vec::with_capacity(8);
                        while !record.is_empty() {
                            let Ok(line) = Line::peek(record) else {
                                break;
                            };
                            // Stop if we're back to level 1 or level 0
                            if line.level <= 1 {
                                break;
                            }
                            event_lines.push(line.to_string());
                            let _ = Line::parse(record);
                        }

                        // Only parse if we actually have event data
                        if !event_lines.is_empty() {
                            let event_str = event_lines.join("\n");
                            let mut event_record = event_str.as_str();
                            if let Ok(event) = FamilyEventDetail::parse(&mut event_record) {
                                family.events.push(event);
                            }
                        }
                    }
                    "NOTE" => {
                        if let Ok(Some(note)) = parse::get_tag_value(record) {
                            family.notes.push(Note { note: Some(note) });
                        }
                    }
                    "RIN" => {
                        family.automated_record_id = Some(line.value.to_string());
                        let _ = Line::parse(record);
                    }
                    "REFN" => {
                        family.user_reference_number = Some(line.value.to_string());
                        let _ = Line::parse(record);
                        // Check for TYPE subfield
                        if let Ok(subline) = Line::peek(record) {
                            if subline.level == 2 && subline.tag == "TYPE" {
                                family.user_reference_type = Some(subline.value.to_string());
                                let _ = Line::parse(record);
                            }
                        }
                    }
                    _ => {
                        let _ = Line::parse(record);
                    }
                }
            }
        }
        // Handle FAMC/FAMS family references (child-to-family or spouse-to-family links)
        else if tag == "FAMC" || tag == "FAMS" {
            // Capture the xref
            family.xref = Xref::new(line.value.to_string());
            let _ = Line::parse(record);

            // Parse FAMC/FAMS subfields (PEDI, ADOP, NOTE)
            while !record.is_empty() {
                let mut consume = true;
                let Ok(line) = Line::peek(record) else {
                    break;
                };

                // If the next level matches or is less than our initial level, stop
                if line.level <= level {
                    break;
                }

                match line.tag {
                    "NOTE" => {
                        if let Ok(Some(note)) = parse::get_tag_value(record) {
                            family.notes.push(Note { note: Some(note) });
                        }
                        consume = false;
                    }
                    "PEDI" => {
                        if let Ok(pedigree) = Pedigree::from_str(line.value) {
                            family.pedigree = Some(pedigree);
                        }
                    }
                    "ADOP" => {
                        if let Ok(adopted_by) = AdoptedBy::from_str(line.value) {
                            family.adopted_by = Some(adopted_by);
                        }
                    }
                    _ => {}
                }

                if consume {
                    let _ = Line::parse(record);
                }
            }
        }

        family
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests a possible bug in Ancestry's format, if a line break is embedded within the content of a note
    /// As far as I can tell, it's a \n embedded into the note, at least, from a hex dump of that content.
    fn parse_family_reference() {
        let data = vec![
            "1 FAMS @F4@",
            "1 FAMC @F2@",
            "2 NOTE Note about this link to his parents family record.",
            "2 NOTE Another note about this link to his parents family record",
            "1 FAMC @F3@",
            "2 PEDI adopted",
            "2 NOTE Note about the link to his adoptive parents family record.",
        ]
        .join("\n");
        let mut record = data.as_str();

        // First family
        let family = Family::parse(&mut record);
        assert!(family.xref == "@F4@");

        // Second family
        let family = Family::parse(&mut record);
        assert!(family.xref == "@F2@");

        let notes = family.notes;
        assert!(
            notes[0].note.as_ref().unwrap() == "Note about this link to his parents family record."
        );
        assert!(
            notes[1].note.as_ref().unwrap()
                == "Another note about this link to his parents family record"
        );

        // Third family
        let family = Family::parse(&mut record);
        assert!(family.xref == "@F3@");
        assert!(family.pedigree.unwrap() == Pedigree::Adopted);

        let notes = family.notes;
        assert!(
            notes[0].note.as_ref().unwrap()
                == "Note about the link to his adoptive parents family record."
        );
    }

    #[test]
    fn parse_fam_record_basic() {
        let data = vec![
            "0 @F1@ FAM",
            "1 HUSB @I1@",
            "1 WIFE @I2@",
            "1 CHIL @I3@",
            "1 CHIL @I4@",
            "1 NCHI 2",
        ]
        .join("\n");
        let mut record = data.as_str();

        let family = Family::parse(&mut record);
        assert_eq!(family.xref, Xref::new("@F1@".to_string()));
        assert_eq!(family.husband, Some(Xref::new("@I1@".to_string())));
        assert_eq!(family.wife, Some(Xref::new("@I2@".to_string())));
        assert_eq!(family.children.len(), 2);
        assert_eq!(family.children[0], Xref::new("@I3@".to_string()));
        assert_eq!(family.children[1], Xref::new("@I4@".to_string()));
        assert_eq!(family.child_count, Some(2));
    }

    #[test]
    fn parse_fam_record_with_marriage() {
        let data = vec![
            "0 @FAMILY1@ FAM",
            "1 HUSB @PERSON1@",
            "1 WIFE @PERSON2@",
            "1 MARR",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "1 CHIL @PERSON3@",
            "1 NCHI 1",
        ]
        .join("\n");
        let mut record = data.as_str();

        let family = Family::parse(&mut record);
        assert_eq!(family.xref, Xref::new("@FAMILY1@".to_string()));
        assert_eq!(family.husband, Some(Xref::new("@PERSON1@".to_string())));
        assert_eq!(family.wife, Some(Xref::new("@PERSON2@".to_string())));
        assert_eq!(family.children.len(), 1);
        assert_eq!(family.children[0], Xref::new("@PERSON3@".to_string()));
        assert_eq!(family.child_count, Some(1));
        assert_eq!(family.events.len(), 1);
    }

    #[test]
    fn parse_fam_record_with_notes() {
        let data = vec![
            "0 @F2@ FAM",
            "1 HUSB @I5@",
            "1 WIFE @I6@",
            "1 NOTE This is a note about the family.",
            "1 NOTE Another note about the family.",
        ]
        .join("\n");
        let mut record = data.as_str();

        let family = Family::parse(&mut record);
        assert_eq!(family.xref, Xref::new("@F2@".to_string()));
        assert_eq!(family.notes.len(), 2);
        assert_eq!(
            family.notes[0].note.as_ref().unwrap(),
            "This is a note about the family."
        );
        assert_eq!(
            family.notes[1].note.as_ref().unwrap(),
            "Another note about the family."
        );
    }

    #[test]
    fn parse_fam_record_with_admin_fields() {
        let data = vec![
            "0 @F3@ FAM",
            "1 HUSB @I7@",
            "1 RIN 12345",
            "1 REFN USER-REF-001",
            "2 TYPE genealogy",
        ]
        .join("\n");
        let mut record = data.as_str();

        let family = Family::parse(&mut record);
        assert_eq!(family.xref, Xref::new("@F3@".to_string()));
        assert_eq!(family.automated_record_id, Some("12345".to_string()));
        assert_eq!(
            family.user_reference_number,
            Some("USER-REF-001".to_string())
        );
        assert_eq!(family.user_reference_type, Some("genealogy".to_string()));
    }

    #[test]
    fn parse_fam_record_minimal() {
        let data = "0 @F10@ FAM";
        let mut record = data;

        let family = Family::parse(&mut record);
        assert_eq!(family.xref, Xref::new("@F10@".to_string()));
        assert_eq!(family.husband, None);
        assert_eq!(family.wife, None);
        assert_eq!(family.children.len(), 0);
        assert_eq!(family.child_count, None);
        assert_eq!(family.events.len(), 0);
        assert_eq!(family.notes.len(), 0);
    }
}
