use std::collections::HashMap;

use super::utils;

pub fn solve() {
    println!("Day 11: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
}

fn part1() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day11.txt").as_str())
        .expect("failed to read file content");

    let mut stones: Vec<i64> = Vec::new();
    for word in text.split(' ') {
        let word_len = word.len() - 1;
        let mut temp_word = word;
        if word.contains("\n") {
            temp_word = &temp_word[0..word_len];
        }

        stones.push(temp_word.parse().unwrap());
    }

    fn get_num_digits(n: i64) -> i64 {
        let mut temp = n;
        let mut num_digits = 1;
        loop {
            if temp <= 9 {
                break;
            }
            temp /= 10;
            num_digits += 1;
        }
        num_digits
    }

    fn blink(stones: Vec<i64>) -> Vec<i64> {
        let mut new_stones: Vec<i64> = Vec::new();
        for s in stones {
            if s == 0 {
                new_stones.push(1);
            } else {
                let num_digits = get_num_digits(s);
                if num_digits % 2 == 0 {
                    new_stones.push(s / (10_i64).pow(num_digits as u32 / 2_u32));
                    new_stones.push(s % (10_i64).pow(num_digits as u32 / 2_u32));
                } else {
                    new_stones.push(s * 2024);
                }
            }
        }
        new_stones
    }

    for _i in 0..25 {
        stones = blink(stones);
    }

    stones.len() as i64
}

fn part2() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day11.txt").as_str())
        .expect("failed to read file content");

    let mut stones: HashMap<i64, i64> = HashMap::new();
    for word in text.split(' ') {
        let word_len = word.len() - 1;
        let mut temp_word = word;
        if word.contains("\n") {
            temp_word = &temp_word[0..word_len];
        }

        stones
            .entry(temp_word.parse().unwrap())
            .and_modify(|amount| *amount += 1)
            .or_insert(1);
    }

    fn get_num_digits(n: i64) -> i64 {
        let mut temp = n;
        let mut num_digits = 1;
        loop {
            if temp <= 9 {
                break;
            }
            temp /= 10;
            num_digits += 1;
        }
        num_digits
    }

    fn blink(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
        let mut new_stones: HashMap<i64, i64> = HashMap::new();
        for (s, amount) in stones {
            if s == 0 {
                new_stones
                    .entry(1)
                    .and_modify(|new_amount| *new_amount += amount)
                    .or_insert(amount);
            } else {
                let num_digits = get_num_digits(s);
                if num_digits % 2 == 0 {
                    new_stones
                        .entry(s / (10_i64).pow(num_digits as u32 / 2_u32))
                        .and_modify(|new_amount| *new_amount += amount)
                        .or_insert(amount);
                    new_stones
                        .entry(s % (10_i64).pow(num_digits as u32 / 2_u32))
                        .and_modify(|new_amount| *new_amount += amount)
                        .or_insert(amount);
                } else {
                    new_stones
                        .entry(s * 2024)
                        .and_modify(|new_amount| *new_amount += amount)
                        .or_insert(amount);
                }
            }
        }
        new_stones
    }

    for _i in 0..75 {
        stones = blink(stones);
    }

    let mut sum = 0;
    for (_s, amount) in stones {
        sum += amount;
    }

    sum
}
