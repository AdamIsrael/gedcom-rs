/// mod.rs
// top-level record types
mod address;
mod adopted_by;
mod change_date;
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
pub use change_date::ChangeDate;
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

/// Describes the genealogical relationship between two individuals
#[derive(Debug, Clone)]
pub struct RelationshipResult<'a> {
    /// Human-readable description of the relationship
    /// Examples: "Parent", "Child", "Sibling", "First Cousin", "Second Cousin Once Removed"
    pub description: String,

    /// Most Recent Common Ancestor(s) - the shared ancestor(s) closest to the individuals
    /// For siblings, this would be their parents
    /// For first cousins, this would be their grandparents
    /// Can be empty if no common ancestor is found (not related)
    pub mrca: Vec<&'a Individual>,

    /// Distance from person1 to MRCA (generations up)
    pub generations_to_mrca_1: Option<usize>,

    /// Distance from person2 to MRCA (generations up)
    pub generations_to_mrca_2: Option<usize>,
}

impl<'a> RelationshipResult<'a> {
    /// Create a new relationship result with no relationship found
    pub fn none() -> Self {
        RelationshipResult {
            description: "Not related".to_string(),
            mrca: Vec::new(),
            generations_to_mrca_1: None,
            generations_to_mrca_2: None,
        }
    }

    /// Create a new relationship result for self (same person)
    pub fn self_relation() -> Self {
        RelationshipResult {
            description: "Self".to_string(),
            mrca: Vec::new(),
            generations_to_mrca_1: Some(0),
            generations_to_mrca_2: Some(0),
        }
    }
}

/// Event types that can be searched for individuals
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    /// Birth event
    Birth,
    /// Death event
    Death,
    /// Christening event
    Christening,
    /// Adult christening event
    ChristeningAdult,
    /// Baptism event
    Baptism,
    /// Bar Mitzvah event
    BarMitzvah,
    /// Bas Mitzvah event
    BasMitzvah,
    /// Blessing event
    Blessing,
    /// Burial event
    Burial,
    /// Census event
    Census,
    /// Confirmation event
    Confirmation,
    /// First Communion event
    FirstCommunion,
    /// Cremation event
    Cremation,
    /// Adoption event
    Adoption,
    /// Emigration event
    Emigration,
    /// Graduation event
    Graduation,
    /// Immigration event
    Immigration,
    /// Naturalization event
    Naturalization,
    /// Probate event
    Probate,
    /// Retirement event
    Retirement,
    /// Will event
    Will,
}

#[derive(Debug, Default)]
pub struct Gedcom {
    pub header: Header,
    pub individuals: Vec<Individual>,
    pub families: Vec<Family>,
    pub sources: Vec<SourceRecord>,
    pub repositories: Vec<RepositoryRecord>,
    pub notes: Vec<NoteRecord>,
    pub multimedia: Vec<MultimediaRecord>,
    pub submitters: Vec<Submitter>,

    /// Validation warnings encountered during parsing
    /// These are non-fatal issues that don't prevent parsing but indicate
    /// potential problems with the GEDCOM file
    pub warnings: Vec<crate::error::GedcomError>,
}

impl Gedcom {
    // ===== Validation Functions =====

    /// Returns whether the GEDCOM file has any validation warnings
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if gedcom.has_warnings() {
    ///     println!("File has {} validation warnings", gedcom.warnings.len());
    ///     for warning in &gedcom.warnings {
    ///         eprintln!("Warning: {}", warning);
    ///     }
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    // ===== Search Functions =====

    /// Find an individual by their cross-reference ID (xref)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(person) = gedcom.find_individual_by_xref("@I1@") {
    ///     println!("Found person: {:?}", person.names);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn find_individual_by_xref(&self, xref: &str) -> Option<&Individual> {
        self.individuals
            .iter()
            .find(|indi| indi.xref.as_ref().map(|x| x.as_str()) == Some(xref))
    }

