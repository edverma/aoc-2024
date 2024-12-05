use super::utils;
use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    println!("Day 5: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
    println!();
}

// get sum of middle numbers of all correct inputs
fn part1() -> i32 {
    let text = utils::read_file_content_as_string(utils::get_path("day05.txt").as_str())
        .expect("failed to read file content");

    let (rules_map, updates_list) = get_rules_map_and_updates_list(text.as_str());

    let mut sum = 0;
    for update in updates_list {
        let mut valid = true;
        for (i, v) in update.iter().enumerate() {
            if let Some(vec) = rules_map.get(v) {
                for update_val in update.iter().take(i) {
                    for vec_val in vec {
                        if update_val == vec_val {
                            valid = false;
                            break;
                        }
                    }

                    if !valid {
                        break;
                    }
                }
            }

            if !valid {
                break;
            }
        }

        if valid {
            sum += update[update.len() / 2];
        }
    }

    sum
}

// reorder invalid updates and return sum of middle number of each reordered update
fn part2() -> i32 {
    let text = utils::read_file_content_as_string(utils::get_path("day05.txt").as_str())
        .expect("failed to read file content");

    let (rules_map, updates_list) = get_rules_map_and_updates_list(text.as_str());

    let mut sum = 0;
    for update in updates_list {
        let mut valid = true;
        for (i, v) in update.iter().enumerate() {
            if let Some(vec) = rules_map.get(v) {
                for update_val in update.iter().take(i) {
                    for vec_val in vec {
                        if update_val == vec_val {
                            valid = false;
                            break;
                        }
                    }

                    if !valid {
                        break;
                    }
                }
            }

            if !valid {
                break;
            }
        }

        if !valid {
            let mut cloned_update = update.clone();
            for (k, vec) in &rules_map {
                let mut k_index: Option<usize> = None;
                let mut lowest_v_index: Option<usize> = None;
                for (i, update_val) in cloned_update.iter().enumerate() {
                    if update_val == k {
                        k_index = Some(i);
                    }
                    for v in vec {
                        if v == update_val
                            && (lowest_v_index.is_none() || lowest_v_index.unwrap() > i)
                        {
                            lowest_v_index = Some(i);
                        }
                    }
                }

                if k_index.is_some()
                    && lowest_v_index.is_some()
                    && k_index.unwrap() > lowest_v_index.unwrap()
                {
                    let element = cloned_update.remove(k_index.unwrap());
                    cloned_update.insert(lowest_v_index.unwrap(), element);
                }
            }

            sum += cloned_update[cloned_update.len() / 2];
        }
    }

    sum
}

fn get_rules_map_and_updates_list(text: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut d: Vec<Vec<i32>> = Vec::new();
    let mut fill_m = true;
    for line in text.lines() {
        if fill_m {
            if let Some(caps) = re.captures(line) {
                let k: i32 = caps[1].parse().expect("failed to parse cap 1");
                let v: i32 = caps[2].parse().expect("failed to parse cap 2");

                if let Some(vec) = m.get_mut(&k) {
                    vec.push(v);
                } else {
                    m.insert(k, vec![v]);
                }
            } else {
                fill_m = false;
                continue;
            }
        } else {
            let nums: Vec<&str> = line.split(',').collect();
            let mut nums_ints: Vec<i32> = Vec::new();
            for n in nums {
                nums_ints.push(n.parse().expect("failedd to parse number"));
            }
            d.push(nums_ints);
        }
    }

    (m, d)
}
