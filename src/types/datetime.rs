use super::Line;
// use crate::parse;

// +1 DATE <TRANSMISSION_DATE>
// +2 TIME <TIME_VALUE>
#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    pub date: Option<String>,
    pub time: Option<String>,
}

impl DateTime {
    /// Parse the current line(s) for a date/time
    pub fn parse(mut buffer: &str) -> (&str, Option<DateTime>) {
        let mut dt = DateTime {
            date: None,
            time: None,
        };
        let mut line: Line;

        (_, line) = Line::peek(buffer).unwrap();

        // if line.level == 1 && line.tag == "DATE" {
        if line.tag == "DATE" {
            let parent_level = line.level;

            // Consume the line
            (buffer, line) = Line::parse(buffer).unwrap();
            dt.date = Some(line.value.to_string());

            // Check to see if we have time as a child of the date record
            (_, line) = Line::peek(buffer).unwrap();
            if line.level == parent_level + 1 && line.tag == "TIME" {
                // Consume the line
                (buffer, line) = Line::parse(buffer).unwrap();
                dt.time = Some(line.value.to_string());
            }
        }

        (buffer, Some(dt))
    }
}
