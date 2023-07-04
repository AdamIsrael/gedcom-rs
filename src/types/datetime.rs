use super::Line;
use crate::parse;

// +1 DATE <TRANSMISSION_DATE>
// +2 TIME <TIME_VALUE>
#[derive(Debug, Clone)]
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

        (_, line) = parse::peek_line(buffer).unwrap();

        // if line.level == 1 && line.tag == "DATE" {
        if line.tag == "DATE" {
            let parent_level = line.level;

            // Consume the line
            (buffer, line) = parse::line(buffer).unwrap();
            dt.date = Some(line.value.unwrap_or("").to_string());

            // Check to see if we have time as a child of the date record
            (_, line) = parse::peek_line(buffer).unwrap();
            if line.level == parent_level + 1 && line.tag == "TIME" {
                // Consume the line
                (buffer, line) = parse::line(buffer).unwrap();
                dt.time = Some(line.value.unwrap_or("").to_string());
            }
        }

        (buffer, Some(dt))
    }
}