    /// Find individuals by name (partial or full match, case-insensitive)
    ///
    /// Returns all individuals whose name contains the search string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// let results = gedcom.find_individuals_by_name("Smith");
    /// for person in results {
    ///     println!("Found: {:?}", person.names);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn find_individuals_by_name(&self, name: &str) -> Vec<&Individual> {
        let search_lower = name.to_lowercase();
        self.individuals
            .iter()
            .filter(|indi| {
                indi.names.iter().any(|n| {
                    n.name
                        .value
                        .as_ref()
                        .map(|v| v.to_lowercase().contains(&search_lower))
                        .unwrap_or(false)
                })
            })
            .collect()
    }

    /// Find a family record by its cross-reference ID (xref)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(family) = gedcom.find_family_by_xref("@F1@") {
    ///     println!("Found family with {} children", family.children.len());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn find_family_by_xref(&self, xref: &str) -> Option<&Family> {
        self.families.iter().find(|fam| fam.xref.as_str() == xref)
    }

    /// Find individuals by event date
    ///
    /// Returns all individuals who have an event of the specified type
    /// that contains the given date string (partial match, case-insensitive).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    /// use gedcom_rs::types::EventType;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// // Find all individuals born in 1965
    /// let results = gedcom.find_individuals_by_event_date(EventType::Birth, "1965");
    /// for person in results {
    ///     println!("Found: {:?}", person.names);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn find_individuals_by_event_date(
        &self,
        event_type: EventType,
        date_pattern: &str,
    ) -> Vec<&Individual> {
        let pattern_lower = date_pattern.to_lowercase();

        self.individuals
            .iter()
            .filter(|individual| {
                let has_matching_date =
                    match event_type {
                        EventType::Birth => individual.birth.iter().any(|e| {
                            Self::event_date_matches(&e.event.detail.date, &pattern_lower)
                        }),
                        EventType::Death => individual.death.iter().any(|e| {
                            Self::event_date_matches(
                                &e.event.as_ref().and_then(|ev| ev.date.clone()),
                                &pattern_lower,
                            )
                        }),
                        EventType::Christening => individual.christening.iter().any(|e| {
                            Self::event_date_matches(&e.event.detail.date, &pattern_lower)
                        }),
                        EventType::ChristeningAdult => individual
                            .christening_adult
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.event.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Baptism => individual
                            .baptism
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::BarMitzvah => individual
                            .barmitzvah
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::BasMitzvah => individual
                            .basmitzvah
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Blessing => individual
                            .blessing
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Burial => individual
                            .burial
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Census => individual
                            .census
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Confirmation => individual
                            .confirmation
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::FirstCommunion => individual
                            .first_communion
                            .as_ref()
                            .map(|e| Self::event_date_matches(&e.detail.date, &pattern_lower))
                            .unwrap_or(false),
                        EventType::Cremation => individual
                            .cremation
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Adoption => individual
                            .adoption
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.event.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Emigration => individual
                            .emigration
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Graduation => individual
                            .graduation
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Immigration => individual
                            .immigration
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Naturalization => individual
                            .naturalization
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Probate => individual
                            .probate
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Retirement => individual
                            .retirement
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                        EventType::Will => individual
                            .will
                            .as_ref()
                            .map(|events| {
                                events.iter().any(|e| {
                                    Self::event_date_matches(&e.detail.date, &pattern_lower)
                                })
                            })
                            .unwrap_or(false),
                    };

                has_matching_date
            })
            .collect()
    }

    /// Helper function to check if an event date matches a pattern
    fn event_date_matches(date_opt: &Option<String>, pattern_lower: &str) -> bool {
        date_opt
            .as_ref()
            .map(|d| d.to_lowercase().contains(pattern_lower))
            .unwrap_or(false)
    }

    // ===== Basic Relationship Functions =====

    /// Get the parents of an individual
    ///
    /// Returns a vector of tuples containing (father, mother) for each family
    /// the individual is a child in. Either parent may be None.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(person) = gedcom.find_individual_by_xref("@I1@") {
    ///     let parents = gedcom.get_parents(person);
    ///     for (father, mother) in parents {
    ///         if let Some(dad) = father {
    ///             println!("Father: {:?}", dad.names);
    ///         }
    ///         if let Some(mom) = mother {
    ///             println!("Mother: {:?}", mom.names);
    ///         }
    ///     }
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_parents(
        &self,
        individual: &Individual,
    ) -> Vec<(Option<&Individual>, Option<&Individual>)> {
        individual
            .famc
            .iter()
            .filter_map(|fam_link| {
                self.find_family_by_xref(fam_link.xref.as_str())
                    .map(|family| {
                        let father = family
                            .husband
                            .as_ref()
                            .and_then(|xref| self.find_individual_by_xref(xref.as_str()));
                        let mother = family
                            .wife
                            .as_ref()
                            .and_then(|xref| self.find_individual_by_xref(xref.as_str()));
                        (father, mother)
                    })
            })
            .collect()
    }

    /// Get the children of an individual
    ///
    /// Returns all children from all families where this individual is a spouse.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(person) = gedcom.find_individual_by_xref("@I1@") {
    ///     let children = gedcom.get_children(person);
    ///     println!("{} has {} children", person.xref.as_ref().unwrap(), children.len());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_children(&self, individual: &Individual) -> Vec<&Individual> {
        individual
            .fams
            .iter()
            .filter_map(|fam_link| self.find_family_by_xref(fam_link.xref.as_str()))
            .flat_map(|family| {
                family
                    .children
                    .iter()
                    .filter_map(|child_xref| self.find_individual_by_xref(child_xref.as_str()))
            })
            .collect()
    }

    /// Get the spouses of an individual
    ///
    /// Returns all spouses from all families where this individual is listed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(person) = gedcom.find_individual_by_xref("@I1@") {
    ///     let spouses = gedcom.get_spouses(person);
    ///     for spouse in spouses {
    ///         println!("Spouse: {:?}", spouse.names);
    ///     }
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_spouses(&self, individual: &Individual) -> Vec<&Individual> {
        let individual_xref = match &individual.xref {
            Some(xref) => xref.as_str(),
            None => return Vec::new(),
        };

        individual
            .fams
            .iter()
            .filter_map(|fam_link| self.find_family_by_xref(fam_link.xref.as_str()))
            .filter_map(|family| {
                // If this individual is the husband, return the wife
                if family.husband.as_ref().map(|x| x.as_str()) == Some(individual_xref) {
                    family
                        .wife
                        .as_ref()
                        .and_then(|xref| self.find_individual_by_xref(xref.as_str()))
                }
                // If this individual is the wife, return the husband
                else if family.wife.as_ref().map(|x| x.as_str()) == Some(individual_xref) {
                    family
                        .husband
                        .as_ref()
                        .and_then(|xref| self.find_individual_by_xref(xref.as_str()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get siblings of an individual
    ///
    /// Returns all siblings (individuals who share at least one parent).
    /// This includes both full siblings and half-siblings, even if they
    /// are in different family records.
    /// Does not include the individual themselves.
    ///
    /// For more specific queries, see:
    /// - `get_full_siblings()` - only full siblings (same mother and father)
    /// - `get_half_siblings()` - only half-siblings (one shared parent)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(person) = gedcom.find_individual_by_xref("@I1@") {
    ///     let siblings = gedcom.get_siblings(person);
    ///     println!("Found {} siblings", siblings.len());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_siblings(&self, individual: &Individual) -> Vec<&Individual> {
        let individual_xref = match &individual.xref {
            Some(xref) => xref.as_str(),
            None => return Vec::new(),
        };

        // Get all parents
        let parents = self.get_parents(individual);
        let mut sibling_xrefs = std::collections::HashSet::new();

        // For each parent, find all their children (who are this individual's siblings)
        for (father, mother) in parents {
            // Get all families where father is the husband
            if let Some(dad) = father {
                if let Some(dad_xref) = &dad.xref {
                    for family in &self.families {
                        if family.husband.as_ref().map(|x| x.as_str()) == Some(dad_xref.as_str()) {
                            for child_xref in &family.children {
                                if child_xref.as_str() != individual_xref {
                                    sibling_xrefs.insert(child_xref.as_str());
                                }
                            }
                        }
                    }
                }
            }

            // Get all families where mother is the wife
            if let Some(mom) = mother {
                if let Some(mom_xref) = &mom.xref {
                    for family in &self.families {
                        if family.wife.as_ref().map(|x| x.as_str()) == Some(mom_xref.as_str()) {
                            for child_xref in &family.children {
                                if child_xref.as_str() != individual_xref {
                                    sibling_xrefs.insert(child_xref.as_str());
                                }
                            }
                        }
                    }
                }
            }
        }

        // Convert xrefs to Individual references
        sibling_xrefs
            .into_iter()
            .filter_map(|xref| self.find_individual_by_xref(xref))
            .collect()
    }

    /// Get full siblings of an individual
    ///
    /// Returns only full siblings (individuals who share both parents).
    /// Does not include the individual themselves.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(person) = gedcom.find_individual_by_xref("@I1@") {
    ///     let full_siblings = gedcom.get_full_siblings(person);
    ///     println!("Found {} full siblings", full_siblings.len());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_full_siblings(&self, individual: &Individual) -> Vec<&Individual> {
        let individual_xref = match &individual.xref {
            Some(xref) => xref.as_str(),
            None => return Vec::new(),
        };

        // Get all families where this individual is a child
        let parent_families: Vec<_> = individual
            .famc
            .iter()
            .filter_map(|fam_link| self.find_family_by_xref(fam_link.xref.as_str()))
            .collect();

        // Collect xrefs of full siblings to deduplicate
        let mut full_sibling_xrefs = std::collections::HashSet::new();

        // If individual has no families or only one family, check that one family
        // Full siblings must come from the same family (same mother and father)
        for family in &parent_families {
            // Get both parents from this family
            let has_both_parents = family.husband.is_some() && family.wife.is_some();

            if !has_both_parents {
                continue;
            }

            for child_xref in &family.children {
                // Exclude self
                if child_xref.as_str() == individual_xref {
                    continue;
                }

                // Check if this sibling appears in ALL the same parent families
                // For full siblings, they must share all parent families
                if let Some(sibling) = self.find_individual_by_xref(child_xref.as_str()) {
                    let sibling_families: Vec<_> = sibling
                        .famc
                        .iter()
                        .filter_map(|fam_link| self.find_family_by_xref(fam_link.xref.as_str()))
                        .collect();

                    // Full siblings share the exact same set of parent families
                    if parent_families.len() == sibling_families.len()
                        && parent_families
                            .iter()
                            .all(|pf| sibling_families.iter().any(|sf| std::ptr::eq(*pf, *sf)))
                    {
                        full_sibling_xrefs.insert(child_xref.as_str());
                    }
                }
            }
        }

        // Convert xrefs back to Individual references
        full_sibling_xrefs
            .into_iter()
            .filter_map(|xref| self.find_individual_by_xref(xref))
            .collect()
    }

    /// Get half-siblings of an individual
    ///
    /// Returns only half-siblings (individuals who share exactly one parent).
    /// Does not include the individual themselves or full siblings.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(person) = gedcom.find_individual_by_xref("@I1@") {
    ///     let half_siblings = gedcom.get_half_siblings(person);
    ///     println!("Found {} half-siblings", half_siblings.len());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_half_siblings(&self, individual: &Individual) -> Vec<&Individual> {
        let all_siblings = self.get_siblings(individual);
        let full_siblings = self.get_full_siblings(individual);

        // Convert full_siblings to a set for efficient lookup
        let full_sibling_xrefs: std::collections::HashSet<_> = full_siblings
            .iter()
            .filter_map(|s| s.xref.as_ref().map(|x| x.as_str()))
            .collect();

        // Filter out full siblings from all siblings
        all_siblings
            .into_iter()
            .filter(|sibling| {
                sibling
                    .xref
                    .as_ref()
                    .map(|x| !full_sibling_xrefs.contains(x.as_str()))
                    .unwrap_or(false)
            })
            .collect()
    }

    // ===== Advanced Relationship Functions =====

    /// Get all ancestors of an individual up to a specified number of generations
    ///
    /// Returns a vector of individuals representing all ancestors. If max_generations
    /// is None, it will traverse all generations until no more ancestors are found.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(person) = gedcom.find_individual_by_xref("@I1@") {
    ///     // Get all ancestors up to 5 generations
    ///     let ancestors = gedcom.get_ancestors(person, Some(5));
    ///     println!("Found {} ancestors", ancestors.len());
    ///     
    ///     // Get ALL ancestors
    ///     let all_ancestors = gedcom.get_ancestors(person, None);
    ///     println!("Found {} total ancestors", all_ancestors.len());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_ancestors(
        &self,
        individual: &Individual,
        max_generations: Option<usize>,
    ) -> Vec<&Individual> {
        use std::collections::{HashSet, VecDeque};

        let mut ancestors = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Add the individual's xref to visited to avoid cycles
        if let Some(xref) = &individual.xref {
            visited.insert(xref.as_str());
        }

        // Queue: (individual, generation)
        queue.push_back((individual, 0));

        while let Some((current, generation)) = queue.pop_front() {
            // Check if we've reached the max generation limit
            if let Some(max_gen) = max_generations {
                if generation >= max_gen {
                    continue;
                }
            }

            // Get parents
            let parents = self.get_parents(current);
            for (father, mother) in parents {
                if let Some(dad) = father {
                    if let Some(dad_xref) = &dad.xref {
                        if visited.insert(dad_xref.as_str()) {
                            ancestors.push(dad);
                            queue.push_back((dad, generation + 1));
                        }
                    }
                }
                if let Some(mom) = mother {
                    if let Some(mom_xref) = &mom.xref {
                        if visited.insert(mom_xref.as_str()) {
                            ancestors.push(mom);
                            queue.push_back((mom, generation + 1));
                        }
                    }
                }
            }
        }

        ancestors
    }

    /// Get all descendants of an individual up to a specified number of generations
    ///
    /// Returns a vector of individuals representing all descendants. If max_generations
    /// is None, it will traverse all generations until no more descendants are found.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let Some(person) = gedcom.find_individual_by_xref("@I1@") {
    ///     // Get all descendants up to 3 generations
    ///     let descendants = gedcom.get_descendants(person, Some(3));
    ///     println!("Found {} descendants", descendants.len());
    ///     
    ///     // Get ALL descendants
    ///     let all_descendants = gedcom.get_descendants(person, None);
    ///     println!("Found {} total descendants", all_descendants.len());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_descendants(
        &self,
        individual: &Individual,
        max_generations: Option<usize>,
    ) -> Vec<&Individual> {
        use std::collections::{HashSet, VecDeque};

        let mut descendants = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Add the individual's xref to visited to avoid cycles
        if let Some(xref) = &individual.xref {
            visited.insert(xref.as_str());
        }

        // Queue: (individual, generation)
        queue.push_back((individual, 0));

        while let Some((current, generation)) = queue.pop_front() {
            // Check if we've reached the max generation limit
            if let Some(max_gen) = max_generations {
                if generation >= max_gen {
                    continue;
                }
            }

            // Get children
            let children = self.get_children(current);
            for child in children {
                if let Some(child_xref) = &child.xref {
                    if visited.insert(child_xref.as_str()) {
                        descendants.push(child);
                        queue.push_back((child, generation + 1));
                    }
                }
            }
        }

        descendants
    }

    /// Find the relationship path between two individuals
    ///
    /// Returns the shortest path between two individuals, or None if they are not related.
    /// The path includes both individuals and all connecting individuals.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let (Some(person1), Some(person2)) = (
    ///     gedcom.find_individual_by_xref("@I1@"),
    ///     gedcom.find_individual_by_xref("@I2@")
    /// ) {
    ///     if let Some(path) = gedcom.find_relationship_path(person1, person2) {
    ///         println!("Relationship path has {} people", path.len());
    ///         for person in path {
    ///             println!("  - {:?}", person.names);
    ///         }
    ///     }
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn find_relationship_path<'a>(
        &'a self,
        from: &'a Individual,
        to: &'a Individual,
    ) -> Option<Vec<&'a Individual>> {
        use std::collections::{HashMap, HashSet, VecDeque};

        let from_xref = from.xref.as_ref()?.as_str();
        let to_xref = to.xref.as_ref()?.as_str();

        if from_xref == to_xref {
            return Some(vec![from]);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent_map: HashMap<&str, &str> = HashMap::new();

        queue.push_back(from);
        visited.insert(from_xref);

        while let Some(current) = queue.pop_front() {
            let current_xref = current.xref.as_ref()?.as_str();

            // Get all related individuals (parents, children, spouses, siblings)
            let mut related = Vec::new();

            // Add parents
            for (father, mother) in self.get_parents(current) {
                if let Some(dad) = father {
                    related.push(dad);
                }
                if let Some(mom) = mother {
                    related.push(mom);
                }
            }

            // Add children
            related.extend(self.get_children(current));

            // Add spouses
            related.extend(self.get_spouses(current));

            // Add siblings
            related.extend(self.get_siblings(current));

            for person in related {
                let person_xref = person.xref.as_ref()?.as_str();

                if !visited.contains(person_xref) {
                    visited.insert(person_xref);
                    parent_map.insert(person_xref, current_xref);
                    queue.push_back(person);

                    // Found the target
                    if person_xref == to_xref {
                        // Reconstruct path
                        let mut path = vec![to];
                        let mut current_key = to_xref;

                        while let Some(&parent_key) = parent_map.get(current_key) {
                            if let Some(parent_person) = self.find_individual_by_xref(parent_key) {
                                path.push(parent_person);
                            }
                            current_key = parent_key;
                        }

                        path.reverse();
                        return Some(path);
                    }
                }
            }
        }

        None
    }

    /// Determine the genealogical relationship between two individuals
    ///
    /// Returns a RelationshipResult containing a human-readable description of the relationship
    /// and the Most Recent Common Ancestor(s) (MRCA).
    ///
    /// # Relationship Types Detected
    ///
    /// - Direct: Parent, Child, Grandparent, Great-Grandparent, etc.
    /// - Sibling relationships: Sibling, Half-Sibling
    /// - Spouse
    /// - Aunt/Uncle, Niece/Nephew, Grand-Aunt/Grand-Uncle, etc.
    /// - Cousins: 1st Cousin, 2nd Cousin, 3rd Cousin, etc.
    /// - Removed cousins: 1st Cousin 1x Removed, 2nd Cousin 2x Removed, etc.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gedcom_rs::parse::parse_gedcom;
    /// use gedcom_rs::parse::GedcomConfig;
    ///
    /// let gedcom = parse_gedcom("file.ged", &GedcomConfig::new())?;
    /// if let (Some(person1), Some(person2)) = (
    ///     gedcom.find_individual_by_xref("@I1@"),
    ///     gedcom.find_individual_by_xref("@I2@")
    /// ) {
    ///     let relationship = gedcom.find_relationship(person1, person2);
    ///     println!("Relationship: {}", relationship.description);
    ///     if !relationship.mrca.is_empty() {
    ///         println!("Common ancestor(s): {} found", relationship.mrca.len());
    ///     }
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn find_relationship<'a>(
        &'a self,
        person1: &'a Individual,
        person2: &'a Individual,
    ) -> RelationshipResult<'a> {
        // Get xrefs for comparison
        let xref1 = match &person1.xref {
            Some(x) => x.as_str(),
            None => return RelationshipResult::none(),
        };
        let xref2 = match &person2.xref {
            Some(x) => x.as_str(),
            None => return RelationshipResult::none(),
        };

        // Check if same person
        if xref1 == xref2 {
            return RelationshipResult::self_relation();
        }

        // Check for spouse relationship
        if self
            .get_spouses(person1)
            .iter()
            .any(|s| s.xref.as_ref().map(|x| x.as_str()) == Some(xref2))
        {
            return RelationshipResult {
                description: "Spouse".to_string(),
                mrca: Vec::new(),
                generations_to_mrca_1: None,
                generations_to_mrca_2: None,
            };
        }

        // Check for parent/child relationship
        for (father, mother) in self.get_parents(person1) {
            if father.and_then(|f| f.xref.as_ref()).map(|x| x.as_str()) == Some(xref2) {
                return RelationshipResult {
                    description: "Father".to_string(),
                    mrca: Vec::new(),
                    generations_to_mrca_1: Some(1),
                    generations_to_mrca_2: Some(0),
                };
            }
            if mother.and_then(|m| m.xref.as_ref()).map(|x| x.as_str()) == Some(xref2) {
                return RelationshipResult {
                    description: "Mother".to_string(),
                    mrca: Vec::new(),
                    generations_to_mrca_1: Some(1),
                    generations_to_mrca_2: Some(0),
                };
            }
        }

        // Check if person2 is parent of person1 (inverse)
        for (father, mother) in self.get_parents(person2) {
            if father.and_then(|f| f.xref.as_ref()).map(|x| x.as_str()) == Some(xref1) {
                let description = match &person2.gender {
                    individual::Gender::Male => "Son",
                    individual::Gender::Female => "Daughter",
                    _ => "Child",
                };
                return RelationshipResult {
                    description: description.to_string(),
                    mrca: Vec::new(),
                    generations_to_mrca_1: Some(0),
                    generations_to_mrca_2: Some(1),
                };
            }
            if mother.and_then(|m| m.xref.as_ref()).map(|x| x.as_str()) == Some(xref1) {
                let description = match &person2.gender {
                    individual::Gender::Male => "Son",
                    individual::Gender::Female => "Daughter",
                    _ => "Child",
                };
                return RelationshipResult {
                    description: description.to_string(),
                    mrca: Vec::new(),
                    generations_to_mrca_1: Some(0),
                    generations_to_mrca_2: Some(1),
                };
            }
        }

        // Check for sibling relationship
        let siblings1 = self.get_siblings(person1);
        if siblings1
            .iter()
            .any(|s| s.xref.as_ref().map(|x| x.as_str()) == Some(xref2))
        {
            // Get parents to determine if full or half siblings
            let parents1: Vec<_> = self.get_parents(person1).into_iter().collect();
            let parents2: Vec<_> = self.get_parents(person2).into_iter().collect();

            let mut common_parents = Vec::new();
            for (f1, m1) in &parents1 {
                for (f2, m2) in &parents2 {
                    if let (Some(father1), Some(father2)) = (f1, f2) {
                        if father1.xref.as_ref() == father2.xref.as_ref()
                            && !common_parents
                                .iter()
                                .any(|p: &&Individual| p.xref.as_ref() == father1.xref.as_ref())
                        {
                            common_parents.push(*father1);
                        }
                    }
                    if let (Some(mother1), Some(mother2)) = (m1, m2) {
                        if mother1.xref.as_ref() == mother2.xref.as_ref()
                            && !common_parents
                                .iter()
                                .any(|p: &&Individual| p.xref.as_ref() == mother1.xref.as_ref())
                        {
                            common_parents.push(*mother1);
                        }
                    }
                }
            }

            let description = if common_parents.len() >= 2 {
                "Sibling".to_string()
            } else {
                "Half-Sibling".to_string()
            };

            return RelationshipResult {
                description,
                mrca: common_parents,
                generations_to_mrca_1: Some(1),
                generations_to_mrca_2: Some(1),
            };
        }

        // Find MRCA and calculate cousin/removed relationships
        self.find_relationship_via_mrca(person1, person2)
    }

    /// Helper function to find relationship through Most Recent Common Ancestor
    fn find_relationship_via_mrca<'a>(
        &'a self,
        person1: &'a Individual,
        person2: &'a Individual,
    ) -> RelationshipResult<'a> {
        use std::collections::{HashMap, HashSet, VecDeque};

        // Build ancestor sets with generation distances for both persons
        let mut ancestors1: HashMap<&str, (usize, &Individual)> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((person1, 0usize));

        while let Some((current, gen)) = queue.pop_front() {
            if let Some(current_xref) = &current.xref {
                ancestors1.insert(current_xref.as_str(), (gen, current));
            }

            for (father, mother) in self.get_parents(current) {
                if let Some(dad) = father {
                    if let Some(dad_xref) = &dad.xref {
                        if !ancestors1.contains_key(dad_xref.as_str()) {
                            queue.push_back((dad, gen + 1));
                        }
                    }
                }
                if let Some(mom) = mother {
                    if let Some(mom_xref) = &mom.xref {
                        if !ancestors1.contains_key(mom_xref.as_str()) {
                            queue.push_back((mom, gen + 1));
                        }
                    }
                }
            }
        }

        // Find common ancestors and their distances
        let mut common_ancestors: Vec<(&Individual, usize, usize)> = Vec::new();
        let mut visited = HashSet::new();
        queue.clear();
        queue.push_back((person2, 0usize));

        while let Some((current, gen2)) = queue.pop_front() {
            if let Some(current_xref) = &current.xref {
                let xref_str = current_xref.as_str();

                // Check if this is a common ancestor
                if let Some(&(gen1, ancestor)) = ancestors1.get(xref_str) {
                    common_ancestors.push((ancestor, gen1, gen2));
                }

                if visited.insert(xref_str) {
                    for (father, mother) in self.get_parents(current) {
                        if let Some(dad) = father {
                            queue.push_back((dad, gen2 + 1));
                        }
                        if let Some(mom) = mother {
                            queue.push_back((mom, gen2 + 1));
                        }
                    }
                }
            }
        }

        if common_ancestors.is_empty() {
            return RelationshipResult::none();
        }

        // Find the most recent common ancestor(s) - those with minimum total distance
        let min_total_distance = match common_ancestors.iter().map(|(_, g1, g2)| g1 + g2).min() {
            Some(min) => min,
            None => return RelationshipResult::none(), // No common ancestors
        };

        let mrca: Vec<&Individual> = common_ancestors
            .iter()
            .filter(|(_, g1, g2)| g1 + g2 == min_total_distance)
            .map(|(ancestor, _, _)| *ancestor)
            .collect();

        // Get the generations for the first MRCA (they should all be the same distance)
        let (_, gen1, gen2) = match common_ancestors
            .iter()
            .find(|(_, g1, g2)| g1 + g2 == min_total_distance)
        {
            Some(result) => result,
            None => return RelationshipResult::none(), // Should not happen, but be safe
        };

        let description = self.describe_relationship(*gen1, *gen2);

        RelationshipResult {
            description,
            mrca,
            generations_to_mrca_1: Some(*gen1),
            generations_to_mrca_2: Some(*gen2),
        }
    }

    /// Helper function to format ordinal numbers (1st, 2nd, 3rd, etc.)
    fn format_ordinal(n: usize) -> String {
        match n {
            1 => "1st".to_string(),
            2 => "2nd".to_string(),
            3 => "3rd".to_string(),
            n if n % 10 == 1 && n % 100 != 11 => format!("{}st", n),
            n if n % 10 == 2 && n % 100 != 12 => format!("{}nd", n),
            n if n % 10 == 3 && n % 100 != 13 => format!("{}rd", n),
            n => format!("{}th", n),
        }
    }

    /// Generate a human-readable relationship description based on generational distances
    fn describe_relationship(&self, generations1: usize, generations2: usize) -> String {
        // Direct ancestor/descendant relationships
        if generations2 == 0 {
            return match generations1 {
                1 => "Parent".to_string(),
                2 => "Grandparent".to_string(),
                3 => "Great-Grandparent".to_string(),
                n => format!("{} Great-Grandparent", Self::format_ordinal(n - 2)),
            };
        }

        if generations1 == 0 {
            return match generations2 {
                1 => "Child".to_string(),
                2 => "Grandchild".to_string(),
                3 => "Great-Grandchild".to_string(),
                n => format!("{} Great-Grandchild", Self::format_ordinal(n - 2)),
            };
        }

        // Aunt/Uncle and Niece/Nephew relationships
        if generations1 == 1 && generations2 == 2 {
            return "Niece/Nephew".to_string();
        }
        if generations1 == 2 && generations2 == 1 {
            return "Aunt/Uncle".to_string();
        }

        // Grand-Aunt/Grand-Uncle and Grand-Niece/Grand-Nephew
        if generations1 == 1 && generations2 >= 3 {
            return match generations2 {
                3 => "Grand-Niece/Grand-Nephew".to_string(),
                4 => "Great-Grand-Niece/Great-Grand-Nephew".to_string(),
                n => format!(
                    "{} Great-Grand-Niece/Great-Grand-Nephew",
                    Self::format_ordinal(n - 3)
                ),
            };
        }
        if generations1 >= 3 && generations2 == 1 {
            return match generations1 {
                3 => "Grand-Aunt/Grand-Uncle".to_string(),
                4 => "Great-Grand-Aunt/Great-Grand-Uncle".to_string(),
                n => format!(
                    "{} Great-Grand-Aunt/Great-Grand-Uncle",
                    Self::format_ordinal(n - 3)
                ),
            };
        }

        // Cousin relationships
        let min_gen = generations1.min(generations2);
        let max_gen = generations1.max(generations2);
        let removed = max_gen - min_gen;

        // Degree of cousinship (1st cousin = 2 generations to MRCA, 2nd = 3, etc.)
        let cousin_degree = min_gen.saturating_sub(1);

        if cousin_degree == 0 {
            return "Not related".to_string();
        }

        let cousin_ordinal = Self::format_ordinal(cousin_degree);

        if removed == 0 {
            format!("{} Cousin", cousin_ordinal)
        } else {
            format!("{} Cousin {}x Removed", cousin_ordinal, removed)
        }
    }
}

