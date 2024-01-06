use std::collections::BTreeMap;
use std::ops::Range;

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    name: String,
    lines: Vec<Vec<u64>>
}

impl Map {
    fn new (name: &str, unprocessed_lines: &[&str]) -> Map {
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
    correlation_map: BTreeMap<u64, u64>,
}

impl RangeMap {
    fn new(name: String) -> RangeMap {
        RangeMap {
            name,
            correlation_map: BTreeMap::new(),
        }
    }

    fn add_ranges(&mut self, start: Range<u64>, end: Range<u64>) {
        for (s, e) in start.zip(end) {
            self.correlation_map.insert(s, e);
        }
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
        for inst in map.lines.iter() {
            let range = inst[2];
            let start = inst[1]..inst[1] + range;
            let end = inst[0]..inst[0] + range;
            range_map.add_ranges(start, end);
        }
        range_maps.push(range_map);
    }

    let mut final_locations: Vec<u64> = vec![];

    for seed in &seeds  {
        let mut location = seed;
        println!("seed: {:#?}", seed);

        for identifier in &identifiers  {
            println!("identifier: {:#?}", identifier);
            println!("location: {:#?}", location);
            if let Some(range_map) = range_maps.iter().find(|&r| r.name == *identifier) {
                if let Some(v) = range_map.correlation_map.get(location) {
                    location = v;
                }
            } else {
                panic!("ERROR: No matching range map with current identifier !")
            }
        }

        final_locations.push(*location);
    }

    println!("seeds: {:#?}", seeds);
    println!("maps: {:#?}", maps);
    println!("ranges_maps: {:#?}", range_maps);
    println!("final_locations: {:#?}", final_locations);
    println!("lowest final location: {:#?}", final_locations.iter().min());
}