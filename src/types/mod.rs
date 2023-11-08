// type Xref = String;

// top-level record types
mod address;
mod copyright;
mod corporation;
mod datetime;
mod gedc;
mod header;
mod individual;
mod line;
mod note;
mod source;
mod sourcedata;

// use std::collections::binary_heap::Iter;

pub use address::*;
pub use copyright::Copyright;
pub use datetime::DateTime;
// pub use gedcom::Gedcom;
pub use gedc::Gedc;
pub use header::*;
pub use individual::*;
pub use line::Line;
pub use note::Note;
pub use source::Source;
pub use sourcedata::SourceData;

#[derive(Debug, Default)]
pub struct Gedcom {
    // It would be nice to drop the Option<> but need to figure out how
    // to do it with the parser setup
    pub header: Header,
    pub individuals: Vec<Individual>,
}

/// Parse a GEDCOM file into a Gedcom struct
///
pub fn parse() {}

// pub fn slurp <'a, I> (iter: I) -> Vec<Line>
// where
//     I: Iterator<Item = &'a Line>,
// {
//     let mut lines: Vec<Line> = Vec::new();
//     let mut i = iter.peekable();

//     while let Some(line) = i.next() {
//         if line.level > 1 {
//             lines.push(line.clone());
//         } else {
//             break;
//         }
//     }
//     lines
// }
