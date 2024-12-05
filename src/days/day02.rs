use super::super::utils;

pub fn solve() {
    println!("Day 2: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
    println!();
}

fn part1() -> i32 {
    let text = utils::read_file_content_as_string(utils::get_path("day02.txt").as_str())
        .expect("failed to read file content");

    let mut num_safe = 0;
    for line in text.lines() {
        let whitespace_iter = line.split_whitespace();

        let mut is_safe = true;
        let mut last_num: Option<i32> = None;
        let mut increasing: Option<bool> = None;
        for num_str in whitespace_iter {
            let num: i32 = num_str.parse().expect("failed to parse string as integer");

            if let Some(ref last_num_value) = last_num {
                let diff = num - *last_num_value;
                let diff_abs = diff.abs();

                if !(1..=3).contains(&diff_abs) {
                    is_safe = false;
                    break;
                }

                if increasing.is_none() {
                    increasing = Some(diff > 0);
                } else if let Some(ref increasing_value) = increasing {
                    if (*increasing_value && diff < 0) || (!(*increasing_value) && diff > 0) {
                        is_safe = false;
                        break;
                    }
                }
            }

            last_num = Some(num)
        }

        if is_safe {
            num_safe += 1;
        }
    }

    num_safe
}

fn part2() -> i32 {
    let text = utils::read_file_content_as_string(utils::get_path("day02.txt").as_str())
        .expect("failed to read file content");

    let mut num_safe = 0;
    for line in text.lines() {
        let whitespace_iter = line.split_whitespace();

        let mut is_safe = true;
        let mut last_num: Option<i32> = None;
        let mut increasing: Option<bool> = None;
        let mut one_unsafe = false;
        for num_str in whitespace_iter {
            let num: i32 = num_str.parse().expect("failed to parse string as integer");

            if let Some(ref last_num_value) = last_num {
                let diff = num - *last_num_value;
                let diff_abs = diff.abs();

                if !(1..=3).contains(&diff_abs) {
                    if one_unsafe {
                        is_safe = false;
                        break;
                    } else {
                        one_unsafe = true;
                        last_num = Some(num);
                        continue;
                    }
                }

                if increasing.is_none() {
                    increasing = Some(diff > 0);
                } else if let Some(ref increasing_value) = increasing {
                    if (*increasing_value && diff < 0) || (!(*increasing_value) && diff > 0) {
                        if one_unsafe {
                            is_safe = false;
                            break;
                        } else {
                            one_unsafe = true;
                            last_num = Some(num);
                            continue;
                        }
                    }
                }
            }

            last_num = Some(num)
        }

        if is_safe {
            num_safe += 1;
        }
    }

    num_safe
}
