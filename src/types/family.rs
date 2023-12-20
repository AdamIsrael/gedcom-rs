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

#[derive(Debug, Eq, PartialEq, Clone)]
/// The Family structure
pub struct Family {
    pub xref: String,
}

impl Family {
    pub fn parse(_record: &str) -> Family {
        // let mut object = Object { xref: "" };

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
        Family {
            xref: "".to_string(),
        }
    }
}
