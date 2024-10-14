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
            "M" => Ok(Gender::Male),
            "F" => Ok(Gender::Female),
            "N" => Ok(Gender::Nonbinary),
            "U" => Ok(Gender::Unknown),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Gender;
    use std::str::FromStr;

    #[test]
    fn parse_gender() {
        assert!(Gender::from_str("M").unwrap() == Gender::Male);
        assert!(Gender::from_str("F").unwrap() == Gender::Female);
        assert!(Gender::from_str("N").unwrap() == Gender::Nonbinary);
        assert!(Gender::from_str("U").unwrap() == Gender::Unknown);
        assert!(Gender::from_str("X").is_err());
    }
}
