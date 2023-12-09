use super::{DateTime, Line};
use crate::parse;

//     +2 DATA <NAME_OF_SOURCE_DATA>
//         +3 DATE <PUBLICATION_DATE>
//         +3 COPR <COPYRIGHT_SOURCE_DATA>
//         +4 [CONT|CONC]<COPYRIGHT_SOURCE_DATA>
#[derive(Debug, Default, PartialEq)]
pub struct SourceData {
    pub name: Option<String>,
    pub date: Option<DateTime>,
    pub copyright: Option<String>,
}

impl SourceData {
    /// Parse a SOUR record
    pub fn parse(mut buffer: &str) -> (&str, Option<SourceData>) {
        let mut data = SourceData {
            name: None,
            date: None,
            copyright: None,
        };
        let mut line: Line;

        (buffer, line) = parse::peek_line(buffer).unwrap();
        if line.tag == "DATA" {
            let lvl = line.level;

            // consume the line
            (buffer, line) = parse::line(buffer).unwrap();
            data.name = Some(line.value.to_string());

            while line.level >= lvl {
                if buffer.is_empty() {
                    break;
                }

                (buffer, line) = parse::peek_line(buffer).unwrap();
                if line.level == 1 {
                    // abort
                    break;
                }
                match line.tag {
                    "DATE" => {
                        (buffer, data.date) = DateTime::parse(buffer);
                    }
                    "COPR" => {
                        // Consume the line and get the value
                        // (buffer, line) = parse::line(buffer).unwrap();
                        // (buffer, data.copyright) = Copyright::parse(buffer);
                        (buffer, data.copyright) = parse::get_tag_value(buffer).unwrap();
                    }
                    _ => {
                        break;
                    }
                }
            }
        }

        (buffer, Some(data))
    }
}

#[cfg(test)]
mod tests {
    use super::SourceData;
    use crate::types::DateTime;

    #[test]
    fn parse() {
        let data = vec![
            "2 DATA Name of source data",
            "3 DATE 1 JAN 1998",
            "3 COPR Copyright of source data",
        ];

        let (_data, _sourcedata) = SourceData::parse(&data.join("\n"));
        let sourcedata = _sourcedata.unwrap();

        assert_eq!(
            Some(sourcedata),
            Some(SourceData {
                name: Some("Name of source data".to_string()),
                date: Some(DateTime {
                    date: Some("1 JAN 1998".to_string()),
                    time: None
                }),
                copyright: Some("Copyright of source data".to_string()),
            })
        );
    }
}
