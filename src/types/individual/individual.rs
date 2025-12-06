use std::str::FromStr;

use crate::parse;
use crate::types::individual::name::*;
use crate::types::{Family, Line, Note, Object, SourceCitation, UserReference, Xref};
use winnow::prelude::*;

use super::{Adoption, Birth, Christening, Death, IndividualEventDetail, Residence};

// n @XREF:INDI@ INDI
// +1 RESN <RESTRICTION_NOTICE>
// +1 <<PERSONAL_NAME_STRUCTURE>>
// +1 SEX <SEX_VALUE>
// +1 <<INDIVIDUAL_EVENT_STRUCTURE>>
// +1 <<INDIVIDUAL_ATTRIBUTE_STRUCTURE>>
// +1 <<LDS_INDIVIDUAL_ORDINANCE>>
// +1 <<CHILD_TO_FAMILY_LINK>>
// +1 <<SPOUSE_TO_FAMILY_LINK>>
// +1 SUBM @<XREF:SUBM>@
// +1 <<ASSOCIATION_STRUCTURE>>
// +1 ALIA @<XREF:INDI>@
// +1 ANCI @<XREF:SUBM>@
// +1 DESI @<XREF:SUBM>@
// +1 RFN <PERMANENT_RECORD_FILE_NUMBER>
// +1 AFN <ANCESTRAL_FILE_NUMBER>
// +1 REFN <USER_REFERENCE_NUMBER>
// +2 TYPE <USER_REFERENCE_TYPE>
// +1 RIN <AUTOMATED_RECORD_ID>
// +1 <<CHANGE_DATE>>
// +1 <<NOTE_STRUCTURE>>
// +1 <<SOURCE_CITATION>>
// +1 <<MULTIMEDIA_LINK>>

// ASSOCIATION_STRUCTURE:=
// n ASSO @<XREF:INDI>@
//   +1 RELA <RELATION_IS_DESCRIPTOR>
//   +1 <<NOTE_STRUCTURE>>
//   +1 <<SOURCE_CITATION>>

/// Association to another individual
#[derive(Debug, Clone, Default)]
pub struct Association {
    /// Reference to associated individual
    pub xref: Xref,

    /// Relationship descriptor
    pub relation: Option<String>,

    /// Notes about this association
    pub notes: Vec<Note>,

    /// Source citations
    pub source_citations: Vec<SourceCitation>,
}

impl Association {
    fn parse(record: &mut &str) -> PResult<Association> {
        let mut assoc = Association::default();

        let Ok(level_line) = Line::peek(record) else {
            return Ok(assoc);
        };

        if level_line.tag != "ASSO" {
            return Ok(assoc);
        }

        assoc.xref = Xref::new(level_line.value.to_string());
        let assoc_level = level_line.level;
        let _ = Line::parse(record);

        // Parse ASSO subfields
        while !record.is_empty() {
            let Ok(line) = Line::peek(record) else {
                break;
            };

            if line.level <= assoc_level {
                break;
            }

            if line.level != assoc_level + 1 {
                let _ = Line::parse(record);
                continue;
            }

            let mut consume = true;

            match line.tag {
                "RELA" => {
                    assoc.relation = Some(line.value.to_string());
                }
                "NOTE" => {
                    if let Ok(Some(note)) = parse::get_tag_value(record) {
                        assoc.notes.push(Note { note: Some(note) });
                    }
                    consume = false;
                }
                "SOUR" => {
                    if let Ok(citation) = SourceCitation::parse(record) {
                        assoc.source_citations.push(citation);
                    }
                    consume = false;
                }
                _ => {}
            }

            if consume {
                let _ = Line::parse(record);
            }
        }

        Ok(assoc)
    }
}

/// Individual attribute (EDUC, DSCR, RELI, etc.)
#[derive(Debug, Clone, Default)]
pub struct IndividualAttribute {
    /// The attribute value
    pub value: String,

    /// Type descriptor
    pub attribute_type: Option<String>,

    /// Date of attribute
    pub date: Option<String>,

    /// Place of attribute
    pub place: Option<String>,

    /// Source citations
    pub source_citations: Vec<SourceCitation>,

    /// Notes
    pub notes: Vec<Note>,
}

impl IndividualAttribute {
    fn parse(record: &mut &str, value: &str) -> PResult<IndividualAttribute> {
        let mut attr = IndividualAttribute {
            value: value.to_string(),
            ..Default::default()
        };

        let Ok(level_line) = Line::peek(record) else {
            return Ok(attr);
        };

        let attr_level = level_line.level;
        let _ = Line::parse(record);

        // Parse attribute subfields
        while !record.is_empty() {
            let Ok(line) = Line::peek(record) else {
                break;
            };

            if line.level <= attr_level {
                break;
            }

            if line.level != attr_level + 1 {
                let _ = Line::parse(record);
                continue;
            }

            let mut consume = true;

            match line.tag {
                "TYPE" => {
                    attr.attribute_type = Some(line.value.to_string());
                }
                "DATE" => {
                    attr.date = Some(line.value.to_string());
                }
                "PLAC" => {
                    attr.place = Some(line.value.to_string());
                }
                "NOTE" => {
                    if let Ok(Some(note)) = parse::get_tag_value(record) {
                        attr.notes.push(Note { note: Some(note) });
                    }
                    consume = false;
                }
                "SOUR" => {
                    if let Ok(citation) = SourceCitation::parse(record) {
                        attr.source_citations.push(citation);
                    }
                    consume = false;
                }
                _ => {}
            }

            if consume {
                let _ = Line::parse(record);
            }
        }

        Ok(attr)
    }
}

#[derive(Debug, Default)]
pub struct Individual {
    // Common events - keep as Vec
    pub birth: Vec<Birth>,
    pub death: Vec<Death>,
    pub christening: Vec<Christening>,
    pub names: Vec<PersonalName>,
    pub residences: Vec<Residence>,
    pub famc: Vec<Family>,
    pub fams: Vec<Family>,
    pub gender: super::Gender,

    // Rare events - use Option<Vec<T>>
    pub adoption: Option<Vec<Adoption>>,
    pub baptism: Option<Vec<IndividualEventDetail>>,
    pub barmitzvah: Option<Vec<IndividualEventDetail>>,
    pub basmitzvah: Option<Vec<IndividualEventDetail>>,
    pub blessing: Option<Vec<IndividualEventDetail>>,
    pub burial: Option<Vec<IndividualEventDetail>>,
    pub census: Option<Vec<IndividualEventDetail>>,
    pub christening_adult: Option<Vec<Christening>>,
    pub confirmation: Option<Vec<IndividualEventDetail>>,
    pub first_communion: Option<IndividualEventDetail>,
    pub cremation: Option<Vec<IndividualEventDetail>>,
    pub emigration: Option<Vec<IndividualEventDetail>>,
    pub graduation: Option<Vec<IndividualEventDetail>>,
    pub immigration: Option<Vec<IndividualEventDetail>>,
    pub naturalization: Option<Vec<IndividualEventDetail>>,
    pub probate: Option<Vec<IndividualEventDetail>>,
    pub retirement: Option<Vec<IndividualEventDetail>>,
    pub will: Option<Vec<IndividualEventDetail>>,

    /// Generic events not covered by a specific type
    pub events: Option<Vec<IndividualEventDetail>>,

    /// The XRef pointer associated with this individual
    pub xref: Option<Xref>,

    // Record-level fields (GEDCOM 5.5.1)
    /// Restriction notice
    pub restriction: Option<String>,

    /// Record-level notes
    pub notes: Vec<Note>,

    /// Record-level source citations
    pub source_citations: Vec<SourceCitation>,

    /// Record-level multimedia links
    pub multimedia_links: Vec<Object>,

    /// User reference numbers
    pub user_reference_numbers: Vec<UserReference>,

    /// Automated record ID
    pub automated_record_id: Option<String>,

    /// Change date - stores the DATE value from CHAN/DATE
    pub change_date: Option<String>,

    /// Submitter references
    pub submitters: Vec<Xref>,

    /// Associations to other individuals
    pub associations: Vec<Association>,

    /// Alias (link to another individual record)
    pub alias: Vec<Xref>,

    /// Ancestor interest (submitter interested in ancestors)
    pub ancestor_interest: Vec<Xref>,

    /// Descendant interest (submitter interested in descendants)
    pub descendant_interest: Vec<Xref>,

    /// Permanent record file number
    pub permanent_record_file_number: Option<String>,

    /// Ancestral file number
    pub ancestral_file_number: Option<String>,

    // Attributes
    /// Education
    pub education: Option<Vec<IndividualAttribute>>,

    /// Physical description
    pub physical_description: Option<Vec<IndividualAttribute>>,

    /// Religion
    pub religion: Option<Vec<IndividualAttribute>>,

