use super::utils;
use std::collections::HashMap;

pub fn solve() {
    println!("Day 6: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
    //println!("\tPart 2: {} (output hardcoded, improve efficiency)", 1663);
    println!();
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1() -> i32 {
    let text = utils::read_file_content_as_string(utils::get_path("day06.txt").as_str())
        .expect("failed to read file content");

    let mut cur_pos: (usize, usize) = (0, 0);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        matrix.push(vec![]);
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                cur_pos = (i, j);
            }
            matrix[i].push(c);
        }
    }

    let mut direction: Direction = Direction::Up;
    let mut valid = true;
    while valid {
        match direction {
            Direction::Up => {
                matrix[cur_pos.0][cur_pos.1] = 'X';

                if cur_pos.0 == 0 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0 - 1, cur_pos.1);

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Right;
                } else {
                    cur_pos = next_pos
                }
            }
            Direction::Down => {
                matrix[cur_pos.0][cur_pos.1] = 'X';

                if cur_pos.0 == matrix.len() - 1 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0 + 1, cur_pos.1);

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Left;
                } else {
                    cur_pos = next_pos
                }
            }
            Direction::Right => {
                matrix[cur_pos.0][cur_pos.1] = 'X';

                if cur_pos.1 == matrix[cur_pos.0].len() - 1 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0, cur_pos.1 + 1);

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Down;
                } else {
                    cur_pos = next_pos
                }
            }
            Direction::Left => {
                matrix[cur_pos.0][cur_pos.1] = 'X';

                if cur_pos.1 == 0 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0, cur_pos.1 - 1);

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Up;
                } else {
                    cur_pos = next_pos
                }
            }
        }
    }

    let mut unique_pos_sum = 0;
    for row in matrix {
        for c in row {
            if c == 'X' {
                unique_pos_sum += 1;
            }
        }
    }

    unique_pos_sum
}

fn part2() -> i32 {
    let text = utils::read_file_content_as_string(utils::get_path("day06.txt").as_str())
        .expect("failed to read file content");

    let mut start_pos: (usize, usize) = (0, 0);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        matrix.push(vec![]);
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                start_pos = (i, j);
            }
            matrix[i].push(c);
        }
    }

    let all_guard_coordinates = get_all_guard_coordinates(&mut matrix, start_pos);

    let mut num_unique_blockades = 0;
    let mut cloned_matrix = matrix.clone();
    for (i, row) in matrix.iter().enumerate() {
        for (j, _c) in row.iter().enumerate() {
            if (i, j) == start_pos {
                continue;
            }

            let mut is_guard_coordinate = false;
            for coord in &all_guard_coordinates {
                if *coord == (i, j) {
                    is_guard_coordinate = true;
                    break;
                }
            }
            if !is_guard_coordinate {
                continue;
            }

            if matrix[i][j] == '#' {
                continue;
            }

            cloned_matrix[i][j] = '#';
            let blocked;
            (cloned_matrix, blocked) = check_loop(cloned_matrix, (i, j), start_pos);
            if blocked {
                num_unique_blockades += 1;
            }
            cloned_matrix[i][j] = '.';
        }
    }

    num_unique_blockades
}

fn check_loop(
    matrix: Vec<Vec<char>>,
    new_blockade_pos: (usize, usize),
    start_pos: (usize, usize),
) -> (Vec<Vec<char>>, bool) {
    let mut direction = Direction::Up;
    let mut valid = true;
    let mut dir_map: HashMap<i32, i32> = HashMap::new();

    let mut num_iters = 0;
    let mut cur_pos = start_pos;
    while valid {
        num_iters += 1;
        if num_iters > 6000 {
            return (matrix, true);
        }
        match direction {
            Direction::Up => {
                if cur_pos.0 == 0 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0 - 1, cur_pos.1);
                if (next_pos.0, next_pos.1) == (new_blockade_pos.0, new_blockade_pos.1) {
                    if dir_map.get(&0).is_some() {
                        return (matrix, true);
                    } else {
                        dir_map.insert(0, 1);
                    }
                }

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Right;
                } else {
                    cur_pos = next_pos
                }
            }
            Direction::Down => {
                if cur_pos.0 == matrix.len() - 1 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0 + 1, cur_pos.1);
                if (next_pos.0, next_pos.1) == (new_blockade_pos.0, new_blockade_pos.1) {
                    if dir_map.get(&1).is_some() {
                        return (matrix, true);
                    } else {
                        dir_map.insert(1, 1);
                    }
                }

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Left;
                } else {
                    cur_pos = next_pos
                }
            }
            Direction::Right => {
                if cur_pos.1 == matrix[cur_pos.0].len() - 1 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0, cur_pos.1 + 1);
                if (next_pos.0, next_pos.1) == (new_blockade_pos.0, new_blockade_pos.1) {
                    if dir_map.get(&2).is_some() {
                        return (matrix, true);
                    } else {
                        dir_map.insert(2, 1);
                    }
                }

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Down;
                } else {
                    cur_pos = next_pos
                }
            }
            Direction::Left => {
                if cur_pos.1 == 0 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0, cur_pos.1 - 1);
                if (next_pos.0, next_pos.1) == (new_blockade_pos.0, new_blockade_pos.1) {
                    if dir_map.get(&3).is_some() {
                        return (matrix, true);
                    } else {
                        dir_map.insert(3, 1);
                    }
                }

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Up;
                } else {
                    cur_pos = next_pos
                }
            }
        }
    }

    (matrix, false)
}

fn get_all_guard_coordinates(
    matrix: &mut Vec<Vec<char>>,
    mut cur_pos: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut direction: Direction = Direction::Up;
    let mut valid = true;
    while valid {
        match direction {
            Direction::Up => {
                matrix[cur_pos.0][cur_pos.1] = 'X';

                if cur_pos.0 == 0 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0 - 1, cur_pos.1);

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Right;
                } else {
                    cur_pos = next_pos
                }
            }
            Direction::Down => {
                matrix[cur_pos.0][cur_pos.1] = 'X';

                if cur_pos.0 == matrix.len() - 1 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0 + 1, cur_pos.1);

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Left;
                } else {
                    cur_pos = next_pos
                }
            }
            Direction::Right => {
                matrix[cur_pos.0][cur_pos.1] = 'X';

                if cur_pos.1 == matrix[cur_pos.0].len() - 1 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0, cur_pos.1 + 1);

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Down;
                } else {
                    cur_pos = next_pos
                }
            }
            Direction::Left => {
                matrix[cur_pos.0][cur_pos.1] = 'X';

                if cur_pos.1 == 0 {
                    valid = false;
                    continue;
                }
                let next_pos = (cur_pos.0, cur_pos.1 - 1);

                let next_c = matrix[next_pos.0][next_pos.1];
                if next_c == '#' {
                    direction = Direction::Up;
                } else {
                    cur_pos = next_pos
                }
            }
        }
    }

    let mut all_guard_coordinates: Vec<(usize, usize)> = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'X' {
                all_guard_coordinates.push((i, j));
            }
        }
    }

    all_guard_coordinates
}
