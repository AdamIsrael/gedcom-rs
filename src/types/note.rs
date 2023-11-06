pub struct Note {
    pub value: Option<String>,
}
impl Note {
    pub fn parse(mut buffer: &str) -> (&str, Option<Note>) {
        let note = Note { value: None };

        (_, tag) = parse::peek_tag(buffer).unwrap_or(("", ""));

        while tag == "CONT" || tag == "CONC" {
            if tag == "CONT" {
                let (asdf, cont) = parse::cont(buffer).unwrap();

                addr += "\n";

                addr += cont;

                buffer = asdf;
            } else if tag == "CONC" {
                let (asdf, cont) = parse::conc(buffer).unwrap();
                addr += " ";
                addr += cont;
                buffer = asdf;
            }

            (_, tag) = parse::peek_tag(buffer).unwrap();
        }

        (buffer, Some(note))
    }
}
