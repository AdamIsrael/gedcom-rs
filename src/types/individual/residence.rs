use crate::types::{FamilyEventDetail, Line};
use winnow::prelude::*;

// n RESI
// +1 <<FAMILY_EVENT_DETAIL>>

#[derive(Debug, Default)]
pub struct Residence {
    pub detail: Option<FamilyEventDetail>,
}

impl Residence {
    // Parse a RESI record
    pub fn parse(record: &mut &str) -> PResult<Residence> {
        let mut residence = Residence {
            detail: None,
        };

        let line = Line::peek(record).unwrap();

        // Check if we've received a top-level event tag, which we want to skip over.
        if line.tag == "RESI" {
            // Consume the current line
            let _ = Line::parse(record);
        }

        // 1 RESI
        // 2 ADDR 73 North Ashley
        // 3 CONT Spencer, Utah UT84991
        // 2 DATE from 1900 to 1905

        let detail = FamilyEventDetail::parse(record).unwrap();

        residence.detail = Some(detail);

        Ok(residence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_residence() {
        let data = vec![
            "1 RESI",
            "2 ADDR 73 North Ashley",
            "3 CONT Spencer, Utah UT 84991",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE RESI",
        ];

        let input = data.join("\n");
        let mut record = input.as_str();

        let residence = Residence::parse(&mut record).unwrap();

        assert!(residence.detail.is_some());
        let fdetail = residence.detail.unwrap();

        assert!(fdetail.husband.is_none());
        assert!(fdetail.wife.is_none());

        assert!(fdetail.detail.is_some());
        let detail = fdetail.detail.unwrap();

        assert!("31 DEC 1997" == detail.date.unwrap());

        assert!(detail.address.is_some());
        let addr = detail.address.unwrap();

        assert!(addr.addr1.unwrap().starts_with("73 North Ashley"));

        assert!("RESI" == detail.r#type.unwrap());

        let place = detail.place.unwrap();
        assert_eq!(place.name.unwrap(), "The place");
    }
}
