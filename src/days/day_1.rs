use std::str::Lines;
use regex::Regex;
pub(crate) fn day_1_part_2() {

    let lines: Lines = include_str!("../../data/day1.txt")
        .lines();

    let numbers_to_find: [(&str, i32); 10] = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut total_sum = 0;

    for line in lines {
        let mut matches: Vec<(usize, i32)> = vec![];

        for (word, number) in &numbers_to_find {
            let re_text: Regex = Regex::new(word).unwrap();

            for _match in re_text.find_iter(&line) {
                matches.push((_match.start(), *number));
            }
        }

        for number in 0..10 {
            let re_numeric: Regex = Regex::new(&number.to_string()).unwrap();

            for _match in re_numeric.find_iter(&line) {
                matches.push((_match.start(), number));
            }
        }
        matches.sort_by_key(|m| m.0);
        matches.dedup_by_key(|m| m.0);

        if matches.is_empty() {
            continue;
        }

        let first: i32 = matches.first().unwrap().1;
        let last: i32 = matches.last().unwrap().1;

        let number: i32 = format!("{}{}", first, last).parse::<i32>().unwrap();

        total_sum += number;
    }

    println!("Total sum of extracted numbers : {}", total_sum);
}