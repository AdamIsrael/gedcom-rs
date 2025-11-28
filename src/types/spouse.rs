// A parser for the HUSB and WIFE structures.
use crate::types::Line;
use crate::types::Xref;

use winnow::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Spouse {
    // the xref of the spouse
    pub xref: Option<Xref>,
    pub age: Option<String>,
}

impl Spouse {
    pub fn parse(record: &mut &str) -> PResult<Spouse> {
        let mut spouse = Spouse::default();

        // Get the starting level of the current line
        let mut line = Line::peek(record)?;
        let level = line.level;

        while !record.is_empty() {
            match line.tag {
                "HUSB" => {
                    let xref = Xref::parse(record)?;
                    if xref.xref.is_some() {
                        spouse.xref = Some(xref);
                    }
                }
                "WIFE" => {
                    let xref = Xref::parse(record)?;
                    if xref.xref.is_some() {
                        spouse.xref = Some(xref);
                    }
                }
                "AGE" => {
                    spouse.age = Some(line.value.to_string());
                    // Consume the line
                    let _ = Line::parse(record);
                }
                _ => {
                    // Consume the line
                    let _ = Line::parse(record);
                    break;
                }
            }
            // Look ahead
            line = Line::peek(record)?;

            // If the next line is at the same level, we're done parsing this structure
            if line.level <= level {
                break;
            }
        }

        Ok(spouse)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spouse_1() {
        let data = vec!["2 HUSB", "3 AGE 42y"];

        let input = data.join("\n");
        let mut record = input.as_str();

        let spouse = Spouse::parse(&mut record).unwrap();
        assert!("42y" == spouse.age.unwrap());
        assert!(spouse.xref.is_none())
    }

    #[test]
    fn parse_spouse_2() {
        let data = vec!["1 HUSB @I5@"];

        let input = data.join("\n");
        let mut record = input.as_str();

        let spouse = Spouse::parse(&mut record).unwrap();

        assert!(spouse.age.is_none());
        assert!(spouse.xref.is_some());
        assert!("@I5@".to_string() == spouse.xref.unwrap().xref.unwrap());
    }

    #[test]
    // Make sure that we're only parsing a single record
    fn parse_spouse_3() {
        let data = vec!["2 HUSB @I5@", "3 AGE 42y", "2 WIFE @I6@", "3 AGE 39y"];

        let input = data.join("\n");
        let mut record = input.as_str();

        let spouse = Spouse::parse(&mut record).unwrap();

        assert!(spouse.age.is_some());
        assert!("42y".to_string() == spouse.age.unwrap());

        assert!(spouse.xref.is_some());
        assert!("@I5@".to_string() == spouse.xref.unwrap().xref.unwrap());
    }
}