#[cfg(test)]
mod relationship_tests {
    use super::*;

    // Helper to create a simple test gedcom structure
    fn create_test_gedcom() -> Gedcom {
        // This would need actual test data - for now, just demonstrate the structure
        Gedcom::default()
    }

    #[test]
    fn test_relationship_result_none() {
        let result = RelationshipResult::none();
        assert_eq!(result.description, "Not related");
        assert!(result.mrca.is_empty());
        assert_eq!(result.generations_to_mrca_1, None);
        assert_eq!(result.generations_to_mrca_2, None);
    }

    #[test]
    fn test_relationship_result_self() {
        let result = RelationshipResult::self_relation();
        assert_eq!(result.description, "Self");
        assert!(result.mrca.is_empty());
        assert_eq!(result.generations_to_mrca_1, Some(0));
        assert_eq!(result.generations_to_mrca_2, Some(0));
    }

    #[test]
    fn test_describe_relationship_parent() {
        let gedcom = create_test_gedcom();
        assert_eq!(gedcom.describe_relationship(1, 0), "Parent");
        assert_eq!(gedcom.describe_relationship(2, 0), "Grandparent");
        assert_eq!(gedcom.describe_relationship(3, 0), "Great-Grandparent");
        assert_eq!(gedcom.describe_relationship(4, 0), "2nd Great-Grandparent");
    }