    /// National ID number
    pub national_id_number: Option<Vec<IndividualAttribute>>,

    /// Property/possessions
    pub property: Option<Vec<IndividualAttribute>>,

    /// Caste name
    pub caste: Option<Vec<IndividualAttribute>>,

    /// Number of children
    pub number_of_children: Option<Vec<IndividualAttribute>>,

    /// Number of marriages
    pub number_of_marriages: Option<Vec<IndividualAttribute>>,

    /// Nobility title
    pub nobility_title: Option<Vec<IndividualAttribute>>,

    /// National or tribal origin
    pub national_origin: Option<Vec<IndividualAttribute>>,
}

// Macro to handle repetitive event parsing with proper error handling
macro_rules! parse_event {
    // For Vec<T> fields
    ($record:expr, $field:expr, $parser:ty) => {{
        if let Ok(event) = <$parser>::parse($record) {
            $field.push(event);
        }
        false
    }};
    // For Option<Vec<T>> fields
    (option_vec, $record:expr, $field:expr, $parser:ty) => {{
        if let Ok(event) = <$parser>::parse($record) {
            $field.get_or_insert_with(Vec::new).push(event);
        }
        false
    }};
    // For Option<T> fields (single value)
    (option, $record:expr, $field:expr, $parser:ty) => {{
        if let Ok(event) = <$parser>::parse($record) {
            $field = Some(event);
        }
        false
    }};
}

// impl<'a> Individual<'a> {
impl Individual {
    /// Helper function to extract xref from either the xref or value field
    #[inline]
    fn extract_xref(line: &Line) -> Option<Xref> {
        let xref_str = if !line.xref.is_empty() {
            line.xref
        } else {
            line.value
        };

        if !xref_str.is_empty() {
            Some(Xref::new(xref_str.to_string()))
        } else {
            None
        }
    }

    pub fn parse(record: &mut &str) -> Individual {
        // pub fn parse(mut record: String) -> Individual {
        let mut individual = Individual::default();

        while !record.is_empty() {
            let Ok(line) = Line::peek(record) else {
                break;
            };

            // Flag to track if we should consume the next line in record
            let mut parse = true;

            match line.level {
                0 => {
                    individual.xref = Some(Xref::new(line.xref.to_string()));
                }
                1 => {
                    match line.tag {
                        "NAME" => {
                            parse = parse_event!(record, individual.names, PersonalName);
                        }
                        "SEX" => {
                            if let Ok(gender) = super::Gender::from_str(line.value) {
                                individual.gender = gender;
                            }
                        }
                        "BIRT" => {
                            parse = parse_event!(record, individual.birth, Birth);
                        }
                        "DEAT" => {
                            // TODO: Support 1 DEAT Y
                            parse = parse_event!(record, individual.death, Death);
                        }
                        "FAMS" => {
                            let fam = Family::parse(record);
                            individual.fams.push(fam);
                            parse = false;
                        }
                        "FAMC" => {
                            let fam = Family::parse(record);
                            individual.famc.push(fam);
                            parse = false;
                        }
                        // baptism
                        "BAPM" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.baptism,
                                IndividualEventDetail
                            );
                        }
                        // christening
                        "CHR" => {
                            parse = parse_event!(record, individual.christening, Christening);
                        }
                        // bar mitzvah
                        "BARM" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.barmitzvah,
                                IndividualEventDetail
                            );
                        }
                        // bas mitzvah
                        "BASM" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.basmitzvah,
                                IndividualEventDetail
                            );
                        }
                        // blessing
                        "BLES" => {
                            // TODO: Need to add tests for this
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.blessing,
                                IndividualEventDetail
                            );
                        }
                        // Adoption
                        "ADOP" => {
                            parse = parse_event!(option_vec, record, individual.adoption, Adoption);
                        }
                        // Adult Christening
                        "CHRA" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.christening_adult,
                                Christening
                            );
                        }
                        // Confirmation
                        "CONF" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.confirmation,
                                IndividualEventDetail
                            );
                        }
                        "FCOM" => {
                            parse = parse_event!(
                                option,
                                record,
                                individual.first_communion,
                                IndividualEventDetail
                            );
                        }
                        "GRAD" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.graduation,
                                IndividualEventDetail
                            );
                        }
                        "EMIG" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.emigration,
                                IndividualEventDetail
                            );
                        }
                        "IMMI" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.immigration,
                                IndividualEventDetail
                            );
                        }
                        "NATU" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.naturalization,
                                IndividualEventDetail
                            );
                        }
                        "CENS" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.census,
                                IndividualEventDetail
                            );
                        }
                        "RETI" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.retirement,
                                IndividualEventDetail
                            );
                        }
                        // probate
                        "PROB" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.probate,
                                IndividualEventDetail
                            );
                        }
                        // burial
                        "BURI" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.burial,
                                IndividualEventDetail
                            );
                        }
                        // Will
                        "WILL" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.will,
                                IndividualEventDetail
                            );
                        }
                        // Cremation
                        "CREM" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.cremation,
                                IndividualEventDetail
                            );
                        }
                        // generic event
                        "EVEN" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.events,
                                IndividualEventDetail
                            );
                        }
                        // residence
                        "RESI" => {
                            parse = parse_event!(record, individual.residences, Residence);
                        }
                        // occupation
                        "OCCU" => {
                            parse = parse_event!(
                                option_vec,
                                record,
                                individual.events,
                                IndividualEventDetail
                            );
                        }
                        "EDUC" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual.education.get_or_insert_with(Vec::new).push(attr);
                            }
                            parse = false;
                        }
                        // physical description
                        "DSCR" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual
                                    .physical_description
                                    .get_or_insert_with(Vec::new)
                                    .push(attr);
                            }
                            parse = false;
                        }
                        // religion
                        "RELI" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual.religion.get_or_insert_with(Vec::new).push(attr);
                            }
                            parse = false;
                        }
                        // national identification number
                        "IDNO" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual
                                    .national_id_number
                                    .get_or_insert_with(Vec::new)
                                    .push(attr);
                            }
                            parse = false;
                        }
                        // property/possessions
                        "PROP" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual.property.get_or_insert_with(Vec::new).push(attr);
                            }
                            parse = false;
                        }
                        // cast(e) name?
                        "CAST" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual.caste.get_or_insert_with(Vec::new).push(attr);
                            }
                            parse = false;
                        }
                        // number of children
                        "NCHI" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual
                                    .number_of_children
                                    .get_or_insert_with(Vec::new)
                                    .push(attr);
                            }
                            parse = false;
                        }
                        // number of marriages
                        "NMR" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual
                                    .number_of_marriages
                                    .get_or_insert_with(Vec::new)
                                    .push(attr);
                            }
                            parse = false;
                        }
                        // nobility title
                        "TITL" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual
                                    .nobility_title
                                    .get_or_insert_with(Vec::new)
                                    .push(attr);
                            }
                            parse = false;
                        }
                        // national or tribe origin
                        "NATI" => {
                            if let Ok(attr) = IndividualAttribute::parse(record, line.value) {
                                individual
                                    .national_origin
                                    .get_or_insert_with(Vec::new)
                                    .push(attr);
                            }
                            parse = false;
                        }
                        "NOTE" => {
                            if let Ok(note) = Note::parse(record) {
                                individual.notes.push(note);
                            }
                            parse = false;
                        }
                        // source records
                        "SOUR" => {
                            if let Ok(citation) = SourceCitation::parse(record) {
                                individual.source_citations.push(citation);
                            }
                            parse = false;
                        }
                        // multimedia links
                        "OBJE" => {
                            if let Ok(obj) = Object::parse(record) {
                                individual.multimedia_links.push(obj);
                            }
                            parse = false;
                        }
                        "ASSO" => {
                            if let Ok(assoc) = Association::parse(record) {
                                individual.associations.push(assoc);
                            }
                            parse = false;
                        }
                        "REFN" => {
                            let number = line.value.to_string();
                            let refn_level = line.level;
                            let _ = Line::parse(record);

                            // Check for TYPE subfield
                            let mut ref_type = None;
                            if let Ok(subline) = Line::peek(record) {
                                if subline.tag == "TYPE" && subline.level == refn_level + 1 {
                                    ref_type = Some(subline.value.to_string());
                                    let _ = Line::parse(record);
                                }
                            }

                            individual
                                .user_reference_numbers
                                .push(UserReference { number, ref_type });
                            parse = false;
                        }
                        "RIN" => {
                            individual.automated_record_id = Some(line.value.to_string());
                        }
                        "CHAN" => {
                            // Parse CHAN structure to get DATE value
                            let level = line.level;
                            let _ = Line::parse(record); // consume CHAN line

                            while !record.is_empty() {
                                let Ok(inner_line) = Line::peek(record) else {
                                    break;
                                };

                                if inner_line.level <= level {
                                    break;
                                }

                                if inner_line.level == level + 1 && inner_line.tag == "DATE" {
                                    individual.change_date = Some(inner_line.value.to_string());
                                }

                                let _ = Line::parse(record);
                            }
                            parse = false;
                        }
                        "RESN" => {
                            individual.restriction = Some(line.value.to_string());
                        }
                        "SUBM" => {
                            if let Some(xref) = Self::extract_xref(&line) {
                                individual.submitters.push(xref);
                            }
                        }
                        "ALIA" => {
                            if let Some(xref) = Self::extract_xref(&line) {
                                individual.alias.push(xref);
                            }
                        }
                        "ANCI" => {
                            if let Some(xref) = Self::extract_xref(&line) {
                                individual.ancestor_interest.push(xref);
                            }
                        }
                        "DESI" => {
                            if let Some(xref) = Self::extract_xref(&line) {
                                individual.descendant_interest.push(xref);
                            }
                        }
                        "RFN" => {
                            individual.permanent_record_file_number = Some(line.value.to_string());
                        }
                        "AFN" => {
                            individual.ancestral_file_number = Some(line.value.to_string());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            // Consume the line
            if parse {
                let _ = Line::parse(record);
            }
        }

        individual
    }
}

