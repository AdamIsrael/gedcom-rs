use crate::parse;

#[derive(Debug, Default, PartialEq)]
pub struct Note {
    pub value: Option<String>,
}

impl Note {
    pub fn parse(mut buffer: &str) -> (&str, Option<Note>) {
        let mut note = Note { value: None };
        let mut text: String = String::from("");
        let mut line;

        (_, line) = parse::peek_line(buffer).unwrap();
        if line.tag == "NOTE" {
            (buffer, line) = parse::line(buffer).unwrap();
            text += line.value.unwrap_or("");

            (_, line) = parse::peek_line(buffer).unwrap();
            while line.tag == "CONC" || line.tag == "CONT" {
                // consume
                (buffer, line) = parse::line(buffer).unwrap();

                // allocate
                text += line.value.unwrap_or("");
                if line.tag == "CONT" {
                    text += "\n";
                }

                // peek ahead
                (_, line) = parse::peek_line(buffer).unwrap();
            }
        }
        note.value = Some(text);

        (buffer, Some(note))
    }
}

#[cfg(test)]
mod tests {
    use super::Note;

    #[test]
    fn parse() {
        let data = vec![
            "1 NOTE This is the first line of a note ",
            "2 CONC and still the same line...",
            "2 CONC and the end of the first line.",
            "2 CONT",
            "2 CONT And this is a new line.",
        ];

        let (_, note) = Note::parse(&data.join("\n"));

        assert!(note.is_some());
        let note = note.unwrap();
        assert!(note.value.is_some());
        assert!(note
            .value
            .as_ref()
            .unwrap()
            .starts_with("This is the first line"));
        assert!(note
            .value
            .as_ref()
            .unwrap()
            .ends_with("And this is a new line.\n"));
    }
}
