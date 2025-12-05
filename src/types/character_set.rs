use super::Line;

#[derive(Debug, Default, PartialEq)]
pub struct CharacterSet {
    /// The version of this Gedcom file.
    pub encoding: Option<String>,

    pub version: Option<String>,
}
impl CharacterSet {
    pub fn parse(mut buffer: &str) -> (&str, Option<CharacterSet>) {
        let mut char = CharacterSet::default();
        let mut line: Line;

        if let Ok(l) = Line::peek(&mut buffer) {
            line = l;
            char.encoding = Some(line.value.to_string());
        } else {
            return (buffer, None);
        }

        if line.tag == "CHAR" {
            if Line::parse(&mut buffer).is_err() {
                return (buffer, None);
            }

            while !buffer.is_empty() {
                // Peek the next line
                if let Ok(l) = Line::peek(&mut buffer) {
                    line = l;
                } else {
                    break;
                }
                match line.tag {
                    "VERS" => {
                        // consume the line
                        if let Ok(l) = Line::parse(&mut buffer) {
                            line = l;
                            char.version = Some(line.value.to_string());
                        }
                    }
                    _ => {
                        // panic!("Unknown tag: {}", line.tag);
                        break;
                    }
                }
            }
        }

        (buffer, Some(char))
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::CharacterSet;

    #[test]
    fn parse() {
        let data = vec!["1 CHAR UTF-8", "2 VERS 5.5.5"];

        let (_data, _char) = CharacterSet::parse(&data.join("\n"));
        let char = _char.unwrap();

        assert!(char.encoding == Some("UTF-8".to_string()));
        assert!(char.version == Some("5.5.5".to_string()));
    }
}
