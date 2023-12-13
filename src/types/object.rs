// use crate::types::Line;

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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Object<'a> {
    pub xref: &'a str,
}

impl<'b> Object<'b> {
    pub fn parse(_record: &str) -> Object {
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
        Object { xref: "" }
    }
}
