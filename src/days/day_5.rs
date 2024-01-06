#[derive(Debug)]
pub struct Map {
    name: String,
    lines: Vec<Vec<u64>>,
}

impl Map {
    fn new(name: &str, unprocessed_lines: &[&str]) -> Map {
        let processed_lines = Map::proc_lines(name, unprocessed_lines);
        Map { name: name.to_string(), lines: processed_lines }
    }

    fn proc_lines(name: &str, lines: &[&str]) -> Vec<Vec<u64>> {
        lines
            .iter()
            .skip_while(|&s| !s.starts_with(name))
            .skip(1)
            .take_while(|&s| !s.is_empty())
            .copied()
            .map(
                |s|
                    s.split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect()
            )
            .collect()
    }
}

#[derive(Debug)]
pub struct RangeMap {
    name: String,
    coordinates: Vec<(u64, u64, u64)>,
}

impl RangeMap {
    fn new(name: String) -> RangeMap {
        RangeMap {
            name,
            coordinates: vec![],
        }
    }

    fn add_coordinates(&mut self, start: u64, end: u64, range: u64) {
        self.coordinates.push((start, end, range))
    }
}

pub(crate) fn day_5_part_1() {
    let lines: Vec<&str> = include_str!("../../data/day5.txt")
        .lines()
        .collect();

    let seeds: Vec<u64> =
        lines[0]
            .split(':')
            .last()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

    let mut maps: Vec<Map> = vec![];
    let identifiers: [&str; 7] =
        [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location"
        ];

    for identifier in &identifiers {
        let map = Map::new(identifier, &lines);
        maps.push(map);
    }

    let mut range_maps: Vec<RangeMap> = vec![];

    for map in &maps {
        let mut range_map = RangeMap::new(map.name.clone());
        for line in map.lines.iter() {
            let start = line[1];
            let end = line[0];
            let range = line[2];
            range_map.add_coordinates(start, end, range);
        }
        range_maps.push(range_map);
    }

    let mut final_locations: Vec<u64> = vec![];

    for seed in &seeds {
        let mut location = *seed;

        for identifier in &identifiers {
            if let Some(range_map) = range_maps.iter().find(|&r| r.name == *identifier) {
                for (start, end, range) in &range_map.coordinates {
                    if location >= *start && location <= *start + range {
                        location = end + (location - start);
                        break;
                    }
                }
            } else {
                panic!("ERROR: No matching range map with current identifier !")
            }
        }

        final_locations.push(location);
    }

    println!("Lowest final location: {:#?}", final_locations.iter().min().unwrap());
}