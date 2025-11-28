use std::str::FromStr;

// CERTAINTY_ASSESSMENT:= {Size=1:1}
// [ 0 | 1 | 2 | 3 ]
// The QUAY tag's value conveys the submitter's quantitative evaluation of the credibility of a piece of
// information, based upon its supporting evidence. Some systems use this feature to rank multiple
// conflicting opinions for display of most likely information first. It is not intended to eliminate the
// receiver's need to evaluate the evidence for themselves.
// 0 = Unreliable evidence or estimated data
// 1 = Questionable reliability of evidence (interviews, census, oral genealogies, or potential for bias
// for example, an autobiography)
// 2 = Secondary evidence, data officially recorded sometime after event
// 3 = Direct and primary evidence used, or by dominance of the evidence

#[derive(Clone, Default, Debug, PartialEq)]
/// The quantitative eveluation of the credibility of a piece of information
/// based upon its supporting evidence.
pub enum Quay {
    #[default]
    Unreliable,
    Questionable,
    Secondary,
    Direct,
}

impl FromStr for Quay {
    type Err = ();

    fn from_str(input: &str) -> Result<Quay, Self::Err> {
        match input {
            "0" => Ok(Quay::Unreliable),
            "1" => Ok(Quay::Questionable),
            "2" => Ok(Quay::Secondary),
            "3" => Ok(Quay::Direct),
            _ => Err(()),
        }
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::Quay;
    use std::str::FromStr;

    #[test]
    fn parse_quay() {
        assert!(Quay::from_str("0").unwrap() == Quay::Unreliable);
        assert!(Quay::from_str("1").unwrap() == Quay::Questionable);
        assert!(Quay::from_str("2").unwrap() == Quay::Secondary);
        assert!(Quay::from_str("3").unwrap() == Quay::Direct);
        assert!(Quay::from_str("4").is_err());
    }
}
