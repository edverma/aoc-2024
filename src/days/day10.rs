use std::collections::HashSet;

use super::utils;

pub fn solve() {
    println!("Day 10: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
}

fn part1() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day10.txt").as_str())
        .expect("failed to read file content");

    let mut matrix: Vec<Vec<i64>> = Vec::new();
    for (i, row) in text.lines().enumerate() {
        matrix.push(Vec::new());
        for c in row.chars() {
            matrix[i].push(c.to_string().parse().unwrap());
        }
    }

    fn find_next_pos(matrix: &[Vec<i64>], pos: (usize, usize)) -> Vec<(usize, usize)> {
        let cur_val = matrix[pos.0][pos.1];
        let mut next_pos: Vec<(usize, usize)> = Vec::new();
        let directions: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for d in directions {
            let next_x: i64 = pos.0 as i64 + d.0;
            let next_y: i64 = pos.1 as i64 + d.1;
            if next_x < 0
                || next_y < 0
                || next_x >= matrix.len() as i64
                || next_y >= matrix[next_x as usize].len() as i64
            {
                continue;
            }

            if matrix[next_x as usize][next_y as usize] == cur_val + 1 {
                next_pos.push((next_x as usize, next_y as usize))
            }
        }
        next_pos
    }

    fn get_score(matrix: &Vec<Vec<i64>>, pos: (usize, usize), ends: &mut HashSet<(usize, usize)>) {
        let cur_val = matrix[pos.0][pos.1];
        if cur_val == 9 {
            ends.insert(pos);
        }

        let next_pos = find_next_pos(matrix, pos);
        for p in next_pos {
            get_score(matrix, p, ends);
        }
    }

    let mut sum = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                let mut set: HashSet<(usize, usize)> = HashSet::new();
                get_score(&matrix, (i, j), &mut set);
                sum += set.len() as i64;
            }
        }
    }

    sum
}

fn part2() -> i64 {
    let text = utils::read_file_content_as_string(utils::get_path("day10.txt").as_str())
        .expect("failed to read file content");

    let mut matrix: Vec<Vec<i64>> = Vec::new();
    for (i, row) in text.lines().enumerate() {
        matrix.push(Vec::new());
        for c in row.chars() {
            matrix[i].push(c.to_string().parse().unwrap());
        }
    }

    fn find_next_pos(matrix: &[Vec<i64>], pos: (usize, usize)) -> Vec<(usize, usize)> {
        let cur_val = matrix[pos.0][pos.1];
        let mut next_pos: Vec<(usize, usize)> = Vec::new();
        let directions: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for d in directions {
            let next_x: i64 = pos.0 as i64 + d.0;
            let next_y: i64 = pos.1 as i64 + d.1;
            if next_x < 0
                || next_y < 0
                || next_x >= matrix.len() as i64
                || next_y >= matrix[next_x as usize].len() as i64
            {
                continue;
            }

            if matrix[next_x as usize][next_y as usize] == cur_val + 1 {
                next_pos.push((next_x as usize, next_y as usize))
            }
        }
        next_pos
    }

    fn get_score(matrix: &Vec<Vec<i64>>, pos: (usize, usize)) -> i64 {
        let cur_val = matrix[pos.0][pos.1];
        if cur_val == 9 {
            return 1;
        }

        let mut sum = 0;
        let next_pos = find_next_pos(matrix, pos);
        for p in next_pos {
            sum += get_score(matrix, p);
        }

        sum
    }

    let mut sum = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                let score = get_score(&matrix, (i, j));
                sum += score;
            }
        }
    }

    sum
}
