// use crate::types::Line;
use crate::parse;

use winnow::prelude::*;

// use nom::{bytes::complete::is_not, IResult};

// 0 @M1@ OBJE
// 1 FILE photo.jpeg
// 2 FORM JPEG
// 3 TYPE photo
// 2 TITL Picture of the book cover
// 1 REFN 01234567890123456789
// 2 TYPE reference
// 1 RIN 1
// 1 NOTE Here are some notes on this multimedia object.
// 2 CONT If decoded it should be an image of a flower.
// 1 NOTE @N1@
// 1 CHAN
// 2 DATE 14 JAN 2001
// 3 TIME 14:10:31

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Object {
    pub xref: Option<String>,
}

impl Object {
    pub fn parse(buffer: &mut &str) -> PResult<Object> {
        let mut obje = Object { xref: None };

        obje.xref = parse::get_tag_value(buffer)?;

        Ok(obje)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::Object;

    #[test]
    fn parse_obje() {
        // 1 OBJE @M7@

        let data = vec!["1 OBJE @M7@"];

        let input = data.join("\n");
        let mut record = input.as_str();
        let obje = Object::parse(&mut record);
        let o = obje.unwrap().xref.unwrap();

        assert!(o == "@M7@");
    }
}