    #[test]
    fn test_describe_relationship_child() {
        let gedcom = create_test_gedcom();
        assert_eq!(gedcom.describe_relationship(0, 1), "Child");
        assert_eq!(gedcom.describe_relationship(0, 2), "Grandchild");
        assert_eq!(gedcom.describe_relationship(0, 3), "Great-Grandchild");
        assert_eq!(gedcom.describe_relationship(0, 4), "2nd Great-Grandchild");
    }

    #[test]
    fn test_describe_relationship_aunt_uncle() {
        let gedcom = create_test_gedcom();
        assert_eq!(gedcom.describe_relationship(2, 1), "Aunt/Uncle");
        assert_eq!(gedcom.describe_relationship(3, 1), "Grand-Aunt/Grand-Uncle");
        assert_eq!(
            gedcom.describe_relationship(4, 1),
            "Great-Grand-Aunt/Great-Grand-Uncle"
        );
    }

    #[test]
    fn test_describe_relationship_niece_nephew() {
        let gedcom = create_test_gedcom();
        assert_eq!(gedcom.describe_relationship(1, 2), "Niece/Nephew");
        assert_eq!(
            gedcom.describe_relationship(1, 3),
            "Grand-Niece/Grand-Nephew"
        );
        assert_eq!(
            gedcom.describe_relationship(1, 4),
            "Great-Grand-Niece/Great-Grand-Nephew"
        );
    }

