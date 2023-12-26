use crate::types::Line;

use std::str::FromStr;

use winnow::prelude::*;

// The GEDCOM specification of this type
//
// +1 MAP {0:1}
// +2 LATI <PLACE_LATITUDE> {1:1} p.58
// +2 LONG <PLACE_LONGITUDE>

#[derive(Clone, Debug, Default)]
pub struct Map {
    pub latitude: f64,
    pub longitude: f64,
}

impl Map {
    /// Parse a map record
    pub fn parse(record: &mut &str) -> PResult<Map> {
        let mut map = Map {
            latitude: 0.0,
            longitude: 0.0,
        };
        let level = Line::peek(record).unwrap().level;

        while !record.is_empty() {
            let mut line = Line::parse(record).unwrap();
            match line.tag {
                "LATI" => {
                    // Need to map this:
                    // N41.913744 -> 41.913744
                    // S41.913744 -> -41.913744
                    map.latitude = f64::from_str(&line.value[1..line.value.len()]).unwrap();
                    if line.value.chars().nth(0) == Some('S') {
                        map.latitude *= -1.0;
                    }
                }
                "LONG" => {
                    // Need to map this:
                    // W88.31085 -> -88.31085
                    // E88.31085 -> 88.31085
                    map.longitude = f64::from_str(&line.value[1..line.value.len()]).unwrap();
                    if line.value.chars().nth(0) == Some('W') {
                        map.longitude *= -1.0;
                    }
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

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_map() {
        let data = vec!["3 MAP", "4 LATI N41.913744", "4 LONG W88.31085"];

        let input = data.join("\n");
        let mut record = input.as_str();
        let map = Map::parse(&mut record).unwrap();

        assert!(map.latitude == 41.913744);
        assert!(map.longitude == -88.31085);
    }
}
