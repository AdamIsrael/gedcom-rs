/// A GEDCOM line
/// level + delim (space) + [optional_xref_ID] + tag + [optional_line_value] + terminator
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Line<'a> {
    pub level: u8,
    pub xref: &'a str,
    pub tag: &'a str,
    pub value: &'a str,
}
