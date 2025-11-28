use crate::types::Line;

// +1 SUBN @<XREF:SUBN>@

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Submission {
    /// The pointer to the SUBN record
    pub xref: Option<String>,
}

impl Submission {
    /// Parses a SUBN block
    pub fn parse(mut buffer: &str) -> (&str, Option<Submission>) {
        let mut submission: Option<Submission> = None;
        
        if let Ok(line) = Line::peek(&mut buffer) {
            if line.level == 1 && line.tag == "SUBN" {
                // advance our position in the buffer
                if let Ok(parsed_line) = Line::parse(&mut buffer) {
                    // This is a temporary hack, because parse::xref strips @ from the id
                    let xref = parsed_line.value;

                    submission = Some(Submission {
                        xref: Some(xref.to_string()),
                    });
                }
            }
        }

        (buffer, submission)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::Submission;

    #[test]
    fn parse_submission() {
        let data = vec!["1 SUBN @U1@"];

        let (_, submission) = Submission::parse(data.join("\n").as_str());
        if let Some(s) = submission {
            assert!(s.xref == Some("@U1@".to_string()));
        } else {
            // We couldn't parse the submission
            assert!(false);
        }
    }
}