    #[test]
    fn test_describe_relationship_cousins() {
        let gedcom = create_test_gedcom();
        // First cousins (both 2 generations from common ancestor)
        assert_eq!(gedcom.describe_relationship(2, 2), "1st Cousin");

        // Second cousins (both 3 generations from common ancestor)
        assert_eq!(gedcom.describe_relationship(3, 3), "2nd Cousin");

        // Third cousins
        assert_eq!(gedcom.describe_relationship(4, 4), "3rd Cousin");
    }

    #[test]
    fn test_describe_relationship_cousins_removed() {
        let gedcom = create_test_gedcom();
        // First cousin once removed
        assert_eq!(gedcom.describe_relationship(2, 3), "1st Cousin 1x Removed");
        assert_eq!(gedcom.describe_relationship(3, 2), "1st Cousin 1x Removed");

        // First cousin twice removed
        assert_eq!(gedcom.describe_relationship(2, 4), "1st Cousin 2x Removed");
        assert_eq!(gedcom.describe_relationship(4, 2), "1st Cousin 2x Removed");

        // Second cousin once removed
        assert_eq!(gedcom.describe_relationship(3, 4), "2nd Cousin 1x Removed");
        assert_eq!(gedcom.describe_relationship(4, 3), "2nd Cousin 1x Removed");

        // Second cousin twice removed
        assert_eq!(gedcom.describe_relationship(3, 5), "2nd Cousin 2x Removed");
    }

