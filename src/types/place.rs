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

#[derive(Clone, Debug, Default)]
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
        let mut place = Place {
            name: None,
            form: vec![],
            phonetic: None,
            roman: None,
            map: None,
            note: None,
        };

        let level = Line::peek(record).unwrap().level;
        while !record.is_empty() {
            let mut parse = true;
            let mut line = Line::peek(record).unwrap();
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
                    place.phonetic = Some(PlaceVariation::parse(record).unwrap());
                    parse = false;
                }
                "ROMN" => {
                    place.roman = Some(PlaceVariation::parse(record).unwrap());
                    parse = false;
                }
                "MAP" => {
                    place.map = Some(Map::parse(record).unwrap());
                    parse = false;
                }
                "NOTE" => {
                    place.note = Some(Note::parse(record).unwrap());
                    parse = false;
                }
                _ => {}
            }

            // If we need to, advance our position in the stream
            if parse {
                Line::parse(record).unwrap();
            }

            // If the next level matches our initial level, we're done parsing
            // this structure.
            line = Line::peek(record).unwrap();
            if line.level == level {
                break;
            }
        }

        Ok(place)
    }
}

#[derive(Clone, Debug, Default)]
pub struct PlaceVariation {
    pub name: Option<String>,
    pub r#type: Option<String>,
}
impl PlaceVariation {
    pub fn parse(record: &mut &str) -> PResult<PlaceVariation> {
        let mut variation = PlaceVariation {
            name: None,
            r#type: None,
        };
        let level = Line::peek(record).unwrap().level;

        while !record.is_empty() {
            let mut line = Line::parse(record).unwrap();
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
            line = Line::peek(record).unwrap();
            if line.level == level {
                break;
            }
        }
        Ok(variation)
    }
}

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
}
