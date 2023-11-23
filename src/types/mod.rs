/// mod.rs
// top-level record types
mod address;
mod corporation;
mod datetime;
mod gedc;
mod header;
mod individual;
mod line;
mod note;
mod source;
mod sourcedata;
mod submitter;

pub use address::*;
pub use datetime::DateTime;
pub use gedc::{Form, Gedc};
pub use header::Header;
pub use individual::*;
pub use line::Line;
pub use note::Note;
pub use source::Source;
pub use sourcedata::SourceData;
pub use submitter::Submitter;

#[derive(Debug, Default)]
pub struct Gedcom {
    pub header: Header,
    pub individuals: Vec<Individual>,
}
