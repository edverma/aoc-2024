use super::utils;
use regex::Regex;

pub fn solve() {
    println!("Day 7: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
    println!();
}

fn part1() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day07.txt").as_str())
        .expect("failed to read file content");

    let mut sum = 0;
    let re_res = Regex::new(r"(\d+):").unwrap();
    let re_eq = Regex::new(r" \d+").unwrap();
    for line in text.lines() {
        let res_caps = re_res.captures(line).unwrap();
        let res_num: i64 = res_caps[1].parse().unwrap();

        let eq_nums: Vec<i64> = re_eq
            .find_iter(line)
            .map(|m| {
                let num: i64;
                let s = &m.as_str()[1..];
                num = s.parse().unwrap();
                num
            })
            .collect();

        let chars: [char; 2] = ['+', '*'];
        let mut operations: Vec<Vec<char>> = Vec::new();
        let mut current: Vec<char> = Vec::new();
        let size = eq_nums.len() - 1;

        fn fill_ops(
            operations: &mut Vec<Vec<char>>,
            current: &mut Vec<char>,
            size: usize,
            chars: [char; 2],
        ) {
            if current.len() == size {
                operations.push(current.clone());
                return;
            }

            for c in chars {
                current.push(c);
                fill_ops(operations, current, size, chars);
                current.pop();
            }
        }

        fill_ops(&mut operations, &mut current, size, chars);

        for op in operations {
            let mut val: i64 = eq_nums[0];
            for i in 1..eq_nums.len() {
                if op[i - 1] == '+' {
                    val += eq_nums[i]
                } else if op[i - 1] == '*' {
                    if let Some(v) = val.checked_mul(eq_nums[i]) {
                        val = v;
                    }
                }
            }

            if val == res_num {
                sum += res_num;
                break;
            }
        }
    }

    sum
}

fn part2() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day07.txt").as_str())
        .expect("failed to read file content");

    let mut sum = 0;
    let re_res = Regex::new(r"(\d+):").unwrap();
    let re_eq = Regex::new(r" \d+").unwrap();
    for line in text.lines() {
        let res_caps = re_res.captures(line).unwrap();
        let res_num: i64 = res_caps[1].parse().unwrap();

        let eq_nums: Vec<i64> = re_eq
            .find_iter(line)
            .map(|m| {
                let num: i64;
                let s = &m.as_str()[1..];
                num = s.parse().unwrap();
                num
            })
            .collect();

        let chars: [char; 3] = ['+', '*', '|'];
        let mut operations: Vec<Vec<char>> = Vec::new();
        let mut current: Vec<char> = Vec::new();
        let size = eq_nums.len() - 1;

        fn fill_ops(
            operations: &mut Vec<Vec<char>>,
            current: &mut Vec<char>,
            size: usize,
            chars: [char; 3],
        ) {
            if current.len() == size {
                operations.push(current.clone());
                return;
            }

            for c in chars {
                current.push(c);
                fill_ops(operations, current, size, chars);
                current.pop();
            }
        }

        fill_ops(&mut operations, &mut current, size, chars);

        for op in operations {
            let mut val: i64 = eq_nums[0];
            for i in 1..eq_nums.len() {
                if op[i - 1] == '+' {
                    val += eq_nums[i]
                } else if op[i - 1] == '*' {
                    if let Some(v) = val.checked_mul(eq_nums[i]) {
                        val = v;
                    }
                } else if op[i - 1] == '|' {
                    val = do_concat(val, eq_nums[i]);
                }
            }

            if val == res_num {
                sum += res_num;
                break;
            }
        }
    }

    sum
}

fn do_concat(num1: i64, num2: i64) -> i64 {
    let str1 = num1.to_string();
    let str2 = num2.to_string();
    let num: String = str1 + str2.as_str();
    num.parse().unwrap()
}
