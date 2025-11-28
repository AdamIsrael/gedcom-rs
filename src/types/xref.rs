// An xref is a cross-reference to another record in the GEDCOM file.
// TODO: go through the types that use a String for xref and upgrade them.
use crate::types::Line;
use winnow::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Xref {
    // The cross-reference to the individual in the GEDCOM
    pub xref: Option<String>,
}

impl Xref {
    /// Parse an xref from the next line in the buffer
    pub fn parse(record: &mut &str) -> PResult<Xref> {
        // We find the xref in two places, potentially. The first is in the tag,
        // the second is in the value. Line::parse will set xref if it's in the tag.
        // If there's no xref, we'll check the value to see if it contains one.
        let record = Line::parse(record)?;

        if !record.xref.is_empty() {
            Ok(Xref {
                xref: Some(record.xref.to_string()),
            })
        } else if record.value.starts_with('@') && record.value.ends_with('@') {
            // HACK: Might be better to do this in Line::parse.
            Ok(Xref {
                xref: Some(record.value.to_string()),
            })
        } else {
            Ok(Xref::default())
        }
    }

    // TODO: implement a function that will lookup an individual by their xref
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_xref_from_tag() {
        let data = vec!["0 @I1@ INDI"];

        let input = data.join("\n");
        let mut record = input.as_str();

        let xref = Xref::parse(&mut record).unwrap();
        assert!("@I1@" == xref.xref.unwrap());
    }

    #[test]
    fn parse_xref_from_value() {
        let data = vec!["1 HUSB @I1@"];

        let input = data.join("\n");
        let mut record = input.as_str();

        let xref = Xref::parse(&mut record).unwrap();
        assert!("@I1@" == xref.xref.unwrap());
    }
}
