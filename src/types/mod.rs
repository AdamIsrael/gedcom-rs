/// mod.rs
// top-level record types
mod address;
mod adopted_by;
mod character_set;
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
mod pedigree;
mod place;
mod quay;
mod source;
mod source_citation;
mod sourcedata;
mod submission;
mod submitter;

pub use address::*;
pub use adopted_by::AdoptedBy;
pub use character_set::CharacterSet;
pub use datetime::DateTime;
pub use event::{EventDetail, EventTypeCitedFrom};
pub use family::Family;
pub use gedc::{Form, Gedc};
pub use header::Header;
pub use individual::*;
pub use line::Line;
pub use map::Map;
pub use note::Note;
pub use object::Object;
pub use pedigree::Pedigree;
pub use place::Place;
pub use quay::Quay;
pub use source::Source;
pub use source_citation::SourceCitation;
pub use sourcedata::SourceData;
pub use submission::Submission;
pub use submitter::Submitter;

#[derive(Debug, Default)]
pub struct Gedcom {
    pub header: Header,
    pub individuals: Vec<Individual>,
}
