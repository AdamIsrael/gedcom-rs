use super::Line;
// use crate::parse;

#[derive(Debug, Default, PartialEq)]
// pub enum Form {
//     LineageLinked,
//     EventLineageLinked,
//     Unknown,
// }
pub struct Form {
    pub name: Option<String>,
    pub version: Option<String>,
}
impl Form {
    pub fn parse(mut buffer: &str) -> (&str, Option<Form>) {
        let mut form = Form::default();

        let Ok(mut line) = Line::peek(&mut buffer) else {
            return (buffer, Some(form));
        };

        if line.tag == "FORM" {
            let Ok(parsed_line) = Line::parse(&mut buffer) else {
                return (buffer, Some(form));
            };
            line = parsed_line;

            form.name = Some(line.value.to_string());

            while !buffer.is_empty() {
                // Peek the next line
                let Ok(next_line) = Line::peek(&mut buffer) else {
                    break;
                };
                line = next_line;
                match line.tag {
                    "VERS" => {
                        // consume the line
                        let Ok(parsed_line) = Line::parse(&mut buffer) else {
                            break;
                        };
                        line = parsed_line;
                        form.version = Some(line.value.to_string());
                    }
                    _ => {
                        // panic!("Unknown tag: {}", line.tag);
                        break;
                    }
                }
            }
        }

        (buffer, Some(form))
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Gedc {
    /// The version of this Gedcom file.
    pub version: Option<String>,

    /// The GEDCOM form used to construct this transmission.
    pub form: Option<Form>,
}
impl Gedc {
    pub fn parse(mut buffer: &str) -> (&str, Option<Gedc>) {
        let mut gedc = Gedc::default();

        let Ok(mut line) = Line::peek(&mut buffer) else {
            return (buffer, Some(gedc));
        };

        if line.tag == "GEDC" {
            let _ = Line::parse(&mut buffer);

            while !buffer.is_empty() {
                // Peek the next line
                let Ok(next_line) = Line::peek(&mut buffer) else {
                    break;
                };
                line = next_line;
                match line.tag {
                    "FORM" => {
                        (buffer, gedc.form) = Form::parse(buffer);
                    }
                    "VERS" => {
                        // consume the line
                        let Ok(parsed_line) = Line::parse(&mut buffer) else {
                            break;
                        };
                        line = parsed_line;
                        gedc.version = Some(line.value.to_string());
                    }
                    _ => {
                        // panic!("Unknown tag: {}", line.tag);
                        break;
                    }
                }
            }
        }

        (buffer, Some(gedc))
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::Gedc;

    #[test]
    fn parse() {
        let data = vec![
            "1 GEDC",
            "2 VERS 5.5.5",
            "2 FORM LINEAGE-LINKED",
            "3 VERS 5.5.5",
        ];

        let (_data, _gedc) = Gedc::parse(&data.join("\n"));
        let gedc = _gedc.unwrap();
        let form = gedc.form.unwrap();

        assert!(gedc.version == Some("5.5.5".to_string()));

        assert!(form.name == Some("LINEAGE-LINKED".to_string()));
        assert!(form.version == Some("5.5.5".to_string()));

        // // verify that the entire buffer has been consumed
        // assert!(_data.is_empty())
    }
}
