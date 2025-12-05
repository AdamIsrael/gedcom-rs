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

        if let Ok(l) = Line::peek(&mut buffer) {
            line = l;
        } else {
            return (buffer, Some(dt));
        }

        if line.tag == "DATE" {
            let parent_level = line.level;

            // Consume the line
            if let Ok(l) = Line::parse(&mut buffer) {
                line = l;
                dt.date = Some(line.value.to_string());
            } else {
                return (buffer, Some(dt));
            }

            // Check to see if we have time as a child of the date record
            if let Ok(l) = Line::peek(&mut buffer) {
                line = l;
                if line.level == parent_level + 1 && line.tag == "TIME" {
                    // Consume the line
                    if let Ok(l) = Line::parse(&mut buffer) {
                        line = l;
                        dt.time = Some(line.value.to_string());
                    }
                }
            }
        }

        (buffer, Some(dt))
    }
}