#[derive(Debug)]
/// The type of the name.
///
/// Not sure when/where to use this yet but I wanted to capture it from the spec.
pub enum NameType {
    Alias,
    Birth,
    Immigrant,
    Maiden,
    Married,
    Other,
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AdoptedBy, Quay};

    #[test]
    fn parse_indi_baptism() {
        let data: Vec<&str> = vec![
            "1 BAPS",
            "2 DATE ABT 31 DEC 1997",
            "2 PLAC The place",
            "2 AGE 3m",
            "2 TYPE BAPM",
            "2 ADDR",
            "3 ADR1 Church Name",
            "3 ADR2 Street Address",
            "3 CITY City Name",
            "3 POST zip",
            "3 CTRY Country",
            "2 CAUS Birth",
            "2 AGNC The Church",
            "2 OBJE @M8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Sample baptism Source text.",
            "3 QUAY 3",
            "3 NOTE A baptism source note.",
            "2 NOTE A baptism event note (the event of baptism (not LDS), performed in infancy or later. See also BAPL and CHR).",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let baptism = IndividualEventDetail::parse(&mut record).unwrap();
        let detail = baptism.detail;

        assert!(detail.date.is_some());
        assert!(detail.date.unwrap() == "ABT 31 DEC 1997");

        assert!(detail.place.is_some());
        let place = detail.place.unwrap();
        assert!(place.name.is_some());
        assert!(place.name.unwrap() == "The place");

        assert!(detail.address.is_some());
        let addr = detail.address.unwrap();
        assert!(addr.addr1.is_some());
        assert!(addr.addr1.unwrap() == "Church Name");
        assert!(addr.addr2.is_some());
        assert!(addr.addr2.unwrap() == "Street Address");
        assert!(addr.city.is_some());
        assert!(addr.city.unwrap() == "City Name");
        assert!(addr.postal_code.is_some());
        assert!(addr.postal_code.unwrap() == "zip");
        assert!(addr.country.is_some());
        assert!(addr.country.unwrap() == "Country");

        assert!(detail.cause.is_some());
        assert!(detail.cause.unwrap() == "Birth");

        assert!(detail.agency.is_some());
        assert!(detail.agency.unwrap() == "The Church");

        assert!(detail.media.len() == 1);

        assert!(detail.sources.len() == 1);

        assert!(detail.note.is_some());
        assert!(detail.note.unwrap().starts_with("A baptism event note"));
    }

    #[test]
    fn parse_indi_complete() {
        let data: Vec<&str> = vec![
            "0 @I1@ INDI",
            "1 NAME Joseph Tag /Torture/",
            "2 TYPE birth",
            "2 NPFX Prof.",
            "2 GIVN Joseph",
            "2 NICK Joe",
            "2 SPFX Le",
            "2 SURN Torture",
            "2 NSFX Jr.",
            "2 NOTE These are notes about the first NAME structure in this record. These notes are ",
            "3 CONC embedded in the INDIVIDUAL record itself.",
            "3 CONT ",
            "3 CONT This name structure uses all possible tags for a personal name structure.",
            "3 CONT ",
            "3 CONT NOTE: many applications are confused by two NAME structures.",
            "2 SOUR @S1@",
            "3 PAGE 55",
            "3 EVEN BIRT",
            "4 ROLE CHIL",
            "3 DATA",
            "4 DATE 1 JAN 1900",
            "4 TEXT Here is some text from the source specific to this source ",
            "5 CONC citation.",
            "5 CONT Here is more text but on a new line.",
            "3 OBJE @M8@",
            "3 NOTE @N7@",
            "3 QUAY 0",
            "2 FONE Joseph Tag /Torture/",
            "3 TYPE user defined",
            "3 NPFX Prof.",
            "3 GIVN Joseph",
            "3 NICK Joe",
            "3 SPFX Le",
            "3 SURN Torture",
            "3 NSFX Jr.",
            "3 NOTE Phonetisation",
            "3 SOUR @S1@",
            "4 PAGE 55",
            "4 EVEN BIRT",
            "5 ROLE CHIL",
            "4 DATA",
            "5 DATE 1 JAN 1900",
            "5 TEXT Here is some text from the source specific to this source ",
            "6 CONC citation.",
            "6 CONT Here is more text but on a new line.",
            "4 OBJE @M8@",
            "4 NOTE @N7@",
            "4 QUAY 0",
            "2 ROMN Joseph Tag /Torture/",
            "3 TYPE user defined",
            "3 NPFX Prof.",
            "3 GIVN Joseph",
            "3 NICK Joe",
            "3 SPFX Le",
            "3 SURN Torture",
            "3 NSFX Jr.",
            "3 NOTE Romanisation",
            "3 SOUR @S1@",
            "4 PAGE 55",
            "4 EVEN BIRT",
            "5 ROLE CHIL",
            "4 DATA",
            "5 DATE 1 JAN 1900",
            "5 TEXT Here is some text from the source specific to this source ",
            "6 CONC citation.",
            "6 CONT Here is more text but on a new line.",
            "4 OBJE @M8@",
            "4 NOTE @N7@",
            "4 QUAY 0",
            "1 NAME William John /Smith/",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "2 NOTE This is a second personal NAME structure in a single INDIVIDUAL record ",
            "3 CONC which is allowed in GEDCOM.",
            "3 CONT ",
            "3 CONT These notes are embedded in the INDIVIDUAL record.",
            "1 SEX M",
            "1 BIRT",
            "2 TYPE Normal",
            "2 DATE 31 DEC 1965",
            "2 PLAC Salt Lake City, UT, USA",
            "3 FONE Salt Lake City, UT, USA",
            "4 TYPE user defined",
            "3 ROMN Salt Lake City, UT, USA",
            "4 TYPE user defined",
            "3 MAP",
            "4 LATI N0",
            "4 LONG E0",
            "3 NOTE Place note",
            "2 ADDR",
            "3 ADR1 St. Marks Hospital",
            "3 CITY Salt Lake City",
            "3 STAE UT",
            "3 POST 84121",
            "3 CTRY USA",
            "2 AGNC none",
            "2 RELI Religion",
            "2 CAUS Conception",
            "2 NOTE @N8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 EVEN BIRT",
            "4 ROLE CHIL",
            "3 DATA",
            "4 DATE 1 JAN 1900",
            "4 TEXT Here is some text from the source specific to this source ",
            "5 CONC citation.",
            "5 CONT Here is more text but on a new line.",
            "3 OBJE @M8@",
            "3 NOTE Some notes about this birth source citation which are embedded in the citation ",
            "4 CONC structure itself.",
            "3 QUAY 2",
            "2 OBJE @M15@",
            "2 AGE 0y",
            "2 FAMC @F2@",
            "1 BIRT",
            "2 TYPE Normal",
            "2 DATE ABT. DEC 1965",
            "1 DEAT",
            "2 DATE ABT 15 JAN 2001",
            "2 PLAC New York, New York, USA",
            "3 NOTE The place structure has more detail than usually used for places",
            "2 AGE 76y",
            "2 TYPE slow",
            "2 ADDR",
            "3 ADR1 at Home",
            "2 CAUS Cancer",
            "2 AGNC none",
            "2 OBJE @M8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some death source text.",
            "3 QUAY 3",
            "3 NOTE A death source note.",
            "2 NOTE A death event note.",
            "1 FAMS @F1@",
            "2 NOTE Note about the link to the family record with his first spouse.",
            "2 NOTE Another note about the link to the family record with his first spouse.",
            "1 FAMS @F4@",
            "1 FAMC @F2@",
            "2 NOTE Note about this link to his parents family record.",
            "2 NOTE Another note about this link to his parents family record",
            "1 FAMC @F3@",
            "2 PEDI adopted",
            "2 NOTE Note about the link to his adoptive parents family record.",
            "1 BAPM",
            "2 DATE ABT 31 DEC 1997",
            "2 PLAC The place",
            "2 AGE 3m",
            "2 TYPE BAPM",
            "2 ADDR",
            "3 ADR1 Church Name",
            "3 ADR2 Street Address",
            "3 CITY City Name",
            "3 POST zip",
            "3 CTRY Country",
            "2 CAUS Birth",
            "2 AGNC The Church",
            "2 OBJE @M8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Sample baptism Source text.",
            "3 QUAY 3",
            "3 NOTE A baptism source note.",
            "2 NOTE A baptism event note (the event of baptism (not LDS), performed in infancy or later. See also BAPL and CHR).",
            "1 CHR",
            "2 DATE CAL 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE CHR",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Sample CHR Source text.",
            "3 QUAY 3",
            "3 NOTE A christening Source note.",
            "2 NOTE Christening event note (the religious event (not LDS) of baptizing and/or naming a ",
            "3 CONC child).",
            "2 FAMC @F3@",
            "1 CHR",
            "2 DATE EST 30 DEC 1997",
            "2 PLAC The place",
            "2 TYPE CHR",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some christening source text.",
            "5 CONT This is the second christening structure.",
            "3 QUAY 3",
            "3 NOTE A christening Source note.",
            "2 NOTE Alternative christening event note. GEDOM allows more than one of the same type ",
            "3 CONC of event.",
            "1 BARM",
            "2 DATE AFT 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE BARM",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some Bar Mitzvah source text.",
            "3 QUAY 3",
            "3 NOTE A Bar Mitzvah source note.",
            "2 NOTE Bar Mitzvah event note (the ceremonial event held when a Jewish boy reaches age ",
            "3 CONC 13).",
            "1 BASM",
            "2 DATE AFT 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE BASM",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some Bas Mitzvah source text.",
            "3 QUAY 3",
            "3 NOTE A Bas Mitzvah source note.",
            "2 NOTE Bas Mitzvah event note (the ceremonial event held when a Jewish girl reaches age 13, ",
            "3 CONC also known as \"Bat Mitzvah\").",
            "1 ADOP",
            "2 DATE BEF 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE ADOP",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some adoption source text.",
            "3 QUAY 3",
            "3 NOTE An adoption source note.",
            "2 NOTE Adoption event note (pertaining to creation of a child-parent relationship that does ",
            "3 CONC not exist biologically).",
            "2 FAMC @F3@",
            "3 ADOP BOTH",
            "1 CHRA",
            "2 DATE BET 31 DEC 1997 AND 1 FEB 1998",
            "2 PLAC The place",
            "2 TYPE CHRA",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some christening source text.",
            "3 QUAY 3",
            "3 NOTE A christening source note.",
            "2 NOTE Adult christening event note (the religious event (not LDS) of baptizing and/or ",
            "3 CONC naming an adult person).",
            "1 CONF",
            "2 DATE BET 31 DEC 1997 AND 2 JAN 1998",
            "2 PLAC The place",
            "2 TYPE CONF",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some CONF Source text.",
            "3 QUAY 3",
            "3 NOTE A CONF Source note.",
            "2 NOTE CONFIRMATION event note (the religious event (not LDS) of conferring the gift of the Holy Ghost and, among protestants, full church membership).",
            "1 FCOM",
            "2 DATE INT 31 DEC 1997 (a test)",
            "2 PLAC The place",
            "2 TYPE FCOM",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some first communion source text.",
            "3 QUAY 3",
            "3 NOTE An first communion source note.",
            "2 NOTE First communion event note (a religious rite, the first act of sharing in the Lord's ",
            "3 CONC supper as part of church worship).",
            "1 GRAD",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE GRAD",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some graduation source text.",
            "3 QUAY 3",
            "3 NOTE A graduation source note.",
            "2 NOTE Graduation event note (an event of awarding educational diplomas or degrees to ",
            "3 CONC individuals).",
            "1 EMIG",
            "2 DATE 1997",
            "2 PLAC The place",
            "2 TYPE EMIG",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some emigration source text.",
            "3 QUAY 3",
            "3 NOTE An emigration source note.",
            "2 NOTE Emigration event note (an event of leaving one's homeland with the intent of residing ",
            "3 CONC elsewhere).",
            "1 IMMI",
            "2 DATE DEC 1997",
            "2 PLAC The place",
            "2 TYPE IMMI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some immigration source text.",
            "3 QUAY 3",
            "3 NOTE An immigration source note.",
            "2 NOTE Immigration event note (an event of entering into a new locality with the intent of ",
            "3 CONC residing there).",
            "1 NATU",
            "2 DATE 1100 BCE",
            "2 PLAC The place",
            "2 TYPE NATU",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some naturalization source text.",
            "3 QUAY 3",
            "3 NOTE A naturalization source note.",
            "2 NOTE Naturalization event note (the event of obtaining citizenship).",
            "1 CENS",
            "2 DATE @#DHEBREW@ 2 TVT 5758",
            "2 PLAC The place",
            "2 TYPE CENS",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some census source text.",
            "3 QUAY 3",
            "3 NOTE A census source note.",
            "2 NOTE Census event note (the event of the periodic count of the population for a designated ",
            "3 CONC locality, such as a national or state Census).",
            "1 RETI",
            "2 DATE @#DFRENCH R@ 11 NIVO 0006",
            "2 PLAC The place",
            "2 TYPE RETI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some retirement source text.",
            "3 QUAY 3",
            "3 NOTE A retirement source note.",
            "2 NOTE Retirement event note (an event of exiting an occupational relationship with an ",
            "3 CONC employer after a qualifying time period).",
            "1 PROB",
            "2 DATE FROM @#DHEBREW@ 25 SVN 5757 TO @#DHEBREW@ 26 IYR 5757",
            "2 PLAC The place",
            "2 TYPE PROB",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some probate source text.",
            "3 QUAY 3",
            "3 NOTE A probate source note.",
            "2 NOTE Probate event note (an event of judicial determination of the validity of a will. May ",
            "3 CONC indicate several related court activities over several dates).",
            "1 BURI",
            "2 DATE @#DFRENCH R@ 5 VEND 0010",
            "2 PLAC The place",
            "2 TYPE BURI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some burial source text.",
            "3 QUAY 3",
            "3 NOTE A burial source note.",
            "2 NOTE Burial event note (the event of the proper disposing of the mortal remains of a ",
            "3 CONC deceased person).",
            "1 WILL",
            "2 DATE INT @#DHEBREW@ 2 TVT 5758 (interpreted)",
            "2 PLAC The place",
            "2 TYPE WILL",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some will source text.",
            "3 QUAY 3",
            "3 NOTE A will source note.",
            "2 NOTE Will event note (a legal document treated as an event, by which a person disposes of ",
            "3 CONC his or her estate, to take effect after death. The event date is the date the will was ",
            "3 CONC signed while the person was alive. See also Probate).",
            "1 CREM",
            "2 DATE AFT 15 JAN 2001",
            "1 EVEN",
            "2 DATE 5 MAY 0005",
            "2 PLAC The place",
            "2 TYPE EVEN",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some generic event source text.",
            "3 QUAY 3",
            "3 NOTE A generic event source note.",
            "2 NOTE Generic event note (a noteworthy happening related to an individual, a group, or an ",
            "3 CONC organization). The TYPE tag specifies the type of event.",
            "1 RESI",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 AGE 35y",
            "2 TYPE RESI",
            "2 ADDR",
            "3 ADR1 Special Address Line 1",
            "3 ADR2 Special Address Line 2",
            "3 ADR3 Special Address Line 3",
            "3 CITY City Name",
            "3 STAE State name",
            "3 POST 0123456789",
            "3 CTRY USA",
            "2 PHON +1-800-555-5555",
            "2 CAUS Needed housing",
            "2 AGNC None",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some residence source text.",
            "3 QUAY 3",
            "3 NOTE A residence source note.",
            "2 NOTE Residence attribute note (the act of dwelling at an address for a period of time).",
            "1 OCCU Occupation",
            "2 DATE 31 DEC 1997",
            "2 AGE 40y",
            "2 PLAC The place",
            "2 TYPE OCCU",
            "2 ADDR",
            "3 ADR1 Work address line 1",
            "3 ADR2 Work address line 2",
            "3 ADR3 Work address line 3",
            "3 CITY Work city",
            "3 STAE Work state",
            "3 POST Work post",
            "3 CTRY Work country",
            "2 CAUS Need for money",
            "2 AGNC Employer",
            "2 OBJE @M7@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some occupation source text.",
            "3 QUAY 3",
            "3 NOTE An occupation source note.",
            "2 NOTE Occupation attribute note (the type of work or profession of an individual).",
            "1 OCCU Another occupation",
            "2 DATE 31 DEC 1998",
            "2 PLAC The place",
            "2 TYPE OCCU",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some occupation source text.",
            "3 QUAY 3",
            "3 NOTE An occupation source note.",
            "2 NOTE Occupation attribute note. This is the second occupation attribute in the record.",
            "1 EDUC Education",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE EDUC",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some education source text.",
            "3 QUAY 3",
            "3 NOTE An education source note.",
            "2 NOTE Education attribute note (indicator of a level of education attained).",
            "1 DSCR Physical description",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE PHYS",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some physical description source text.",
            "3 QUAY 3",
            "3 NOTE A physical description source note.",
            "2 NOTE Physical description attribute note (the physical characteristics of a person, place, or ",
            "3 CONC thing).",
            "1 RELI Religion",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE RELI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some religion source text.",
            "3 QUAY 3",
            "3 NOTE A religion source note.",
            "2 NOTE Religion attribute note (a religious denomination to which a person is affiliated or for ",
            "3 CONC which a record applies).",
            "1 IDNO 6942",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE IDNO",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some national identification number source text.",
            "3 QUAY 3",
            "3 NOTE An national identification number source note.",
            "2 NOTE National identification number attribute note (a number assigned to identify a person ",
            "3 CONC within some significant external system).",
            "1 PROP Possessions",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE PROP",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some possessions source text.",
            "3 QUAY 3",
            "3 NOTE @N11@",
            "2 NOTE Possessions or property attribute note (pertaining to possessions such as real estate ",
            "3 CONC or other property of interest).",
            "1 CAST Cast name",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE CAST",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some caste name source text.",
            "3 QUAY 3",
            "3 NOTE A caste name source note.",
            "2 NOTE Caste name attribute note (the name of an individual's rank or status in society, based ",
            "3 CONC on racial or religious differences, or differences in wealth, inherited rank, profession, ",
            "3 CONC occupation, etc).",
            "1 NCHI 42",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE NCHI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some number of children source text.",
            "3 QUAY 3",
            "3 NOTE Am number of children source note.",
            "2 NOTE Number of children attribute note.",
            "1 NMR 42",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE NMR",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some number of marriages source text.",
            "3 QUAY 3",
            "3 NOTE An number of marriages source note.",
            "2 NOTE Number of marriages attribute note.",
            "1 TITL Nobility title",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE TITL",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some title source text.",
            "3 QUAY 3",
            "3 NOTE A title source note.",
            "2 NOTE Title attribute note (a description of a specific writing or other work, such as the title ",
            "3 CONC of a book when used in a source context, or a formal designation used by an ",
            "3 CONC individual in connection with positions of royalty or other social status, ",
            "3 CONT such as Grand Duke).",
            "1 NATI National or tribe origin",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE NATI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some nationality source text.",
            "3 QUAY 3",
            "3 NOTE An nationality source note.",
            "2 NOTE Nationality attribute note (the national heritage of an individual).",
            "1 NOTE @N4@",
            "1 NOTE This is a second set of notes for this single individual record. It is embedded in the ",
            "2 CONC INDIVIDUAL record instead of being in a separate NOTE record.",
            "2 CONT ",
            "2 CONT These notes also have a source citation to a SOURCE record. In GEDCOM ",
            "2 CONC this source can only be a single line and links to a SOURCE record.",
            "1 SOUR @S1@",
            "2 PAGE 42",
            "2 DATA",
            "3 DATE 31 DEC 1900",
            "3 TEXT Some sample text from the first source on this record.",
            "2 QUAY 0",
            "2 NOTE A source note.",
            "1 SOUR @S2@",
            "2 NOTE @N12@",
            "1 OBJE @M7@",
            "1 ASSO @I9@",
            "2 RELA Has multimedia links",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Sample text about this source on an association.",
            "2 NOTE Note on association link.",
            "1 ASSO @I5@",
            "2 RELA Father",
            "1 REFN 01234567890123456789",
            "2 TYPE reference",
            "1 RIN 12",
            "1 CHAN",
            "2 DATE 12 FEB 2001",
            "3 TIME 19:16:42",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let mut indi = Individual::parse(&mut record);

        assert_eq!(2, indi.names.len());
        assert_eq!(Some(Xref::new("@I1@")), indi.xref);

        // Check the name.name
        assert_eq!(
            Some("Joseph Tag /Torture/"),
            indi.names[0].name.value.as_deref()
        );
        assert_eq!(Some("Joseph"), indi.names[0].name.given.as_deref());
        assert_eq!(Some("Torture"), indi.names[0].name.surname.as_deref());
        assert_eq!(Some("Joe"), indi.names[0].name.nickname.as_deref());
        assert_eq!(Some("Prof."), indi.names[0].name.prefix.as_deref());
        assert_eq!(Some("Le"), indi.names[0].name.suffix.as_deref());
        assert_eq!(Some("Jr."), indi.names[0].name.surname_prefix.as_deref());
        assert_eq!(Some("birth"), indi.names[0].name.r#type.as_deref());

        // Check the indi.names[0].romanized
        assert_eq!(
            Some("Joseph Tag /Torture/"),
            indi.names[0].romanized.value.as_deref()
        );
        assert_eq!(Some("Joseph"), indi.names[0].romanized.given.as_deref());
        assert_eq!(Some("Torture"), indi.names[0].romanized.surname.as_deref());
        assert_eq!(Some("Joe"), indi.names[0].romanized.nickname.as_deref());
        assert_eq!(Some("Prof."), indi.names[0].romanized.prefix.as_deref());
        assert_eq!(Some("Le"), indi.names[0].romanized.suffix.as_deref());
        assert_eq!(
            Some("Jr."),
            indi.names[0].romanized.surname_prefix.as_deref()
        );
        assert_eq!(
            Some("user defined"),
            indi.names[0].romanized.r#type.as_deref()
        );

        // Check the indi.names[0].phonetic
        assert_eq!(
            Some("Joseph Tag /Torture/"),
            indi.names[0].phonetic.value.as_deref()
        );
        assert_eq!(Some("Joseph"), indi.names[0].phonetic.given.as_deref());
        assert_eq!(Some("Torture"), indi.names[0].phonetic.surname.as_deref());
        assert_eq!(Some("Joe"), indi.names[0].phonetic.nickname.as_deref());
        assert_eq!(Some("Prof."), indi.names[0].phonetic.prefix.as_deref());
        assert_eq!(Some("Le"), indi.names[0].phonetic.suffix.as_deref());
        assert_eq!(
            Some("Jr."),
            indi.names[0].phonetic.surname_prefix.as_deref()
        );
        assert_eq!(
            Some("user defined"),
            indi.names[0].phonetic.r#type.as_deref()
        );

        // Birth
        assert!(indi.birth.len() == 2);
        let birth = indi.birth.first().unwrap();

        let event = &birth.event;

        assert!(event.detail.r#type.as_ref().unwrap() == "Normal");
        assert!(event.detail.date.as_ref().unwrap() == "31 DEC 1965");

        let place = event.detail.place.as_ref().unwrap();
        assert!(place.name.as_ref().unwrap() == "Salt Lake City, UT, USA");
        assert!(place.note.as_ref().unwrap().note.as_ref().unwrap() == "Place note");

        let place_phonetic = place.phonetic.as_ref().unwrap();
        assert!(place_phonetic.name.as_ref().unwrap() == "Salt Lake City, UT, USA");
        assert!(place_phonetic.r#type.as_ref().unwrap() == "user defined");
        let place_roman = place.roman.as_ref().unwrap();
        assert!(place_roman.name.as_ref().unwrap() == "Salt Lake City, UT, USA");
        assert!(place_roman.r#type.as_ref().unwrap() == "user defined");
        let place_map = place.map.as_ref().unwrap();
        assert!(place_map.latitude == 0.0);
        assert!(place_map.longitude == 0.0);

        let addr = event.detail.address.as_ref().unwrap();
        assert!(addr.addr1.as_ref().unwrap() == "St. Marks Hospital");
        assert!(addr.city.as_ref().unwrap() == "Salt Lake City");
        assert!(addr.state.as_ref().unwrap() == "UT");
        assert!(addr.postal_code.as_ref().unwrap() == "84121");
        assert!(addr.country.as_ref().unwrap() == "USA");

        assert!(event.detail.agency.as_ref().unwrap() == "none");
        assert!(event.detail.religion.as_ref().unwrap() == "Religion");
        assert!(event.detail.cause.as_ref().unwrap() == "Conception");

        // Good to know: notes can be an xref that refer to a top-level note,
        // i.e, @N8@ -> '0 NOTE @N8@'.
        // I need to write some kind of resolver
        // TODO: Convert to a Note (and add xref to Note)
        assert!(event.detail.note.as_ref().unwrap() == "@N8@");

        let source = event.detail.sources.first().unwrap();
        assert!(source.xref.as_ref().unwrap() == "@S1@");
        assert!(source.page.as_ref().unwrap() == &42);

        let sdata = source.data.as_ref().unwrap();
        assert!(sdata.date.as_ref().unwrap() == "1 JAN 1900");
        assert!(sdata.text.as_ref().unwrap().note.as_ref().unwrap() == "Here is some text from the source specific to this source citation.\nHere is more text but on a new line.");

        let sevent = source.event.as_ref().unwrap();
        assert!(sevent.role.as_ref().unwrap() == "CHIL");
        assert!(sevent.r#type.as_ref().unwrap() == "BIRT");

        assert!(source.media.len() == 1);
        let media = source.media.first().unwrap();
        assert!(media.xref == Some("@M8@".to_string()));

        assert!(source.note.as_ref().unwrap().note.as_ref().unwrap() == "Some notes about this birth source citation which are embedded in the citation structure itself.");

        assert!(source.quay.as_ref().unwrap() == &Quay::Secondary);

        let obje = event.detail.media.first().unwrap();
        assert!(obje.xref == Some("@M15@".to_string()));

        assert!(event.age.as_ref().unwrap() == "0y");

        assert!(birth.family.as_ref().unwrap().xref == "@F2@");

        // Death
        // "1 DEAT",
        let death = indi.death.first().unwrap();

        let devent = death.event.as_ref().unwrap();
        // "2 DATE ABT 15 JAN 2001",
        assert!(devent.date.is_some());
        assert!(devent.date.as_ref().unwrap() == "ABT 15 JAN 2001");

        // "2 PLAC New York, New York, USA",
        // "3 NOTE The place structure has more detail than usually used for places",
        // "2 AGE 76y",
        assert!(death.age.as_ref().unwrap() == "76y");
        // "2 TYPE slow",
        assert!(devent.r#type.as_ref().unwrap() == "slow");

        // "2 ADDR",
        // "3 ADR1 at Home",
        assert!(devent.address.is_some());
        let addr = devent.address.as_ref().unwrap();
        assert!(addr.addr1.as_ref().unwrap() == "at Home");

        // "2 CAUS Cancer",
        assert!(devent.cause.as_ref().unwrap() == "Cancer");

        // "2 AGNC none",
        assert!(devent.agency.as_ref().unwrap() == "none");

        // "2 OBJE @M8@",
        assert!(devent.media.len() == 1);
        let obj = devent.media.first().unwrap();
        assert!(obj.xref == Some("@M8@".to_string()));

        // "2 SOUR @S1@",
        assert!(devent.sources.len() == 1);
        let source = devent.sources.first().unwrap();
        assert!(source.xref.as_ref().unwrap() == "@S1@");

        // "3 PAGE 42",
        assert!(source.page.as_ref().unwrap() == &42);

        // "3 DATA",
        let sdata = source.data.as_ref().unwrap();

        // "4 DATE 31 DEC 1900",
        assert!(sdata.date.as_ref().unwrap() == "31 DEC 1900");

        // "4 TEXT Some death source text.",
        assert!(sdata.text.as_ref().unwrap().note.as_ref().unwrap() == "Some death source text.");

        // "3 QUAY 3",
        assert!(source.quay.as_ref().unwrap() == &Quay::Direct);

        // "3 NOTE A death source note.",
        assert!(source.note.as_ref().unwrap().note.as_ref().unwrap() == "A death source note.");

        // "2 NOTE A death event note.",
        assert!(devent.note.as_ref().unwrap() == "A death event note.");

        // Family links
        // FAMS
        assert!(indi.fams.len() == 2);

        // FAMC
        assert!(indi.famc.len() == 2);

        // Baptism
        // "1 BAPM",
        let bapm = indi.baptism.as_mut().unwrap().pop().unwrap();

        // "2 DATE ABT 31 DEC 1997",
        assert!(bapm.detail.date.unwrap() == "ABT 31 DEC 1997");

        // "2 PLAC The place",
        assert!(bapm.detail.place.unwrap().name.unwrap() == "The place");

        // "2 AGE 3m",
        assert!(bapm.age.unwrap() == "3m");

        // "2 TYPE BAPM",
        assert!(bapm.detail.r#type.unwrap() == "BAPM");

        // "2 ADDR",
        let addr = bapm.detail.address.unwrap();

        // "3 ADR1 Church Name",
        assert!(addr.addr1.unwrap() == "Church Name");

        // "3 ADR2 Street Address",
        assert!(addr.addr2.unwrap() == "Street Address");

        // "3 CITY City Name",
        assert!(addr.city.unwrap() == "City Name");

        // "3 POST zip",
        assert!(addr.postal_code.unwrap() == "zip");

        // "3 CTRY Country",
        assert!(addr.country.unwrap() == "Country");

        // "2 CAUS Birth",
        assert!(bapm.detail.cause.unwrap() == "Birth");

        // "2 AGNC The Church",
        assert!(bapm.detail.agency.unwrap() == "The Church");

        // "2 OBJE @M8@",
        let media = bapm.detail.media;
        assert!(media[0].xref == Some("@M8@".to_string()));

        // Sources
        let mut sources = bapm.detail.sources;
        let source = sources.pop().unwrap();

        // "2 SOUR @S1@",
        assert!(source.xref.unwrap() == "@S1@".to_string());

        // "3 PAGE 42",
        assert!(source.page.unwrap() == 42);

        // "3 DATA",
        let sdata = source.data.unwrap();

        // "4 DATE 31 DEC 1900",
        assert!(sdata.date.unwrap() == "31 DEC 1900");

        // "4 TEXT Sample baptism Source text.",
        assert!(sdata.text.unwrap().note.unwrap() == "Sample baptism Source text.");

        // "3 QUAY 3",
        assert!(source.quay.as_ref().unwrap() == &Quay::Direct);

        // "3 NOTE A baptism source note.",
        assert!(source.note.unwrap().note.unwrap() == "A baptism source note.");

        // "2 NOTE A baptism event note (the event of baptism (not LDS), performed in infancy or later. See also BAPL and CHR).",
        assert!(bapm
            .detail
            .note
            .unwrap()
            .starts_with("A baptism event note"));

        // Christening

        // "1 CHR",
        let chr = indi.christening.first().unwrap();

        // "2 DATE CAL 31 DEC 1997",
        assert!(chr.event.detail.date.as_ref().unwrap() == "CAL 31 DEC 1997");

        // "2 PLAC The place",
        assert!(
            chr.event
                .detail
                .place
                .as_ref()
                .unwrap()
                .name
                .as_ref()
                .unwrap()
                == "The place"
        );

        // "2 TYPE CHR",
        assert!(chr.event.detail.r#type.as_ref().unwrap() == "CHR");

        let source = chr.event.detail.sources.first().unwrap();

        // "2 SOUR @S1@",
        assert!(source.xref.as_ref().unwrap() == "@S1@");

        // "3 PAGE 42",
        assert!(source.page.unwrap() == 42);

        // "3 DATA",
        let data = source.data.as_ref().unwrap();
        // "4 DATE 31 DEC 1900",
        assert!(data.date.as_ref().unwrap() == "31 DEC 1900");

        // "4 TEXT Sample CHR Source text.",
        assert!(data.text.as_ref().unwrap().note.as_ref().unwrap() == "Sample CHR Source text.");

        // "3 QUAY 3",
        assert!(source.quay.as_ref().unwrap() == &Quay::Direct);

        // "3 NOTE A christening Source note.",
        assert!(
            source.note.as_ref().unwrap().note.as_ref().unwrap() == "A christening Source note."
        );

        // "2 NOTE Christening event note (the religious event (not LDS) of baptizing and/or naming a ",
        // "3 CONC child).",
        assert!(chr.event.detail.note.as_ref().unwrap() == "Christening event note (the religious event (not LDS) of baptizing and/or naming a child).");

        // "2 FAMC @F3@",
        assert!(chr.family.as_ref().unwrap().xref == "@F3@".to_string());

        // "1 BARM",
        let barm = indi.barmitzvah.as_ref().unwrap().first().unwrap();
        // "2 DATE AFT 31 DEC 1997",
        assert!(barm.detail.date.as_ref().unwrap() == "AFT 31 DEC 1997");

        // "2 PLAC The place",
        assert!(barm.detail.place.as_ref().unwrap().name.as_ref().unwrap() == "The place");

        // "2 TYPE BARM",
        assert!(barm.detail.r#type.as_ref().unwrap() == "BARM");

        let source = barm.detail.sources.first().unwrap();
        // "2 SOUR @S1@",
        assert!(source.xref.as_ref().unwrap() == "@S1@");
        // "3 PAGE 42",
        assert!(source.page.unwrap() == 42);

        // "3 DATA",
        let sdata = source.data.as_ref().unwrap();

        // "4 DATE 31 DEC 1900",
        assert!(sdata.date.as_ref().unwrap() == "31 DEC 1900");

        // "4 TEXT Some Bar Mitzvah source text.",
        assert!(
            sdata.text.as_ref().unwrap().note.as_ref().unwrap() == "Some Bar Mitzvah source text."
        );

        // "3 QUAY 3",
        assert!(source.quay.as_ref().unwrap() == &Quay::Direct);

        // "3 NOTE A Bar Mitzvah source note.",
        assert!(
            source.note.as_ref().unwrap().note.as_ref().unwrap() == "A Bar Mitzvah source note."
        );

        // "2 NOTE Bar Mitzvah event note (the ceremonial event held when a Jewish boy reaches age ",
        // "3 CONC 13).",
        assert!(barm.detail.note.as_ref().unwrap() == "Bar Mitzvah event note (the ceremonial event held when a Jewish boy reaches age 13).");

        // Baz Mitzvah
        // "1 BASM",
        let basm = indi.basmitzvah.as_ref().unwrap().first().unwrap();

        // "2 DATE AFT 31 DEC 1997",
        assert!(basm.detail.date.as_ref().unwrap() == "AFT 31 DEC 1997");

        // "2 PLAC The place",
        assert!(basm.detail.place.as_ref().unwrap().name.as_ref().unwrap() == "The place");

        // "2 TYPE BARM",
        assert!(basm.detail.r#type.as_ref().unwrap() == "BASM");

        let source = basm.detail.sources.first().unwrap();
        // "2 SOUR @S1@",
        assert!(source.xref.as_ref().unwrap() == "@S1@");
        // "3 PAGE 42",
        assert!(source.page.unwrap() == 42);

        // "3 DATA",
        let sdata = source.data.as_ref().unwrap();

        // "4 DATE 31 DEC 1900",
        assert!(sdata.date.as_ref().unwrap() == "31 DEC 1900");

        // "4 TEXT Some Bar Mitzvah source text.",
        assert!(
            sdata.text.as_ref().unwrap().note.as_ref().unwrap() == "Some Bas Mitzvah source text."
        );

        // "3 QUAY 3",
        assert!(source.quay.as_ref().unwrap() == &Quay::Direct);

        // "3 NOTE A Bar Mitzvah source note.",
        assert!(
            source.note.as_ref().unwrap().note.as_ref().unwrap() == "A Bas Mitzvah source note."
        );

        // "2 NOTE Bas Mitzvah event note (the ceremonial event held when a Jewish girl reaches age 13, ",
        // "3 CONC also known as \"Bat Mitzvah\").",
        assert!(basm.detail.note.as_ref().unwrap() == "Bas Mitzvah event note (the ceremonial event held when a Jewish girl reaches age 13, also known as \"Bat Mitzvah\").");

        // "1 ADOP",
        let adoption = indi.adoption.as_ref().unwrap().first().unwrap();

        // "2 DATE BEF 31 DEC 1997",
        assert!(adoption.event.detail.date.as_ref().unwrap() == "BEF 31 DEC 1997");

        // "2 PLAC The place",
        assert!(
            adoption
                .event
                .detail
                .place
                .as_ref()
                .unwrap()
                .name
                .as_ref()
                .unwrap()
                == "The place"
        );

        // "2 TYPE ADOP",
        assert!(adoption.event.detail.r#type.as_ref().unwrap() == "ADOP");

        // "2 SOUR @S1@",
        let source = adoption.event.detail.sources.first().unwrap();
        assert!(source.xref.as_ref().unwrap() == "@S1@");

        // "3 PAGE 42",
        assert!(source.page.unwrap() == 42);

        // "3 DATA",
        let sdata = source.data.as_ref().unwrap();

        // "4 DATE 31 DEC 1900",
        assert!(sdata.date.as_ref().unwrap() == "31 DEC 1900");

        // "4 TEXT Some adoption source text.",
        assert!(
            sdata.text.as_ref().unwrap().note.as_ref().unwrap() == "Some adoption source text."
        );

        // "3 QUAY 3",
        assert!(source.quay.as_ref().unwrap() == &Quay::Direct);

        // "3 NOTE An adoption source note.",
        assert!(source.note.as_ref().unwrap().note.as_ref().unwrap() == "An adoption source note.");

        // "2 NOTE Adoption event note (pertaining to creation of a child-parent relationship that does ",
        // "3 CONC not exist biologically).",
        assert!(adoption.event.detail.note.as_ref().unwrap() == "Adoption event note (pertaining to creation of a child-parent relationship that does not exist biologically).");

        // "2 FAMC @F3@",
        let family = adoption.family.as_ref().unwrap();
        assert!(family.xref == "@F3@");
        // "3 ADOP BOTH",
        assert!(family.adopted_by.is_some());
        assert!(family.adopted_by.as_ref().unwrap() == &AdoptedBy::Both);

        // Adult Christening
        // "1 CHRA",
        let chr = indi.christening_adult.as_ref().unwrap().first().unwrap();

        // "2 DATE BET 31 DEC 1997 AND 1 FEB 1998",
        assert!(chr.event.detail.date.as_ref().unwrap() == "BET 31 DEC 1997 AND 1 FEB 1998");

        // "2 PLAC The place",
        assert!(
            chr.event
                .detail
                .place
                .as_ref()
                .unwrap()
                .name
                .as_ref()
                .unwrap()
                == "The place"
        );
        // "2 TYPE CHRA",
        assert!(chr.event.detail.r#type.as_ref().unwrap() == "CHRA");

        let source = chr.event.detail.sources.first().unwrap();
        // "2 SOUR @S1@",
        assert!(source.xref.as_ref().unwrap() == "@S1@");

        // "3 PAGE 42",
        assert!(source.page.unwrap() == 42);

        // "3 DATA",
        let data = source.data.as_ref().unwrap();
        // "4 DATE 31 DEC 1900",
        assert!(data.date.as_ref().unwrap() == "31 DEC 1900");
        // "4 TEXT Some christening source text.",
        assert!(
            data.text.as_ref().unwrap().note.as_ref().unwrap() == "Some christening source text."
        );
        // "3 QUAY 3",
        assert!(source.quay.as_ref().unwrap() == &Quay::Direct);
        // "3 NOTE A christening source note.",
        assert!(
            source.note.as_ref().unwrap().note.as_ref().unwrap() == "A christening source note."
        );

        // "2 NOTE Adult christening event note (the religious event (not LDS) of baptizing and/or ",
        // "3 CONC naming an adult person).",
        assert!(chr.event.detail.note.as_ref().unwrap() == "Adult christening event note (the religious event (not LDS) of baptizing and/or naming an adult person).");

        // CONFIRMATION
        // "1 CONF",
        assert!(indi.confirmation.as_ref().unwrap().len() == 1);
        let confirmation = indi.confirmation.as_ref().unwrap().first().unwrap();

        // "2 DATE BET 31 DEC 1997 AND 2 JAN 1998",
        assert!(confirmation.detail.date.as_ref().unwrap() == "BET 31 DEC 1997 AND 2 JAN 1998");

        // "2 PLAC The place",
        assert!(
            confirmation
                .detail
                .place
                .as_ref()
                .unwrap()
                .name
                .as_ref()
                .unwrap()
                == "The place"
        );

        // "2 TYPE CONF",
        assert!(confirmation.detail.r#type.as_ref().unwrap() == "CONF");

        let source = confirmation.detail.sources.first().unwrap();

        // "2 SOUR @S1@",
        assert!(source.xref.as_ref().unwrap() == "@S1@");

        // "3 PAGE 42",
        assert!(source.page.unwrap() == 42);

        // "3 DATA",
        let sdata = source.data.as_ref().unwrap();

        // "4 DATE 31 DEC 1900",
        assert!(sdata.date.as_ref().unwrap() == "31 DEC 1900");

        // "4 TEXT Some CONF Source text.",
        assert!(sdata.text.as_ref().unwrap().note.as_ref().unwrap() == "Some CONF Source text.");

        // "3 QUAY 3",
        assert!(source.quay.as_ref().unwrap() == &Quay::Direct);

        // "3 NOTE A CONF Source note.",
        assert!(source.note.as_ref().unwrap().note.as_ref().unwrap() == "A CONF Source note.");

        // "2 NOTE CONFIRMATION event note (the religious event (not LDS) of conferring the gift of the Holy Ghost and, among protestants, full church membership).",
        assert!(confirmation.detail.note.as_ref().unwrap() == "CONFIRMATION event note (the religious event (not LDS) of conferring the gift of the Holy Ghost and, among protestants, full church membership).");

        // First Communion
        assert!(indi.first_communion.is_some());
    }

    #[test]
    fn test_individual_record_level_notes() {
        let data = vec![
            "0 @I1@ INDI",
            "1 NAME Test /Person/",
            "1 NOTE This is a record-level note",
            "1 NOTE @N1@",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.notes.len(), 2);
    }

    #[test]
    fn test_individual_source_citations() {
        let data = vec![
            "0 @I1@ INDI",
            "1 NAME Test /Person/",
            "1 SOUR @S1@",
            "2 PAGE 42",
            "1 SOUR @S2@",
            "2 PAGE 84",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.source_citations.len(), 2);
        assert_eq!(indi.source_citations[0].xref.as_ref().unwrap(), "@S1@");
        assert_eq!(indi.source_citations[0].page.unwrap(), 42);
    }

    #[test]
    fn test_individual_multimedia_links() {
        let data = vec![
            "0 @I1@ INDI",
            "1 NAME Test /Person/",
            "1 OBJE @M1@",
            "1 OBJE @M2@",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.multimedia_links.len(), 2);
    }

    #[test]
    fn test_individual_user_references() {
        let data = vec![
            "0 @I1@ INDI",
            "1 NAME Test /Person/",
            "1 REFN 12345",
            "2 TYPE user_id",
            "1 REFN ABC-123",
            "2 TYPE custom",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.user_reference_numbers.len(), 2);
        assert_eq!(indi.user_reference_numbers[0].number, "12345");
        assert_eq!(
            indi.user_reference_numbers[0].ref_type.as_ref().unwrap(),
            "user_id"
        );
    }

    #[test]
    fn test_individual_automated_record_id() {
        let data = vec!["0 @I1@ INDI", "1 NAME Test /Person/", "1 RIN 54321"];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.automated_record_id.as_ref().unwrap(), "54321");
    }

    #[test]
    fn test_individual_change_date() {
        let data = vec![
            "0 @I1@ INDI",
            "1 NAME Test /Person/",
            "1 CHAN",
            "2 DATE 15 JAN 2024",
            "3 TIME 14:30:00",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.change_date.as_ref().unwrap(), "15 JAN 2024");
    }

    #[test]
    fn test_individual_restriction() {
        let data = vec!["0 @I1@ INDI", "1 NAME Test /Person/", "1 RESN confidential"];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.restriction.as_ref().unwrap(), "confidential");
    }

    #[test]
    fn test_individual_submitters() {
        let data = vec![
            "0 @I1@ INDI",
            "1 NAME Test /Person/",
            "1 SUBM @U1@",
            "1 SUBM @U2@",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.submitters.len(), 2);
        assert_eq!(indi.submitters[0].as_str(), "@U1@");
    }

    #[test]
    fn test_individual_associations() {
        let data = vec![
            "0 @I1@ INDI",
            "1 NAME Test /Person/",
            "1 ASSO @I2@",
            "2 RELA Godfather",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.associations.len(), 1);
        assert_eq!(indi.associations[0].xref.as_str(), "@I2@");
        assert_eq!(indi.associations[0].relation.as_ref().unwrap(), "Godfather");
    }

    #[test]
    fn test_individual_alias() {
        let data = vec!["0 @I1@ INDI", "1 NAME Test /Person/", "1 ALIA @I2@"];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.alias.len(), 1);
        assert_eq!(indi.alias[0].as_str(), "@I2@");
    }

    #[test]
    fn test_individual_ancestor_interest() {
        let data = vec!["0 @I1@ INDI", "1 NAME Test /Person/", "1 ANCI @U1@"];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.ancestor_interest.len(), 1);
        assert_eq!(indi.ancestor_interest[0].as_str(), "@U1@");
    }

    #[test]
    fn test_individual_descendant_interest() {
        let data = vec!["0 @I1@ INDI", "1 NAME Test /Person/", "1 DESI @U1@"];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.descendant_interest.len(), 1);
        assert_eq!(indi.descendant_interest[0].as_str(), "@U1@");
    }

    #[test]
    fn test_individual_permanent_record_file_number() {
        let data = vec!["0 @I1@ INDI", "1 NAME Test /Person/", "1 RFN AF12345"];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(
            indi.permanent_record_file_number.as_ref().unwrap(),
            "AF12345"
        );
    }

    #[test]
    fn test_individual_ancestral_file_number() {
        let data = vec!["0 @I1@ INDI", "1 NAME Test /Person/", "1 AFN 1234-5678"];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        assert_eq!(indi.ancestral_file_number.as_ref().unwrap(), "1234-5678");
    }

    #[test]
    fn test_individual_attributes() {
        let data = vec![
            "0 @I1@ INDI",
            "1 NAME Test /Person/",
            "1 EDUC Bachelor of Science",
            "2 DATE 2010",
            "1 DSCR Tall with brown hair",
            "1 RELI Catholic",
            "1 IDNO 123-45-6789",
            "2 TYPE SSN",
            "1 PROP House and land",
            "1 CAST Brahmin",
            "1 NCHI 3",
            "1 NMR 2",
            "1 TITL Duke",
            "1 NATI American",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let indi = Individual::parse(&mut record);

        // Education
        assert!(indi.education.is_some());
        let education = indi.education.as_ref().unwrap();
        assert_eq!(education.len(), 1);
        assert_eq!(education[0].value, "Bachelor of Science");
        assert_eq!(education[0].date.as_ref().unwrap(), "2010");

        // Physical description
        assert!(indi.physical_description.is_some());
        assert_eq!(
            indi.physical_description.as_ref().unwrap()[0].value,
            "Tall with brown hair"
        );

        // Religion
        assert!(indi.religion.is_some());
        assert_eq!(indi.religion.as_ref().unwrap()[0].value, "Catholic");

        // National ID
        assert!(indi.national_id_number.is_some());
        let natid = &indi.national_id_number.as_ref().unwrap()[0];
        assert_eq!(natid.value, "123-45-6789");
        assert_eq!(natid.attribute_type.as_ref().unwrap(), "SSN");

        // Property
        assert!(indi.property.is_some());
        assert_eq!(indi.property.as_ref().unwrap()[0].value, "House and land");

        // Caste
        assert!(indi.caste.is_some());
        assert_eq!(indi.caste.as_ref().unwrap()[0].value, "Brahmin");

        // Number of children
        assert!(indi.number_of_children.is_some());
        assert_eq!(indi.number_of_children.as_ref().unwrap()[0].value, "3");

        // Number of marriages
        assert!(indi.number_of_marriages.is_some());
        assert_eq!(indi.number_of_marriages.as_ref().unwrap()[0].value, "2");

        // Nobility title
        assert!(indi.nobility_title.is_some());
        assert_eq!(indi.nobility_title.as_ref().unwrap()[0].value, "Duke");

        // National origin
        assert!(indi.national_origin.is_some());
        assert_eq!(indi.national_origin.as_ref().unwrap()[0].value, "American");
    }

    #[test]
    fn test_association_parse() {
        let data = vec![
            "1 ASSO @I2@",
            "2 RELA Godfather",
            "2 NOTE Association note",
            "2 SOUR @S1@",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let assoc = Association::parse(&mut record).unwrap();

        assert_eq!(assoc.xref.as_str(), "@I2@");
        assert_eq!(assoc.relation.as_ref().unwrap(), "Godfather");
        assert_eq!(assoc.notes.len(), 1);
        assert_eq!(assoc.source_citations.len(), 1);
    }

    #[test]
    fn test_individual_attribute_parse() {
        let data = vec![
            "1 EDUC PhD in Computer Science",
            "2 TYPE Doctorate",
            "2 DATE 2015",
            "2 PLAC Stanford University",
            "2 NOTE Educational achievement",
            "2 SOUR @S1@",
        ];

        let buffer = data.join("\n");
        let mut record = buffer.as_str();
        let attr = IndividualAttribute::parse(&mut record, "PhD in Computer Science").unwrap();

        assert_eq!(attr.value, "PhD in Computer Science");
        assert_eq!(attr.attribute_type.as_ref().unwrap(), "Doctorate");
        assert_eq!(attr.date.as_ref().unwrap(), "2015");
        assert_eq!(attr.place.as_ref().unwrap(), "Stanford University");
        assert_eq!(attr.notes.len(), 1);
        assert_eq!(attr.source_citations.len(), 1);
    }
}
