use super::utils;
use std::collections::HashMap;

pub fn solve() {
    println!("Day 1: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
    println!();
}

pub fn part1() -> i32 {
    let text = utils::read_file_content_as_string(utils::get_path("day01.txt").as_str())
        .expect("failed to read file content");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in text.lines() {
        let mut whitespace_iter = line.split_whitespace();
        let num_str_1 = whitespace_iter.next().expect("failed to find number");
        let num_1: i32 = num_str_1
            .parse()
            .expect("failed to parse string as integer");
        list1.push(num_1);

        let num_str_2 = whitespace_iter.next().expect("failed to find number");
        let num_2: i32 = num_str_2
            .parse()
            .expect("failed to parse string as integer");
        list2.push(num_2);
    }

    list1.sort();
    list2.sort();

    let mut diff: i32 = 0;
    for i in 0..list1.len() {
        let temp_diff = list1[i] - list2[i];
        diff += temp_diff.abs();
    }

    diff
}

pub fn part2() -> i32 {
    let path = "/Users/edverma/Development/aoc-2024/inputs/day01.txt";
    let text = utils::read_file_content_as_string(path).expect("failed to read file content");

    let mut left_list = Vec::new();
    let mut right_map = HashMap::new();
    for line in text.lines() {
        let mut whitespace_iter = line.split_whitespace();
        let num_str_1 = whitespace_iter.next().expect("failed to find number");
        let num_1: i32 = num_str_1
            .parse()
            .expect("failed to parse string as integer");
        left_list.push(num_1);

        let num_str_2 = whitespace_iter.next().expect("failed to find number");
        let num_2: i32 = num_str_2
            .parse()
            .expect("failed to parse string as integer");
        *right_map.entry(num_2).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for v in left_list.iter() {
        if let Some(&freq) = right_map.get(v) {
            similarity_score += v * freq;
        }
    }

    similarity_score
}
