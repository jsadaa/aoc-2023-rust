use std::collections::HashMap;
use regex::Regex;
use std::str::Lines;

pub(crate) fn day_3_part_1() {
    let lines: Lines = include_str!("../../data/day3.txt")
        .lines();

    let mut parts_nums: Vec<i32> = Vec::new();
    let reg_num = Regex::new("\\d+").unwrap();
    let mut nums_pos: Vec<(i32, usize, usize, usize)> = Vec::new();
    let mut syms_pos: Vec<(char, usize, usize)> = Vec::new();

    for (i, line) in lines.enumerate() {
        for mat in reg_num.find_iter(line) {
            let num = mat.as_str().parse::<i32>().unwrap();
            nums_pos.push((num, i, mat.start(), mat.end()));
        }
        for (j, ch) in line.chars().enumerate() {
            if !ch.is_alphanumeric() && ch != '.' && ch != ' ' {
                syms_pos.push((ch, i, j));
            }
        }
    }

    let adj_on_line_if_start_n_0 =
        |index_n, index_s, start_s, end_n|
            index_n == index_s && start_s == end_n;
    let adj_on_line =
        |index_n, index_s, start_n, start_s, end_n|
            index_n == index_s && (start_s == start_n - 1 || start_s == end_n);
    let adj_bel_if_start_n_0 =
        |index_n, index_s, start_n, start_s, end_n|
            index_s == index_n + 1 && (start_s >= start_n && start_s <= end_n);
    let adj_bel =
        |index_n, index_s, start_n, start_s, end_n|
            index_s == index_n + 1 && (start_s >= start_n - 1 && start_s <= end_n);
    let adj_abv_if_start_n_0 =
        |index_n, index_s, start_n, start_s, end_n|
            index_s == index_n - 1  && (start_s >= start_n && start_s <= end_n);
    let adj_abv =
        |index_n, index_s, start_n, start_s, end_n|
            index_s == index_n - 1 && (start_s >= start_n - 1 && start_s <= end_n);

    for num in &nums_pos {
        if syms_pos.iter().any(|sym| {

            let (_, index_n, start_n, end_n) = *num;
            let (_, index_s, start_s) = *sym;

            let is_start_n_0 = start_n < 1;
            let adj_on_line = if is_start_n_0 {
                adj_on_line_if_start_n_0(index_n, index_s, start_s, end_n)
            } else {
                adj_on_line(index_n, index_s, start_n, start_s, end_n)
            };

            let adj_bel = if is_start_n_0 {
                adj_bel_if_start_n_0(index_n, index_s, start_n, start_s, end_n)
            } else {
                adj_bel(index_n, index_s, start_n, start_s, end_n)
            };

            let adj_abv = if index_n < 1 {
                false
            } else if is_start_n_0 {
                adj_abv_if_start_n_0(index_n, index_s, start_n, start_s, end_n)
            } else {
                adj_abv(index_n, index_s, start_n, start_s, end_n)
            };

            adj_on_line || adj_bel || adj_abv
        }) {
            parts_nums.push(num.0);
        }
    }

    println!("Sum of part numbers : {:#?}", parts_nums.iter().sum::<i32>());
}

pub(crate) fn day_3_part_2() {
    let lines: Lines = include_str!("../../data/day3.txt")
        .lines();

    let mut adj_records: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let reg_num = Regex::new("\\d+").unwrap();
    let mut nums_pos: Vec<(i32, usize, usize, usize)> = Vec::new();
    let mut syms_pos: Vec<(char, usize, usize)> = Vec::new();
    let mut gear_ratios_sum = 0;

    for (i, line) in lines.enumerate() {
        for mat in reg_num.find_iter(line) {
            let num = mat.as_str().parse::<i32>().unwrap();
            nums_pos.push((num, i, mat.start(), mat.end()));
        }
        for (j, ch) in line.chars().enumerate() {
            if !ch.is_alphanumeric() && ch != '.' && ch != ' ' {
                syms_pos.push((ch, i, j));
            }
        }
    }

    let adj_on_line_if_start_n_0 =
        |index_n, index_s, start_s, end_n|
            index_n == index_s && start_s == end_n;
    let adj_on_line =
        |index_n, index_s, start_n, start_s, end_n|
            index_n == index_s && (start_s == start_n - 1 || start_s == end_n);
    let adj_bel_if_start_n_0 =
        |index_n, index_s, start_n, start_s, end_n|
            index_s == index_n + 1 && (start_s >= start_n && start_s <= end_n);
    let adj_bel =
        |index_n, index_s, start_n, start_s, end_n|
            index_s == index_n + 1 && (start_s >= start_n - 1 && start_s <= end_n);
    let adj_abv_if_start_n_0 =
        |index_n, index_s, start_n, start_s, end_n|
            index_s == index_n - 1  && (start_s >= start_n && start_s <= end_n);
    let adj_abv =
        |index_n, index_s, start_n, start_s, end_n|
            index_s == index_n - 1 && (start_s >= start_n - 1 && start_s <= end_n);

    for num in &nums_pos {
        for sym in &syms_pos {
            let (number, index_n, start_n, end_n) = *num;
            let (_, index_s, start_s) = *sym;

            let is_start_n_0 = start_n < 1;
            let adj_on_line = if is_start_n_0 {
                adj_on_line_if_start_n_0(index_n, index_s, start_s, end_n)
            } else {
                adj_on_line(index_n, index_s, start_n, start_s, end_n)
            };

            let adj_bel = if is_start_n_0 {
                adj_bel_if_start_n_0(index_n, index_s, start_n, start_s, end_n)
            } else {
                adj_bel(index_n, index_s, start_n, start_s, end_n)
            };

            let adj_abv = if index_n < 1 {
                false
            } else if is_start_n_0 {
                adj_abv_if_start_n_0(index_n, index_s, start_n, start_s, end_n)
            } else {
                adj_abv(index_n, index_s, start_n, start_s, end_n)
            };

            let is_adj = adj_on_line || adj_bel || adj_abv;
            if is_adj {
                let key = (index_s, start_s);

                match adj_records.get(&key) {
                    Some(nums) => {
                        let mut new_nums = nums.clone();
                        new_nums.push(number);
                        adj_records.insert(key, new_nums);
                    },
                    None => {
                        adj_records.insert(key, vec![number]);
                    }
                }
            }
        }
    }

    for (_pos, nums) in adj_records.iter() {
        if nums.len() == 2 {
            gear_ratios_sum += nums.first().unwrap() * nums.last().unwrap();
        }
    }

    println!("Sum of gear ratios : {:#?}", gear_ratios_sum);
}