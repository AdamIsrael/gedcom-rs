// type Xref = String;

// top-level record types
mod address;
mod corporation;
mod header;
mod line;
mod source;

// use std::collections::binary_heap::Iter;

pub use address::*;
pub use header::*;
pub use line::Line;
pub use source::Source;

#[derive(Debug, Default)]
pub struct Gedcom {
    // It would be nice to drop the Option<> but need to figure out how
    // to do it with the parser setup
    pub header: Header,
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
