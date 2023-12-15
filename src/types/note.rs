/// Parse a Note structure
// use super::Line;
use crate::parse;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Note {
    /// The note
    pub note: Option<String>,
}

impl Note {
    pub fn parse(mut buffer: &str) -> (&str, Option<Note>) {
        let mut note = Note { note: None };

        note.note = parse::get_tag_value(&mut buffer).unwrap();

        (buffer, Some(note))
    }
}

#[cfg(test)]
mod tests {
    use super::Note;

    #[test]
    fn parse_note() {
        // 1 NOTE This file demonstrates all tags that are allowed in GEDCOM 5.5. Here are some comments about the HEADER record
        // 2 CONC and comments about where to look for information on the other 9 types of GEDCOM records. Most other records will
        // 2 CONC have their own notes that describe what to look for in that record and what to hope the importing software will find.
        // 2 CONT
        // 2 CONT Many applications will fail to import these notes. The notes are therefore also provided with the files as a plain-text
        // 2 CONC "Read-Me" file.

        let data = vec![
            "1 NOTE This is the first line of a note.",
            "2 CONT This is the second line of a note.",
            "2 CONC This is also on the second line.",
            "2 CONT This line should be the last line.",
        ];

        let (_, note) = Note::parse(data.join("\n").as_str());
        let n = note.unwrap().note.unwrap();

        assert!(n.starts_with("This is the first line of a note.\n"));
        assert!(n.ends_with("the last line."));
        assert!(n == "This is the first line of a note.\nThis is the second line of a note. This is also on the second line.\nThis line should be the last line.");
    }
}
