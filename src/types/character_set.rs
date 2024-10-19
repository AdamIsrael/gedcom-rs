use super::Line;

#[derive(Debug, Default, PartialEq)]
pub struct CharacterSet {
    /// The version of this Gedcom file.
    pub encoding: Option<String>,

    pub version: Option<String>,
}
impl CharacterSet {
    pub fn parse(mut buffer: &str) -> (&str, Option<CharacterSet>) {
        let mut char = CharacterSet {
            encoding: None,
            version: None,
        };
        let mut line: Line;

        line = Line::peek(&mut buffer).unwrap();
        char.encoding = Some(line.value.to_string());

        if line.tag == "CHAR" {
            Line::parse(&mut buffer).unwrap();

            while !buffer.is_empty() {
                // Peek the next line
                line = Line::peek(&mut buffer).unwrap();
                match line.tag {
                    "VERS" => {
                        // consume the line
                        line = Line::parse(&mut buffer).unwrap();
                        char.version = Some(line.value.to_string());
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
