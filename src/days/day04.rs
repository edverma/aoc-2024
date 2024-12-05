use super::utils;

pub fn solve() {
    println!("Day 4: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
    println!();
}

fn part1() -> i32 {
    let text = utils::read_file_content_as_string(utils::get_path("day04.txt").as_str())
        .expect("failed to read file content");

    let mut text_vec: Vec<&str> = Vec::new();
    for line in text.lines() {
        text_vec.push(line);
    }

    let mut res = 0;
    for (i, line) in text_vec.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'X' {
                res += get_num_xmas_strings(&text_vec, i, j);
            }
        }
    }

    res
}

fn get_num_xmas_strings(text_vec: &[&str], i: usize, j: usize) -> i32 {
    let mut chars: Vec<Vec<char>> = Vec::new();

    if i >= 3 {
        chars.push(vec![
            text_vec
                .get(i - 1)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j)
                .unwrap_or(' '),
            text_vec
                .get(i - 2)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j)
                .unwrap_or(' '),
            text_vec
                .get(i - 3)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j)
                .unwrap_or(' '),
        ]);

        if j >= 3 {
            chars.push(vec![
                text_vec
                    .get(i - 1)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j - 1)
                    .unwrap_or(' '),
                text_vec
                    .get(i - 2)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j - 2)
                    .unwrap_or(' '),
                text_vec
                    .get(i - 3)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j - 3)
                    .unwrap_or(' '),
            ]);
        }

        if j < text_vec[i].len() - 3 {
            chars.push(vec![
                text_vec
                    .get(i - 1)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j + 1)
                    .unwrap_or(' '),
                text_vec
                    .get(i - 2)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j + 2)
                    .unwrap_or(' '),
                text_vec
                    .get(i - 3)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j + 3)
                    .unwrap_or(' '),
            ]);
        }
    }

    if i < text_vec.len() - 3 {
        chars.push(vec![
            text_vec
                .get(i + 1)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j)
                .unwrap_or(' '),
            text_vec
                .get(i + 2)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j)
                .unwrap_or(' '),
            text_vec
                .get(i + 3)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j)
                .unwrap_or(' '),
        ]);

        if j >= 3 {
            chars.push(vec![
                text_vec
                    .get(i + 1)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j - 1)
                    .unwrap_or(' '),
                text_vec
                    .get(i + 2)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j - 2)
                    .unwrap_or(' '),
                text_vec
                    .get(i + 3)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j - 3)
                    .unwrap_or(' '),
            ]);
        }

        if j < text_vec[i].len() - 3 {
            chars.push(vec![
                text_vec
                    .get(i + 1)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j + 1)
                    .unwrap_or(' '),
                text_vec
                    .get(i + 2)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j + 2)
                    .unwrap_or(' '),
                text_vec
                    .get(i + 3)
                    .unwrap_or(&String::from("").as_str())
                    .chars()
                    .nth(j + 3)
                    .unwrap_or(' '),
            ])
        }
    }

    if j >= 3 {
        chars.push(vec![
            text_vec
                .get(i)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j - 1)
                .unwrap_or(' '),
            text_vec
                .get(i)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j - 2)
                .unwrap_or(' '),
            text_vec
                .get(i)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j - 3)
                .unwrap_or(' '),
        ]);
    }

    if j < text_vec[i].len() - 3 {
        chars.push(vec![
            text_vec
                .get(i)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j + 1)
                .unwrap_or(' '),
            text_vec
                .get(i)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j + 2)
                .unwrap_or(' '),
            text_vec
                .get(i)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j + 3)
                .unwrap_or(' '),
        ]);
    }

    let mut s_vec: Vec<String> = Vec::new();
    for v in chars {
        let s = String::from_iter(v.iter());
        s_vec.push(s);
    }

    let mut num_xmas_strings = 0;
    for s in s_vec {
        if s == "MAS" {
            num_xmas_strings += 1;
        }
    }

    num_xmas_strings
}

fn get_num_x_mas_strings(text_vec: &[&str], i: usize, j: usize) -> i32 {
    let mut chars: Vec<Vec<char>> = Vec::new();

    if i >= 1 && j >= 1 && i < text_vec.len() - 1 && j < text_vec[i].len() - 1 {
        chars.push(vec![
            text_vec
                .get(i - 1)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j - 1)
                .unwrap_or(' '),
            text_vec
                .get(i + 1)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j + 1)
                .unwrap_or(' '),
        ]);

        chars.push(vec![
            text_vec
                .get(i - 1)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j + 1)
                .unwrap_or(' '),
            text_vec
                .get(i + 1)
                .unwrap_or(&String::from("").as_str())
                .chars()
                .nth(j - 1)
                .unwrap_or(' '),
        ]);
    } else {
        return 0;
    }

    let mut s_vec: Vec<String> = Vec::new();
    for v in chars {
        let s = String::from_iter(v.iter());
        s_vec.push(s);
    }

    for s in s_vec {
        if s != "MS" && s != "SM" {
            return 0;
        }
    }

    1
}

fn part2() -> i32 {
    let text = utils::read_file_content_as_string(utils::get_path("day04.txt").as_str())
        .expect("failed to read file content");

    let mut text_vec: Vec<&str> = Vec::new();
    for line in text.lines() {
        text_vec.push(line);
    }

    let mut res = 0;
    for (i, line) in text_vec.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'A' {
                res += get_num_x_mas_strings(&text_vec, i, j);
            }
        }
    }

    res
}
