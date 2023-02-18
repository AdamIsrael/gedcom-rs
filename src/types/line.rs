/// A GEDCOM line
/// level + delim (space) + [optional_xref_ID] + tag + [optional_line_value] + terminator
#[derive(Debug, Clone)]
pub struct Line {
    // An integer (0-99)
    pub level: u8,

    // Max. 22 characters, including the enclosing @ signs
    pub xref: Option<String>,

    // Max. 32 characters, with the first 15 characters being unique
    pub tag: String,

    pub value: Option<String>,
}
// pub struct Line<'a> {
//     // An integer (0-99)
//     pub level: u8,

//     // Max. 22 characters, including the enclosing @ signs
//     pub xref: Option<&'a str>,

//     // Max. 32 characters, with the first 15 characters being unique
//     pub tag: &'a str,

//     pub value: Option<&'a str>,
// }