    #[test]
    fn test_describe_relationship_higher_order_cousins() {
        let gedcom = create_test_gedcom();
        assert_eq!(gedcom.describe_relationship(5, 5), "4th Cousin");
        assert_eq!(gedcom.describe_relationship(6, 6), "5th Cousin");
        assert_eq!(gedcom.describe_relationship(11, 11), "10th Cousin");
        assert_eq!(gedcom.describe_relationship(12, 12), "11th Cousin");
    }
}

#[cfg(test)]
mod api_tests {
    use super::*;
    use std::fs;

    /// Helper to create a test GEDCOM with family data for API testing
    /// Returns a Gedcom with 6 individuals and 2 families
    fn create_test_family_gedcom() -> Gedcom {
        use crate::parse::{parse_gedcom, GedcomConfig};
        use std::sync::atomic::{AtomicU64, Ordering};

        // Use atomic counter to ensure unique filenames for parallel test execution
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_file = format!("test_api_family_{}.ged", id);

        let content = "\
0 HEAD
1 CHAR UTF-8
0 @I1@ INDI
1 NAME John /Doe/
1 SEX M
1 BIRT
2 DATE 1 JAN 1900
1 FAMS @F1@
0 @I2@ INDI
1 NAME Jane /Smith/
1 SEX F
1 FAMS @F1@
0 @I3@ INDI
1 NAME Robert /Doe/
1 SEX M
1 BIRT
2 DATE 15 JUN 1925
1 FAMC @F1@
1 FAMS @F2@
0 @I4@ INDI
1 NAME Mary /Jones/
1 SEX F
1 FAMS @F2@
0 @I5@ INDI
1 NAME Alice /Doe/
1 SEX F
1 BIRT
2 DATE 10 MAR 1950
1 FAMC @F2@
0 @I6@ INDI
1 NAME Bob /Doe/
1 SEX M
1 FAMC @F2@
0 @F1@ FAM
1 HUSB @I1@
1 WIFE @I2@
1 CHIL @I3@
0 @F2@ FAM
1 HUSB @I3@
1 WIFE @I4@
1 CHIL @I5@
1 CHIL @I6@
0 TRLR
";

        fs::write(&temp_file, content).expect("Failed to write test file");
        let gedcom =
            parse_gedcom(&temp_file, &GedcomConfig::new()).expect("Failed to parse test GEDCOM");
        let _ = fs::remove_file(&temp_file);

        gedcom
    }

    // ========================================================================
    // Search and Query API Tests
    // ========================================================================

    #[test]
    fn test_has_warnings() {
        let mut gedcom = Gedcom::default();
        assert!(!gedcom.has_warnings());

        gedcom
            .warnings
            .push(crate::error::GedcomError::ValidationError {
                record_type: "TEST".to_string(),
                record_xref: None,
                field: "FIELD".to_string(),
                message: "Test warning".to_string(),
            });

        assert!(gedcom.has_warnings());
    }

    #[test]
    fn test_find_individual_by_xref_found() {
        let gedcom = create_test_family_gedcom();

        let result = gedcom.find_individual_by_xref("@I1@");
        assert!(result.is_some());

        let individual = result.unwrap();
        assert_eq!(individual.xref.as_ref().unwrap().as_str(), "@I1@");
        assert!(!individual.names.is_empty());
    }

    #[test]
    fn test_find_individual_by_xref_not_found() {
        let gedcom = create_test_family_gedcom();

        let result = gedcom.find_individual_by_xref("@I999@");
        assert!(result.is_none());
    }

