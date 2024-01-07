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

// The below solution for part 2 takes wayyyyy to much time to compute the real input (never saw the output actually)
// It works fine with the example input but i have to find a new approach for computing the real input

use std::collections::{BTreeMap, HashMap};

struct ConversionMap {
    ranges: BTreeMap<u64, (u64, u64)>,
}

impl ConversionMap {
    fn new(lines: &[&str]) -> Self {
        let mut ranges = BTreeMap::new();
        for line in lines {
            let nums: Vec<u64> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if nums.len() == 3 {
                let (start, end, length) = (nums[1], nums[0], nums[2]);
                ranges.insert(start, (end, end + length));
            }
        }
        ConversionMap { ranges }
    }

    fn convert_range(&self, values: &[u64]) -> Vec<u64> {
        values.iter().map(|&value| self.convert(value)).collect()
    }

    fn convert(&self, value: u64) -> u64 {
        if let Some((&start, &(end, range_end))) = self.ranges.range(..=value).next_back() {
            if value < start + (range_end - end) {
                return end + (value - start);
            }
        }
        value
    }
}

fn precalculate_conversions(seeds: &[(u64, u64)], maps: &[ConversionMap]) -> HashMap<u64, u64> {
    let mut conversions = HashMap::new();
    for &(start, length) in seeds {
        let mut current_values: Vec<u64> = (start..start + length).collect();
        for map in maps {
            current_values = map.convert_range(&current_values);
        }
        for &value in &current_values {
            conversions.entry(value).or_insert(value);
        }
    }
    conversions
}

pub fn day_5_part_2() {
    let lines: Vec<&str> = include_str!("../../data/day5.txt")
        .lines()
        .collect();

    let seeds_ranges: Vec<(u64, u64)> = lines[0].split(':').last().unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|slice| (slice[0], slice[1]))
        .collect();

    let conversion_maps: Vec<ConversionMap> = lines.split(|line| line.is_empty())
        .skip(1)
        .map(ConversionMap::new)
        .collect();

    let conversions = precalculate_conversions(&seeds_ranges, &conversion_maps);
    let lowest_location = *conversions.values().min().unwrap_or(&u64::MAX);

    println!("Lowest final location: {}", lowest_location);
}

