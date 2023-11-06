use super::Line;
use crate::parse;

#[derive(Debug, Default, PartialEq)]
pub struct Copyright {
    pub copyright: Option<String>,
    pub note: Option<String>,
}

impl Copyright {
    /// Parse the COPR tag
    pub fn parse(mut buffer: &str) -> (&str, Option<Copyright>) {
        let mut copyright = Copyright {
            copyright: None,
            note: None,
        };
        let mut line: Line;

        // There's only one line in a COPR, so consume it and get the value
        (_, line) = parse::peek_line(buffer).unwrap();

        if line.tag == "COPR" {
            (buffer, line) = parse::line(buffer).unwrap();
            copyright.copyright = Some(line.value.unwrap_or("").to_string());
        } else if line.tag == "NOTE" {
            let mut note: String = String::from("");
            (_, line) = parse::peek_line(buffer).unwrap();
            while line.tag == "CONC" || line.tag == "CONT" {
                // consume
                (_, line) = parse::line(buffer).unwrap();

                // allocate
                note += line.value.unwrap_or("");
                if line.tag == "CONT" {
                    note += "\n";
                }

                // peek ahead
                (_, line) = parse::peek_line(buffer).unwrap();
            }
            copyright.note = Some(note);
        }
        // TODO: Check for NOTE, followed by n CONC/CONT lines

        (buffer, Some(copyright))
    }
}

#[cfg(test)]
mod tests {
    use super::Copyright;

    #[test]
    fn parse1() {
        let data = vec![
            "1 COPR © 1997 by H. Eichmann, parts © 1999-2000 by J. A. Nairn.",
            "1 NOTE This file demonstrates all tags that are allowed in GEDCOM 5.5. Here are some comments about the HEADER record",
            "2 CONC and comments about where to look for information on the other 9 types of GEDCOM records. Most other records will",
            "2 CONC have their own notes that describe what to look for in that record and what to hope the importing software will find.",
            "2 CONT",
            "2 CONT Many applications will fail to import these notes. The notes are therefore also provided with the files as a plain-text",
            "2 CONC \"Read-Me\" file.",
        ];

        let (_data, _copr) = Copyright::parse(&data.join("\n"));
        // let (_data, _copr) = Copyright::parse(&data.join("\n"));
        let copr = _copr.unwrap();

        assert!(
            copr.copyright
                == Some("© 1997 by H. Eichmann, parts © 1999-2000 by J. A. Nairn.".to_string())
        );

        // // verify that the entire buffer has been consumed
        // assert!(_data.is_empty())
    }

    #[test]
    fn parse2() {
        let buffer = "3 COPR Copyright of source data\n";
        let (_data, _copr) = Copyright::parse(buffer);
        let copr = _copr.unwrap();

        assert!(copr.copyright == Some("Copyright of source data".to_string()));

        // verify that the entire buffer has been consumed
        assert!(_data.is_empty())
    }
}
