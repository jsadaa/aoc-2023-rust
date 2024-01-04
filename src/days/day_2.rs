use std::cmp::max;
use std::collections::HashMap;
use std::str::Lines;

pub(crate) fn day_2_part_1() {
    let lines: Lines = include_str!("../../data/day2.txt")
        .lines();

    const RED_COUNT: i32 = 12;
    const GREEN_COUNT: i32 = 13;
    const BLUE_COUNT: i32 = 14;

    let mut index_count: i32 = 0;

    for (i, line) in lines.enumerate() {
        let record: &str =
            line
                .split(':')
                .last()
                .unwrap();

        let subsets: Vec<&str> =
            record
                .split(';')
                .collect::<Vec<&str>>();

        let mut game_count_list: HashMap<String, i32> = HashMap::new();

        for subset in &subsets {
            let subset_counts_list: Vec<&str> =
                subset
                    .split(',')
                    .collect::<Vec<&str>>();

            for color_count in &subset_counts_list {
                let color_count_split: Vec<&str> =
                    color_count
                        .split_whitespace()
                        .collect::<Vec<&str>>();

                let count: i32 =
                    color_count_split
                        .first()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();

                let color: String =
                    color_count_split
                        .last()
                        .unwrap()
                        .to_string();

                let count = match game_count_list.get(&color) {
                    Some(c) => max(count, *c),
                    None => count,
                };

                game_count_list.insert(color, count);
            }
        }

        let red_count: i32 = match game_count_list.get("red") {
            Some(c) => *c,
            None => 0,
        };

        let green_count: i32 = match game_count_list.get("green") {
            Some(c) => *c,
            None => 0,
        };

        let blue_count: i32 = match game_count_list.get("blue") {
            Some(c) => *c,
            None => 0,
        };

        if red_count <= RED_COUNT && green_count <= GREEN_COUNT && blue_count <= BLUE_COUNT {
            index_count += (i + 1) as i32;
        }
    }

    println!("Total sum of possible indexes : {:?}", index_count);
}

pub(crate) fn day_2_part_2() {
    let lines: Lines = include_str!("../../data/day2.txt")
        .lines();

    let mut power_sum: i32 = 0;

    for line in lines {
        let record: &str =
            line
                .split(':')
                .last()
                .unwrap();

        let subsets: Vec<&str> =
            record
                .split(';')
                .collect::<Vec<&str>>();

        let mut game_count_list: HashMap<String, i32> = HashMap::new();

        for subset in &subsets {
            let subset_counts_list: Vec<&str> =
                subset
                    .split(',')
                    .collect::<Vec<&str>>();

            for color_count in &subset_counts_list {
                let color_count_split: Vec<&str> =
                    color_count
                        .split_whitespace()
                        .collect::<Vec<&str>>();

                let count: i32 =
                    color_count_split
                        .first()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();

                let color: String =
                    color_count_split
                        .last()
                        .unwrap()
                        .to_string();

                let count = match game_count_list.get(&color) {
                    Some(c) => max(count, *c),
                    None => count,
                };

                game_count_list.insert(color, count);
            }
        }

        let red_count: i32 = match game_count_list.get("red") {
            Some(c) => *c,
            None => 0,
        };

        let green_count: i32 = match game_count_list.get("green") {
            Some(c) => *c,
            None => 0,
        };

        let blue_count: i32 = match game_count_list.get("blue") {
            Some(c) => *c,
            None => 0,
        };

        power_sum += red_count * green_count * blue_count;
    }

    println!("Total sum of sets powers : {:?}", power_sum);
}