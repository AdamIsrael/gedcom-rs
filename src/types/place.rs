/// The Place structure
use crate::types::{Line, Map, Note};

use winnow::prelude::*;

// PLACE_STRUCTURE:=
// n PLAC <PLACE_NAME> {1:1} p.58
// +1 FORM <PLACE_HIERARCHY> {0:1} p.58
// +1 FONE <PLACE_PHONETIC_VARIATION> {0:M} p.59
// +2 TYPE <PHONETIC_TYPE> {1:1} p.57
// +1 ROMN <PLACE_ROMANIZED_VARIATION> {0:M} p.59
// +2 TYPE <ROMANIZED_TYPE> {1:1} p.61
// +1 MAP {0:1}
// +2 LATI <PLACE_LATITUDE> {1:1} p.58
// +2 LONG <PLACE_LONGITUDE> {1:1} p.58
// +1 <<NOTE_STRUCTURE>> {0:M} p.37

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Place {
    pub name: Option<String>,
    pub form: Vec<String>,
    pub phonetic: Option<PlaceVariation>,
    pub roman: Option<PlaceVariation>,
    pub map: Option<Map>,
    pub note: Option<Note>,
}

impl Place {
    pub fn parse(record: &mut &str) -> PResult<Place> {
        let mut place = Place::default();

        let Ok(level_line) = Line::peek(record) else {
            return Ok(place);
        };
        let level = level_line.level;

        while !record.is_empty() {
            let mut parse = true;
            let Ok(mut line) = Line::peek(record) else {
                break;
            };
            match line.tag {
                "FORM" => {
                    // TODO: implement this
                    // Per the spec, "This usage is not common and, therefore, not encouraged.
                    // It should only be used when a system has over-structured its place-names."

                    // Parse the value of the line as a comma-delimited list
                    place.form = line.value.split(',').map(|s| s.to_string()).collect();
                }
                "PLAC" => {
                    place.name = Some(line.value.to_string());
                }
                "FONE" => {
                    if let Ok(phonetic) = PlaceVariation::parse(record) {
                        place.phonetic = Some(phonetic);
                    }
                    parse = false;
                }
                "ROMN" => {
                    if let Ok(roman) = PlaceVariation::parse(record) {
                        place.roman = Some(roman);
                    }
                    parse = false;
                }
                "MAP" => {
                    if let Ok(map) = Map::parse(record) {
                        place.map = Some(map);
                    }
                    parse = false;
                }
                "NOTE" => {
                    if let Ok(note) = Note::parse(record) {
                        place.note = Some(note);
                    }
                    parse = false;
                }
                _ => {}
            }

            // If we need to, advance our position in the stream
            if parse {
                let _ = Line::parse(record);
            }

            // If the next level matches our initial level, we're done parsing
            // this structure.
            let Ok(peek_line) = Line::peek(record) else {
                break;
            };
            line = peek_line;
            if line.level == level {
                break;
            }
        }

        Ok(place)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlaceVariation {
    pub name: Option<String>,
    pub r#type: Option<String>,
}
impl PlaceVariation {
    pub fn parse(record: &mut &str) -> PResult<PlaceVariation> {
        let mut variation = PlaceVariation::default();
        let Ok(level_line) = Line::peek(record) else {
            return Ok(variation);
        };
        let level = level_line.level;

        while !record.is_empty() {
            let Ok(mut line) = Line::parse(record) else {
                break;
            };
            match line.tag {
                "FONE" => {
                    variation.name = Some(line.value.to_string());
                }
                "ROMN" => {
                    variation.name = Some(line.value.to_string());
                }
                "TYPE" => {
                    variation.r#type = Some(line.value.to_string());
                }
                _ => {}
            }

            // If the next level matches our initial level, we're done parsing
            // this structure.
            let Ok(peek_line) = Line::peek(record) else {
                break;
            };
            line = peek_line;
            if line.level == level {
                break;
            }
        }
        Ok(variation)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_place() {
        let data = vec![
            "2 PLAC Salt Lake City, UT, USA",
            "3 FORM parish, county, country",
            "3 FONE Salt Lake City, UT, USA",
            "4 TYPE user defined",
            "3 ROMN Salt Lake City, UT, USA",
            "4 TYPE user defined",
            "3 MAP",
            "4 LATI N0",
            "4 LONG E0",
            "3 NOTE Place note",
        ];

        let input = data.join("\n");
        let mut record = input.as_str();
        let place = Place::parse(&mut record).unwrap();

        assert!(place.name.is_some());
        assert!(place.name.unwrap() == "Salt Lake City, UT, USA");

        assert!(place.form.len() == 3);

        let phonetic = place.phonetic.unwrap();
        assert!(phonetic.name == Some("Salt Lake City, UT, USA".to_string()));
        assert!(phonetic.r#type == Some("user defined".to_string()));

        let roman = place.roman.unwrap();
        assert!(roman.name == Some("Salt Lake City, UT, USA".to_string()));
        assert!(roman.r#type == Some("user defined".to_string()));

        assert!(place.map.is_some());
        let map = place.map.unwrap();
        assert!(map.latitude == 0.0);
        assert!(map.longitude == 0.0);
    }

    #[test]
    fn parse_variation() {
        let data = vec!["3 FONE Salt Lake City, UT, USA", "4 TYPE user defined"];

        let input = data.join("\n");
        let mut record = input.as_str();
        let variation = PlaceVariation::parse(&mut record).unwrap();

        assert!(variation.name.is_some());
        assert!(variation.r#type.is_some());

        assert!(variation.name.unwrap() == "Salt Lake City, UT, USA");
        assert!(variation.r#type.unwrap() == "user defined");
    }

    #[test]
    fn test_parse_place_simple() {
        let data = vec!["2 PLAC London, England"];
        let input = data.join("\n");
        let mut record = input.as_str();
        let place = Place::parse(&mut record).unwrap();

        assert_eq!(place.name, Some("London, England".to_string()));
        assert!(place.form.is_empty());
        assert!(place.phonetic.is_none());
        assert!(place.roman.is_none());
        assert!(place.map.is_none());
    }

    #[test]
    fn test_parse_place_with_form() {
        let data = vec!["2 PLAC London", "3 FORM city,country"];
        let input = data.join("\n");
        let mut record = input.as_str();
        let place = Place::parse(&mut record).unwrap();

        assert_eq!(place.name, Some("London".to_string()));
        assert_eq!(place.form.len(), 2);
        assert_eq!(place.form[0], "city");
        assert_eq!(place.form[1], "country");
    }

    #[test]
    fn test_parse_place_default() {
        let place = Place::default();
        assert!(place.name.is_none());
        assert!(place.form.is_empty());
        assert!(place.phonetic.is_none());
        assert!(place.roman.is_none());
        assert!(place.map.is_none());
    }

    #[test]
    fn test_place_variation_default() {
        let variation = PlaceVariation::default();
        assert!(variation.name.is_none());
        assert!(variation.r#type.is_none());
    }

    #[test]
    fn test_parse_place_empty_input() {
        let mut record = "";
        let place = Place::parse(&mut record).unwrap();
        assert_eq!(place, Place::default());
    }

    #[test]
    fn test_place_clone() {
        let place1 = Place {
            name: Some("Test".to_string()),
            form: vec!["city".to_string()],
            phonetic: None,
            roman: None,
            map: None,
            note: None,
        };
        let place2 = place1.clone();
        assert_eq!(place1, place2);
    }

    #[test]
    fn test_place_variation_clone() {
        let var1 = PlaceVariation {
            name: Some("Test".to_string()),
            r#type: Some("type".to_string()),
        };
        let var2 = var1.clone();
        assert_eq!(var1, var2);
    }
}
