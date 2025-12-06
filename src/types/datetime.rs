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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_only() {
        let input = "1 DATE 1 JAN 2000\n";
        let (remaining, dt) = DateTime::parse(input);

        assert!(remaining.is_empty());
        let dt = dt.unwrap();
        assert_eq!(dt.date, Some("1 JAN 2000".to_string()));
        assert_eq!(dt.time, None);
    }

    #[test]
    fn test_parse_date_with_time() {
        let input = "1 DATE 1 JAN 2000\n2 TIME 14:30:00\n";
        let (remaining, dt) = DateTime::parse(input);

        assert!(remaining.is_empty());
        let dt = dt.unwrap();
        assert_eq!(dt.date, Some("1 JAN 2000".to_string()));
        assert_eq!(dt.time, Some("14:30:00".to_string()));
    }

    #[test]
    fn test_parse_no_date() {
        let input = "1 NAME Test\n";
        let (remaining, dt) = DateTime::parse(input);

        // Buffer should not be consumed
        assert_eq!(remaining, input);
        let dt = dt.unwrap();
        assert_eq!(dt.date, None);
        assert_eq!(dt.time, None);
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";
        let (_remaining, dt) = DateTime::parse(input);

        let dt = dt.unwrap();
        assert_eq!(dt.date, None);
        assert_eq!(dt.time, None);
    }

    #[test]
    fn test_parse_date_with_following_tag() {
        let input = "1 DATE 1 JAN 2000\n1 PLAC London\n";
        let (remaining, dt) = DateTime::parse(input);

        // Should stop at PLAC line
        assert!(remaining.contains("PLAC"));
        let dt = dt.unwrap();
        assert_eq!(dt.date, Some("1 JAN 2000".to_string()));
        assert_eq!(dt.time, None);
    }

    #[test]
    fn test_parse_date_time_with_following_tag() {
        let input = "1 DATE 1 JAN 2000\n2 TIME 14:30:00\n1 PLAC London\n";
        let (remaining, dt) = DateTime::parse(input);

        assert!(remaining.contains("PLAC"));
        let dt = dt.unwrap();
        assert_eq!(dt.date, Some("1 JAN 2000".to_string()));
        assert_eq!(dt.time, Some("14:30:00".to_string()));
    }

    #[test]
    fn test_datetime_eq() {
        let dt1 = DateTime {
            date: Some("1 JAN 2000".to_string()),
            time: Some("12:00:00".to_string()),
        };
        let dt2 = DateTime {
            date: Some("1 JAN 2000".to_string()),
            time: Some("12:00:00".to_string()),
        };
        assert_eq!(dt1, dt2);
    }

    #[test]
    fn test_datetime_clone() {
        let dt1 = DateTime {
            date: Some("1 JAN 2000".to_string()),
            time: None,
        };
        let dt2 = dt1.clone();
        assert_eq!(dt1, dt2);
    }

    #[test]
    fn test_parse_complex_date() {
        let input = "1 DATE BET 1 JAN 2000 AND 31 DEC 2000\n";
        let (_remaining, dt) = DateTime::parse(input);

        let dt = dt.unwrap();
        assert_eq!(dt.date, Some("BET 1 JAN 2000 AND 31 DEC 2000".to_string()));
    }

    #[test]
    fn test_parse_time_wrong_level() {
        // TIME at same level as DATE should not be captured
        let input = "1 DATE 1 JAN 2000\n1 TIME 14:30:00\n";
        let (remaining, dt) = DateTime::parse(input);

        assert!(remaining.contains("TIME"));
        let dt = dt.unwrap();
        assert_eq!(dt.date, Some("1 JAN 2000".to_string()));
        assert_eq!(dt.time, None); // TIME not captured because it's at wrong level
    }
}
