/// mod.rs
// top-level record types
mod address;
mod corporation;
mod datetime;
mod event;
mod family;
mod gedc;
mod header;
mod individual;
mod line;
mod map;
mod note;
mod object;
mod place;
mod quay;
mod source;
mod source_citation;
mod sourcedata;
mod submitter;

pub use address::*;
pub use datetime::DateTime;
pub use event::EventTypeCitedFrom;
pub use family::Family;
pub use gedc::{Form, Gedc};
pub use header::Header;
pub use individual::*;
pub use line::Line;
pub use map::Map;
pub use note::Note;
pub use object::Object;
pub use place::Place;
pub use quay::Quay;
pub use source::Source;
pub use source_citation::SourceCitation;
pub use sourcedata::SourceData;
pub use submitter::Submitter;

#[derive(Debug, Default)]
pub struct Gedcom {
    pub header: Header,
    pub individuals: Vec<Individual>,
}
