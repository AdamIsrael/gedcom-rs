use crate::types::SourceCitation;

#[derive(Debug, Default)]
pub struct Birth {
    pub date: String,
    pub place: String,
    pub sources: Vec<SourceCitation>,
}
