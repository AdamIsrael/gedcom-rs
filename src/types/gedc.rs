use super::Line;
use crate::parse;

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
        let mut form = Form {
            name: None,
            version: None,
        };

        let mut line: Line;

        (_, line) = parse::peek_line(buffer).unwrap();

        if line.tag == "FORM" {
            (buffer, line) = parse::line(buffer).unwrap();

            form.name = Some(line.value.to_string());

            while !buffer.is_empty() {
                // Peek the next line
                (_, line) = parse::peek_line(buffer).unwrap();
                match line.tag {
                    "VERS" => {
                        // consume the line
                        (buffer, line) = parse::line(buffer).unwrap();
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
        let mut gedc = Gedc {
            version: None,
            form: None,
        };
        let mut line: Line;

        (_, line) = parse::peek_line(buffer).unwrap();

        if line.tag == "GEDC" {
            (buffer, _) = parse::line(buffer).unwrap();

            while !buffer.is_empty() {
                // Peek the next line
                (_, line) = parse::peek_line(buffer).unwrap();
                match line.tag {
                    "FORM" => {
                        (buffer, gedc.form) = Form::parse(buffer);
                    }
                    "VERS" => {
                        // consume the line
                        (buffer, line) = parse::line(buffer).unwrap();
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