    #[test]
    fn test_find_individual_by_xref_all_individuals() {
        let gedcom = create_test_family_gedcom();

        // Verify all 6 individuals can be found
        for xref in &["@I1@", "@I2@", "@I3@", "@I4@", "@I5@", "@I6@"] {
            let result = gedcom.find_individual_by_xref(xref);
            assert!(result.is_some(), "Should find individual {}", xref);
            assert_eq!(result.unwrap().xref.as_ref().unwrap().as_str(), *xref);
        }
    }

    #[test]
    fn test_find_individuals_by_name_exact_match() {
        let gedcom = create_test_family_gedcom();

        let results = gedcom.find_individuals_by_name("John");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].xref.as_ref().unwrap().as_str(), "@I1@");
    }

    #[test]
    fn test_find_individuals_by_name_case_insensitive() {
        let gedcom = create_test_family_gedcom();

        let results = gedcom.find_individuals_by_name("JOHN");
        assert_eq!(results.len(), 1);

        let results_lower = gedcom.find_individuals_by_name("john");
        assert_eq!(results_lower.len(), 1);
    }

    #[test]
    fn test_find_individuals_by_name_surname_match() {
        let gedcom = create_test_family_gedcom();

        let results = gedcom.find_individuals_by_name("Doe");
        // Should match: John Doe, Robert Doe, Alice Doe, Bob Doe = 4
        assert_eq!(results.len(), 4);
    }

    #[test]
    fn test_find_individuals_by_name_partial_match() {
        let gedcom = create_test_family_gedcom();

        let results = gedcom.find_individuals_by_name("o");
        // Should match: John, Robert, Bob, Jones = at least 4
        assert!(results.len() >= 4);
    }

    #[test]
    fn test_find_individuals_by_name_no_match() {
        let gedcom = create_test_family_gedcom();

        let results = gedcom.find_individuals_by_name("NonExistentName");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_find_family_by_xref_found() {
        let gedcom = create_test_family_gedcom();

        let result = gedcom.find_family_by_xref("@F1@");
        assert!(result.is_some());

        let family = result.unwrap();
        assert_eq!(family.xref.as_str(), "@F1@");
    }

    #[test]
    fn test_find_family_by_xref_not_found() {
        let gedcom = create_test_family_gedcom();

        let result = gedcom.find_family_by_xref("@F999@");
        assert!(result.is_none());
    }

    #[test]
    fn test_find_family_by_xref_all_families() {
        let gedcom = create_test_family_gedcom();

        for xref in &["@F1@", "@F2@"] {
            let result = gedcom.find_family_by_xref(xref);
            assert!(result.is_some(), "Should find family {}", xref);
            assert_eq!(result.unwrap().xref.as_str(), *xref);
        }
    }

    #[test]
    fn test_find_individuals_by_event_date_birth_exact() {
        let gedcom = create_test_family_gedcom();

        let results = gedcom.find_individuals_by_event_date(EventType::Birth, "1 JAN 1900");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].xref.as_ref().unwrap().as_str(), "@I1@");
    }

    #[test]
    fn test_find_individuals_by_event_date_partial() {
        let gedcom = create_test_family_gedcom();

        let results = gedcom.find_individuals_by_event_date(EventType::Birth, "1950");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].xref.as_ref().unwrap().as_str(), "@I5@");
    }

    #[test]
    fn test_find_individuals_by_event_date_no_match() {
        let gedcom = create_test_family_gedcom();

        let results = gedcom.find_individuals_by_event_date(EventType::Birth, "2000");
        assert_eq!(results.len(), 0);
    }

    // ========================================================================
    // Relationship API Tests
    // ========================================================================

    #[test]
    fn test_get_parents_with_parents() {
        let gedcom = create_test_family_gedcom();

        // I3 (Robert) has parents I1 and I2
        let robert = gedcom.find_individual_by_xref("@I3@").unwrap();
        let parents = gedcom.get_parents(robert);

        assert_eq!(parents.len(), 1);
        let (father, mother) = &parents[0];

        assert!(father.is_some());
        assert!(mother.is_some());
        assert_eq!(father.unwrap().xref.as_ref().unwrap().as_str(), "@I1@");
        assert_eq!(mother.unwrap().xref.as_ref().unwrap().as_str(), "@I2@");
    }

    #[test]
    fn test_get_parents_no_parents() {
        let gedcom = create_test_family_gedcom();

        // I1 (John) has no parents
        let john = gedcom.find_individual_by_xref("@I1@").unwrap();
        let parents = gedcom.get_parents(john);

        assert_eq!(parents.len(), 0);
    }

    #[test]
    fn test_get_children_with_children() {
        let gedcom = create_test_family_gedcom();

        // I3 (Robert) has children I5 and I6
        let robert = gedcom.find_individual_by_xref("@I3@").unwrap();
        let children = gedcom.get_children(robert);

        assert_eq!(children.len(), 2);

        let child_xrefs: Vec<_> = children
            .iter()
            .map(|c| c.xref.as_ref().unwrap().as_str())
            .collect();
        assert!(child_xrefs.contains(&"@I5@"));
        assert!(child_xrefs.contains(&"@I6@"));
    }

    #[test]
    fn test_get_children_no_children() {
        let gedcom = create_test_family_gedcom();

        // I5 (Alice) has no children
        let alice = gedcom.find_individual_by_xref("@I5@").unwrap();
        let children = gedcom.get_children(alice);

        assert_eq!(children.len(), 0);
    }

    #[test]
    fn test_get_spouses_with_spouse() {
        let gedcom = create_test_family_gedcom();

        // I1 (John) is married to I2 (Jane)
        let john = gedcom.find_individual_by_xref("@I1@").unwrap();
        let spouses = gedcom.get_spouses(john);

        assert_eq!(spouses.len(), 1);
        assert_eq!(spouses[0].xref.as_ref().unwrap().as_str(), "@I2@");
    }

    #[test]
    fn test_get_spouses_no_spouse() {
        let gedcom = create_test_family_gedcom();

        // I5 (Alice) has no spouse
        let alice = gedcom.find_individual_by_xref("@I5@").unwrap();
        let spouses = gedcom.get_spouses(alice);

        assert_eq!(spouses.len(), 0);
    }

    #[test]
    fn test_get_siblings_with_siblings() {
        let gedcom = create_test_family_gedcom();

        // I5 (Alice) has sibling I6 (Bob)
        let alice = gedcom.find_individual_by_xref("@I5@").unwrap();
        let siblings = gedcom.get_siblings(alice);

        assert_eq!(siblings.len(), 1);
        assert_eq!(siblings[0].xref.as_ref().unwrap().as_str(), "@I6@");
    }

    #[test]
    fn test_get_siblings_no_siblings() {
        let gedcom = create_test_family_gedcom();

        // I3 (Robert) is an only child
        let robert = gedcom.find_individual_by_xref("@I3@").unwrap();
        let siblings = gedcom.get_siblings(robert);

        assert_eq!(siblings.len(), 0);
    }

    #[test]
    fn test_get_full_siblings_same_parents() {
        let gedcom = create_test_family_gedcom();

        // I5 and I6 are full siblings (same parents)
        let alice = gedcom.find_individual_by_xref("@I5@").unwrap();
        let full_siblings = gedcom.get_full_siblings(alice);

        assert_eq!(full_siblings.len(), 1);
        assert_eq!(full_siblings[0].xref.as_ref().unwrap().as_str(), "@I6@");
    }

    #[test]
    fn test_get_full_siblings_no_siblings() {
        let gedcom = create_test_family_gedcom();

        let robert = gedcom.find_individual_by_xref("@I3@").unwrap();
        let full_siblings = gedcom.get_full_siblings(robert);

        assert_eq!(full_siblings.len(), 0);
    }

    #[test]
    fn test_get_half_siblings_none() {
        let gedcom = create_test_family_gedcom();

        // I5 and I6 are full siblings, not half-siblings
        let alice = gedcom.find_individual_by_xref("@I5@").unwrap();
        let half_siblings = gedcom.get_half_siblings(alice);

        assert_eq!(half_siblings.len(), 0);
    }

    #[test]
    fn test_get_ancestors_multiple_generations() {
        let gedcom = create_test_family_gedcom();

        // I5 (Alice) should have 4 ancestors: I3, I4, I1, I2
        let alice = gedcom.find_individual_by_xref("@I5@").unwrap();
        let ancestors = gedcom.get_ancestors(alice, Some(10));

        assert_eq!(ancestors.len(), 4);

        let ancestor_xrefs: Vec<_> = ancestors
            .iter()
            .map(|a| a.xref.as_ref().unwrap().as_str())
            .collect();

        // Should include parents
        assert!(ancestor_xrefs.contains(&"@I3@")); // Father
        assert!(ancestor_xrefs.contains(&"@I4@")); // Mother

        // Should include grandparents
        assert!(ancestor_xrefs.contains(&"@I1@")); // Grandfather
        assert!(ancestor_xrefs.contains(&"@I2@")); // Grandmother
    }

    #[test]
    fn test_get_ancestors_with_max_generations() {
        let gedcom = create_test_family_gedcom();

        let alice = gedcom.find_individual_by_xref("@I5@").unwrap();
        let ancestors = gedcom.get_ancestors(alice, Some(1));

        // Should only get parents, not grandparents
        assert_eq!(ancestors.len(), 2);

        let ancestor_xrefs: Vec<_> = ancestors
            .iter()
            .map(|a| a.xref.as_ref().unwrap().as_str())
            .collect();

        assert!(ancestor_xrefs.contains(&"@I3@"));
        assert!(ancestor_xrefs.contains(&"@I4@"));
    }

    #[test]
    fn test_get_ancestors_no_ancestors() {
        let gedcom = create_test_family_gedcom();

        let john = gedcom.find_individual_by_xref("@I1@").unwrap();
        let ancestors = gedcom.get_ancestors(john, Some(10));

        assert_eq!(ancestors.len(), 0);
    }

    #[test]
    fn test_get_descendants_multiple_generations() {
        let gedcom = create_test_family_gedcom();

        // I1 (John) should have 3 descendants: I3, I5, I6
        let john = gedcom.find_individual_by_xref("@I1@").unwrap();
        let descendants = gedcom.get_descendants(john, Some(10));

        assert_eq!(descendants.len(), 3);

        let descendant_xrefs: Vec<_> = descendants
            .iter()
            .map(|d| d.xref.as_ref().unwrap().as_str())
            .collect();

        assert!(descendant_xrefs.contains(&"@I3@")); // Son
        assert!(descendant_xrefs.contains(&"@I5@")); // Granddaughter
        assert!(descendant_xrefs.contains(&"@I6@")); // Grandson
    }

    #[test]
    fn test_get_descendants_with_max_generations() {
        let gedcom = create_test_family_gedcom();

        let john = gedcom.find_individual_by_xref("@I1@").unwrap();
        let descendants = gedcom.get_descendants(john, Some(1));

        // Should only get children, not grandchildren
        assert_eq!(descendants.len(), 1);
        assert_eq!(descendants[0].xref.as_ref().unwrap().as_str(), "@I3@");
    }

    #[test]
    fn test_get_descendants_no_descendants() {
        let gedcom = create_test_family_gedcom();

        let alice = gedcom.find_individual_by_xref("@I5@").unwrap();
        let descendants = gedcom.get_descendants(alice, Some(10));

        assert_eq!(descendants.len(), 0);
    }

    #[test]
    fn test_find_relationship_path_parent_child() {
        let gedcom = create_test_family_gedcom();

        let john = gedcom.find_individual_by_xref("@I1@").unwrap();
        let robert = gedcom.find_individual_by_xref("@I3@").unwrap();

        let path = gedcom.find_relationship_path(john, robert);
        assert!(path.is_some());

        let path_vec = path.unwrap();
        assert!(path_vec.len() >= 2);
        assert_eq!(path_vec[0].xref.as_ref().unwrap().as_str(), "@I1@");
        assert_eq!(
            path_vec[path_vec.len() - 1].xref.as_ref().unwrap().as_str(),
            "@I3@"
        );
    }

    #[test]
    fn test_find_relationship_path_self() {
        let gedcom = create_test_family_gedcom();

        let john = gedcom.find_individual_by_xref("@I1@").unwrap();

        let path = gedcom.find_relationship_path(john, john);
        assert!(path.is_some());

        let path_vec = path.unwrap();
        assert_eq!(path_vec.len(), 1);
        assert_eq!(path_vec[0].xref.as_ref().unwrap().as_str(), "@I1@");
    }

    #[test]
    fn test_find_relationship_self() {
        let gedcom = create_test_family_gedcom();

        let john = gedcom.find_individual_by_xref("@I1@").unwrap();

        let result = gedcom.find_relationship(john, john);
        assert_eq!(result.description, "Self");
        assert_eq!(result.generations_to_mrca_1, Some(0));
        assert_eq!(result.generations_to_mrca_2, Some(0));
    }

    #[test]
    fn test_find_relationship_parent() {
        let gedcom = create_test_family_gedcom();

        let john = gedcom.find_individual_by_xref("@I1@").unwrap();
        let robert = gedcom.find_individual_by_xref("@I3@").unwrap();

        let result = gedcom.find_relationship(john, robert);
        assert!(result.description.contains("Child") || result.description.contains("Son"));
    }

    #[test]
    fn test_find_relationship_siblings() {
        let gedcom = create_test_family_gedcom();

        let alice = gedcom.find_individual_by_xref("@I5@").unwrap();
        let bob = gedcom.find_individual_by_xref("@I6@").unwrap();

        let result = gedcom.find_relationship(alice, bob);
        assert_eq!(result.description, "Sibling");
        assert_eq!(result.generations_to_mrca_1, Some(1));
        assert_eq!(result.generations_to_mrca_2, Some(1));
    }
}
