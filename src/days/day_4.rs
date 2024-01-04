use std::collections::{HashSet, VecDeque};
use std::str::Lines;

pub(crate) fn day_4_part_1() {
    let lines: Lines = include_str!("../../data/day4.txt")
        .lines();

    let mut total_points: i32 = 0;

    for line in lines {
        let nums: &str =
            line
                .split(':')
                .last()
                .unwrap();

        let split_nums: Vec<&str> =
            nums
                .split('|')
                .collect();

        let win_nums: Vec<i32> =
            split_nums
                .first()
                .unwrap()
                .split_whitespace()
                .filter_map(|str| str.parse().ok())
                .collect();

        let nums_u_have: Vec<i32> =
            split_nums
                .last()
                .unwrap()
                .split_whitespace()
                .filter_map(|str| str.parse().ok())
                .collect();

        let mut card_points = 0;

        let mat_nums: Vec<&i32> =
            nums_u_have
                .iter()
                .filter(|num| win_nums.contains(num))
                .collect();

        if !mat_nums.is_empty() {
            card_points = 1;
            for _ in 1..mat_nums.len() {
                card_points *= 2;
            }
        }

        total_points += card_points;
    }

    println!("Cards points : {}", total_points)
}

#[derive(Debug)]
pub struct Card {
    nums: Vec<u32>,
    wins: HashSet<u32>,
    matches: Option<u32>, // Stocker les correspondances calculées
}

impl Card {
    fn new(nums: Vec<u32>, wins: Vec<u32>) -> Card {
        Card {
            nums,
            wins: wins.into_iter().collect(),
            matches: None,
        }
    }

    fn matches(&mut self) {
        let matches = self.nums.iter().filter(|n| self.wins.contains(n)).count() as u32;
        self.matches = Some(matches);
    }
}

pub fn day_4_part_2() {
    let lines: Vec<&str> = include_str!("../../data/day4.txt").lines().collect();

    let mut cards: Vec<Card> = lines.iter()
        .map(|line| {
            let parts: Vec<&str> = line.splitn(2, '|').collect();
            let nums: Vec<u32> = parts[0].split_whitespace().skip(2).map(|n| n.parse().unwrap()).collect();
            let wins: Vec<u32> = parts[1].split_whitespace().map(|n| n.parse().unwrap()).collect();

            Card::new(nums, wins)
        })
        .collect();

    let mut queue: VecDeque<(usize, u32)> = VecDeque::new(); // File d'attente avec index de carte et nombre de copies
    let mut total_cards = 0;

    for i in 0..cards.len() {
        cards[i].matches();
        queue.push_back((i, 1)); // Initialiser avec une copie de chaque carte
    }

    while let Some((i, copies)) = queue.pop_front() {
        let card = &cards[i];
        if let Some(matches) = card.matches {
            for _ in 0..copies {
                for j in 1..=matches as usize {
                    if i + j < cards.len() {
                        queue.push_back((i + j, 1));
                    }
                }
                total_cards += 1;
            }
        }
    }

    println!("Total scratchcards: {}", total_cards);
}
