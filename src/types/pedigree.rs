use std::str::FromStr;

// PEDIGREE_LINKAGE_TYPE:= {Size=5:7}
// [ adopted | birth | foster | sealing ]
// A code used to indicate the child to family relationship for pedigree navigation purposes.
// Where:
// adopted = indicates adoptive parents.
// birth = indicates birth parents.
// foster = indicates child was included in a foster or guardian family.
// sealing = indicates child was sealed to parents other than birth parents.

#[derive(Default, Debug, PartialEq, Clone)]
/// The quantitative eveluation of the credibility of a piece of information
/// based upon its supporting evidence.
pub enum Pedigree {
    /// Adoptive parents
    Adopted,
    #[default]
    /// Birth parents
    Birth,
    /// Foster parents
    Foster,
    // Sealed to parents other than birth parents
    Sealing,
}

impl FromStr for Pedigree {
    type Err = ();

    fn from_str(input: &str) -> Result<Pedigree, Self::Err> {
        match input {
            "adopted" => Ok(Pedigree::Adopted),
            "birth" => Ok(Pedigree::Birth),
            "foster" => Ok(Pedigree::Foster),
            "sealing" => Ok(Pedigree::Sealing),
            _ => Err(()),
        }
    }
}
