use crate::types::{Line, Note};

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
    pub note: Option<Note>,
}

impl Family {
    pub fn parse(record: &mut &str) -> Family {
        let mut family = Family {
            xref: "".to_string(),
            note: None,
        };

        let line = Line::peek(record).unwrap();
        // let level = line.level;
        let tag = line.tag;

        // If we're at the top of the record, consume the line
        if tag == "FAMC" || tag == "FAMS" {
            // Capture the xref
            family.xref = line.value.to_string();
            Line::parse(record).unwrap();
        }

        // "1 FAMS @F1@",
        // "2 NOTE Note about the link to the family record with his first spouse.",
        // "2 NOTE Another note about the link to the family record with his first spouse.",

        // while !record.is_empty() {
        // let mut line = Line::parse(record).unwrap();
        // match line.tag {
        //     "NAME" => {
        //         // fam.name = Some(line.value.to_string());
        //     }
        //     _ => {}
        // }

        // If the next level matches our initial level, we're done parsing
        // this structure.
        //     line = Line::peek(record).unwrap();
        //     if line.level == level {
        //         break;
        //     }
        // }

        // while !record.is_empty() {
        //     let (buffer, line) = Line::parse(&record).unwrap();

        //     // If we're at the top of the record, get the xref
        //     // && level == 0
        //     match line.level {
        //         0 => {
        //             object.xref = line.xref;
        //         }
        //         _ => {
        //         }
        //     }
        // }
        // object
        // Family {
        //     xref: "".to_string(),
        // }
        family
    }
}
