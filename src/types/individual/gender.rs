use std::str::FromStr;

#[derive(Default, Debug, PartialEq)]
/// The Gender of the Individual
pub enum Gender {
    Male,
    Female,
    Nonbinary,
    #[default]
    Unknown,
}

impl FromStr for Gender {
    type Err = ();

    fn from_str(input: &str) -> Result<Gender, Self::Err> {
        match input {
            "M"  => Ok(Gender::Male),
            "F"  => Ok(Gender::Female),
            "N"  => Ok(Gender::Nonbinary),
            "U" => Ok(Gender::Unknown),
            _      => Err(()),
        }
    }
}