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
mod multimedia_record;
mod note;
mod note_record;
mod object;
mod pedigree;
mod place;
mod quay;
mod repository_record;
mod source;
mod source_citation;
mod source_record;
mod sourcedata;
mod spouse;
mod submission;
mod submitter;
mod xref;

pub use address::*;
pub use adopted_by::AdoptedBy;
pub use character_set::CharacterSet;
pub use datetime::DateTime;
pub use event::{EventDetail, EventTypeCitedFrom, FamilyEventDetail};
pub use family::Family;
pub use gedc::{Form, Gedc};
pub use header::Header;
pub use individual::*;
pub use line::Line;
pub use map::Map;
pub use multimedia_record::{MultimediaFile, MultimediaRecord};
pub use note::Note;
pub use note_record::NoteRecord;
pub use object::Object;
pub use pedigree::Pedigree;
pub use place::Place;
pub use quay::Quay;
pub use repository_record::RepositoryRecord;
pub use source::Source;
pub use source_citation::SourceCitation;
pub use source_record::{SourceDataEvent, SourceRecord, SourceRecordData, UserReference};
pub use sourcedata::SourceData;
pub use spouse::Spouse;
pub use submission::Submission;
pub use submitter::Submitter;
pub use xref::Xref;

#[derive(Debug, Default)]
pub struct Gedcom {
    pub header: Header,
    pub individuals: Vec<Individual>,
    pub families: Vec<Family>,
    pub sources: Vec<SourceRecord>,
    pub repositories: Vec<RepositoryRecord>,
    pub notes: Vec<NoteRecord>,
    pub multimedia: Vec<MultimediaRecord>,
}
