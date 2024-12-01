use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn part1() {
    let path = "/Users/edverma/Development/aoc-2024/src/december_1/input.txt";
    let text = read_file_content_as_string(path).expect("failed to read file content");

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

    println!("{}", diff)
}

pub fn part2() {
    let result: Result<String, Box<dyn Error>> =
        read_file_content_as_string("/Users/edverma/Development/aoc-2024/src/december_1/input.txt");
    let text: String = result.expect("failed to read file content");

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

    println!("{}", similarity_score);
}

fn read_file_content_as_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let string_content: String = fs::read_to_string(path)?;
    Ok(string_content)
}
