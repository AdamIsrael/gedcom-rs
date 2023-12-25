use std::str::FromStr;

use crate::{
    parse,
    types::{Line, Note, Pedigree},
};

// use super::pedigree;

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

#[derive(Debug, Clone)]
/// The Family structure
pub struct Family {
    pub xref: String,
    pub notes: Vec<Note>,
    pub pedigree: Option<Pedigree>,
}

impl Family {
    pub fn parse(record: &mut &str) -> Family {
        let mut family = Family {
            xref: "".to_string(),
            notes: vec![],
            pedigree: None,
        };

        let line = Line::peek(record).unwrap();
        let level = line.level;
        let tag = line.tag;

        // If we're at the top of the record, consume the line
        if tag == "FAMC" || tag == "FAMS" {
            // Capture the xref
            family.xref = line.value.to_string();
            Line::parse(record).unwrap();
        }

        while !record.is_empty() {
            let mut consume = true;
            let line = Line::peek(record).unwrap();

            // If the next level matches our initial level, we're done parsing
            // this structure.
            if line.level == level {
                break;
            }

            match line.tag {
                "NOTE" => {
                    if let Some(note) = parse::get_tag_value(record).unwrap() {
                        family.notes.push(Note { note: Some(note) });
                    }
                    consume = false;
                }
                "PEDI" => {
                    let pedigree = Pedigree::from_str(line.value).unwrap();
                    family.pedigree = Some(pedigree);
                }
                _ => {}
            }

            if consume {
                Line::parse(record).unwrap();
            }
        }

        family
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests a possible bug in Ancestry's format, if a line break is embedded within the content of a note
    /// As far as I can tell, it's a \n embedded into the note, at least, from a hex dump of that content.
    fn parse_family() {
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
}
