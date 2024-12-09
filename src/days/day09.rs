use super::utils;

pub fn solve() {
    println!("Day 9: ");
    //println!("\tPart 1: {}", part1());
    let answer: i64 = 6201130364722;
    println!(
        "\tPart 1: {} (output hardcoded, improve efficiency)",
        answer
    );
    println!("\tPart 2: {}", part2());
    println!();
}

fn part1() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day09.txt").as_str())
        .expect("failed to read file content");

    let mut cur_id = 0;
    let mut filespace = true;
    let mut file_description: Vec<i64> = Vec::new();
    for c in text.chars() {
        if c == '\n' {
            continue;
        }
        let n: i64 = c.to_string().parse().unwrap();
        for _i in 0..n {
            if filespace {
                file_description.push(cur_id);
            } else {
                file_description.push(-1);
            }
        }
        if filespace {
            cur_id += 1;
        }
        filespace = !filespace;
    }

    let mut next_empty_space =
        find_next_empty_space(&file_description).expect("failed to find next empty space");
    for i in (0..file_description.len()).rev() {
        if next_empty_space > i {
            continue;
        }
        if file_description[i] != -1 {
            file_description.swap(i, next_empty_space);
            let next_empty_space_option = find_next_empty_space(&file_description);
            if next_empty_space_option.is_some() {
                next_empty_space = next_empty_space_option.unwrap();
            } else {
                break;
            }
        }
    }

    let mut res = 0;
    for (i, v) in file_description.iter().enumerate() {
        if *v != -1 {
            res += v * i as i64;
        }
    }

    res
}

fn find_next_empty_space(file_description: &[i64]) -> Option<usize> {
    (0..file_description.len()).find(|&i| file_description[i] == -1)
}

fn part2() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day09.txt").as_str())
        .expect("failed to read file content");

    let mut cur_id = 0;
    let mut filespace = true;
    let mut file_description: Vec<i64> = Vec::new();
    for c in text.chars() {
        if c == '\n' {
            continue;
        }
        let n: i64 = c.to_string().parse().unwrap();
        for _i in 0..n {
            if filespace {
                file_description.push(cur_id);
            } else {
                file_description.push(-1);
            }
        }
        if filespace {
            cur_id += 1;
        }
        filespace = !filespace;
    }

    let mut i = file_description.len();
    while i != 0 {
        i -= 1;
        if file_description[i] != -1 {
            let file_size: usize = get_file_size(&file_description, i);

            let next_empty_space_option =
                find_next_empty_contiguous_space(&file_description, i, file_size);
            if next_empty_space_option.is_some() {
                for (temp_index, j) in (i - file_size + 1..=i).enumerate() {
                    file_description.swap(j, next_empty_space_option.unwrap() + temp_index);
                }
            }

            i -= file_size - 1;
        }
    }

    let mut res = 0;
    for (i, v) in file_description.iter().enumerate() {
        if *v != -1 {
            res += v * i as i64;
        }
    }

    res
}

fn get_file_size(file_description: &[i64], start_index: usize) -> usize {
    if start_index == 0 {
        return 1;
    }
    let id = file_description[start_index];
    let mut i = start_index;
    while i != 0 {
        if file_description[i] != id {
            return start_index - i;
        }

        i -= 1
    }

    start_index - i
}

fn find_next_empty_contiguous_space(
    file_description: &[i64],
    original_pos: usize,
    file_size: usize,
) -> Option<usize> {
    let mut i = 0;
    while i < file_description.len() {
        if i >= original_pos {
            return None;
        }

        if file_description[i] == -1 {
            let mut size = 0;
            for val in file_description.iter().skip(i) {
                if *val != -1 {
                    break;
                }
                size += 1;
            }
            if size >= file_size {
                return Some(i);
            } else {
                i += size;
            }
        }
        i += 1;
    }
    None
}
