use std::collections::{HashMap, HashSet};

use super::utils;

pub fn solve() {
    println!("Day 8: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
    println!();
}

fn part1() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day08.txt").as_str())
        .expect("failed to read file content");

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in text.lines() {
        matrix.push(Vec::new());
        for c in line.chars() {
            if c == '\n' {
                continue;
            }
            let last_elem_index = matrix.len() - 1;
            matrix[last_elem_index].push(c);
        }
    }

    let mut antenna_positions_map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' {
                if let Some(v) = antenna_positions_map.get_mut(c) {
                    v.push((i as i64, j as i64));
                } else {
                    antenna_positions_map.insert(*c, vec![(i as i64, j as i64)]);
                }
            }
        }
    }

    let mut cloned_matrix = matrix.clone();
    let x_len = matrix.len() as i64;
    let y_len = matrix[0].len() as i64;
    let mut antinode_positions: HashSet<(i64, i64)> = HashSet::new();
    for (_antenna_char, antenna_positions) in antenna_positions_map {
        for i in 0..antenna_positions.len() - 1 {
            for j in i + 1..antenna_positions.len() {
                let antinode_1 = (
                    antenna_positions[i].0 + (antenna_positions[i].0 - antenna_positions[j].0),
                    antenna_positions[i].1 + (antenna_positions[i].1 - antenna_positions[j].1),
                );
                let antinode_2 = (
                    antenna_positions[j].0 - (antenna_positions[i].0 - antenna_positions[j].0),
                    antenna_positions[j].1 - (antenna_positions[i].1 - antenna_positions[j].1),
                );

                if antinode_1.0 >= 0
                    && antinode_1.0 < x_len
                    && antinode_1.1 >= 0
                    && antinode_1.1 < y_len
                {
                    antinode_positions.insert(antinode_1);
                    if cloned_matrix[antinode_1.0 as usize][antinode_1.1 as usize] == '.' {
                        cloned_matrix[antinode_1.0 as usize][antinode_1.1 as usize] = '#';
                    }
                }

                if antinode_2.0 >= 0
                    && antinode_2.0 < x_len
                    && antinode_2.1 >= 0
                    && antinode_2.1 < y_len
                {
                    antinode_positions.insert(antinode_2);
                    if cloned_matrix[antinode_2.0 as usize][antinode_2.1 as usize] == '.' {
                        cloned_matrix[antinode_2.0 as usize][antinode_2.1 as usize] = '#';
                    }
                }
            }
        }
    }

    antinode_positions.len() as i64
}

fn part2() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day08.txt").as_str())
        .expect("failed to read file content");

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in text.lines() {
        matrix.push(Vec::new());
        for c in line.chars() {
            if c == '\n' {
                continue;
            }
            let last_elem_index = matrix.len() - 1;
            matrix[last_elem_index].push(c);
        }
    }

    let mut antenna_positions_map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' {
                if let Some(v) = antenna_positions_map.get_mut(c) {
                    v.push((i as i64, j as i64));
                } else {
                    antenna_positions_map.insert(*c, vec![(i as i64, j as i64)]);
                }
            }
        }
    }

    let mut num_antenna_pairs: i64 = 0;
    for (_k, v) in antenna_positions_map.clone() {
        if v.len() > 1 {
            num_antenna_pairs += v.len() as i64;
        }
    }

    fn add_antinode_positions_forward(
        coord1: (i64, i64),
        coord2: (i64, i64),
        antinode_positions: &mut HashSet<(i64, i64)>,
        max_x: i64,
        max_y: i64,
        cloned_matrix: &mut Vec<Vec<char>>,
    ) {
        let antinode = (
            coord2.0 + (coord2.0 - coord1.0),
            coord2.1 + (coord2.1 - coord1.1),
        );

        if antinode.0 >= 0 && antinode.0 < max_x && antinode.1 >= 0 && antinode.1 < max_y {
            if cloned_matrix[antinode.0 as usize][antinode.1 as usize] == '.' {
                cloned_matrix[antinode.0 as usize][antinode.1 as usize] = '#';
                antinode_positions.insert(antinode);
            }

            add_antinode_positions_forward(
                coord2,
                antinode,
                antinode_positions,
                max_x,
                max_y,
                cloned_matrix,
            );
        }
    }

    fn add_antinode_positions_backward(
        coord1: (i64, i64),
        coord2: (i64, i64),
        antinode_positions: &mut HashSet<(i64, i64)>,
        max_x: i64,
        max_y: i64,
        cloned_matrix: &mut Vec<Vec<char>>,
    ) {
        let antinode = (
            coord1.0 - (coord2.0 - coord1.0),
            coord1.1 - (coord2.1 - coord1.1),
        );

        if antinode.0 >= 0 && antinode.0 < max_x && antinode.1 >= 0 && antinode.1 < max_y {
            if cloned_matrix[antinode.0 as usize][antinode.1 as usize] == '.' {
                cloned_matrix[antinode.0 as usize][antinode.1 as usize] = '#';
                antinode_positions.insert(antinode);
            }

            add_antinode_positions_backward(
                antinode,
                coord1,
                antinode_positions,
                max_x,
                max_y,
                cloned_matrix,
            );
        }
    }

    let mut cloned_matrix = matrix.clone();
    let x_len = matrix.len() as i64;
    let y_len = matrix[0].len() as i64;
    let mut antinode_positions: HashSet<(i64, i64)> = HashSet::new();
    for (_antenna_char, antenna_positions) in antenna_positions_map {
        for i in 0..antenna_positions.len() - 1 {
            for j in i + 1..antenna_positions.len() {
                add_antinode_positions_forward(
                    antenna_positions[i],
                    antenna_positions[j],
                    &mut antinode_positions,
                    x_len,
                    y_len,
                    &mut cloned_matrix,
                );
                add_antinode_positions_backward(
                    antenna_positions[i],
                    antenna_positions[j],
                    &mut antinode_positions,
                    x_len,
                    y_len,
                    &mut cloned_matrix,
                );
            }
        }
    }

    antinode_positions.len() as i64 + num_antenna_pairs
}
