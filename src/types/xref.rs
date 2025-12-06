// An xref is a cross-reference to another record in the GEDCOM file.
use crate::types::Line;
use std::fmt;
use winnow::prelude::*;

/// A cross-reference identifier in GEDCOM format (e.g., "@I1@", "@F1@")
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Xref(String);

impl fmt::Display for Xref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Xref {
    /// Create a new Xref from a string
    pub fn new(s: impl Into<String>) -> Self {
        Xref(s.into())
    }

    /// Get the xref as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if the xref is valid (starts and ends with '@')
    pub fn is_valid(&self) -> bool {
        self.0.starts_with('@') && self.0.ends_with('@')
    }

    /// Parse an xref from the next line in the buffer
    pub fn parse(record: &mut &str) -> PResult<Option<Xref>> {
        // We find the xref in two places, potentially. The first is in the tag,
        // the second is in the value. Line::parse will set xref if it's in the tag.
        // If there's no xref, we'll check the value to see if it contains one.
        let line = Line::parse(record)?;

        if !line.xref.is_empty() {
            Ok(Some(Xref::new(line.xref.to_string())))
        } else if line.value.starts_with('@') && line.value.ends_with('@') {
            // HACK: Might be better to do this in Line::parse.
            Ok(Some(Xref::new(line.value.to_string())))
        } else {
            Ok(None)
        }
    }
}

impl From<String> for Xref {
    fn from(s: String) -> Self {
        Xref(s)
    }
}

impl From<&str> for Xref {
    fn from(s: &str) -> Self {
        Xref(s.to_string())
    }
}

impl AsRef<str> for Xref {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Xref {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Xref {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for Xref {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_xref_from_tag() {
        let data = vec!["0 @I1@ INDI"];

        let input = data.join("\n");
        let mut record = input.as_str();

        let xref = Xref::parse(&mut record).unwrap();
        assert_eq!("@I1@", xref.unwrap().as_str());
    }

    #[test]
    fn parse_xref_from_value() {
        let data = vec!["1 HUSB @I1@"];

        let input = data.join("\n");
        let mut record = input.as_str();

        let xref = Xref::parse(&mut record).unwrap();
        assert_eq!("@I1@", xref.unwrap().as_str());
    }
}
