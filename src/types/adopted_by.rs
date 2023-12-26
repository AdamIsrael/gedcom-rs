use std::str::FromStr;

// ADOPTED_BY_WHICH_PARENT:= {Size=1:4}
// [ HUSB | WIFE | BOTH ]
// A code which shows which parent in the associated family record adopted this person.
// Where:
// HUSB = The HUSBand in the associated family adopted this person.
// WIFE = The WIFE in the associated family adopted this person.
// BOTH = Both HUSBand and WIFE adopted this person.

#[derive(Default, Debug, PartialEq, Clone)]
/// Which parent in the associated family record adopted this person.
pub enum AdoptedBy {
    #[default]
    /// Both HUSBand and WIFE adopted this person.
    Both,
    /// The HUSBand in the associated family adopted this person.
    Husband,
    // The WIFE in the associated family adopted this person.
    Wife,
}

impl FromStr for AdoptedBy {
    type Err = ();

    fn from_str(input: &str) -> Result<AdoptedBy, Self::Err> {
        match input {
            "BOTH" => Ok(AdoptedBy::Both),
            "HUSB" => Ok(AdoptedBy::Husband),
            "WIFE" => Ok(AdoptedBy::Wife),
            _ => Err(()),
        }
    }
}
