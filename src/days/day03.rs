use regex::Regex;

pub fn solve() {
    println!("Day 3: ");
    println!("\tPart 1: {}", part1());
    println!("\tPart 2: {}", part2());
    println!();
}

fn part1() -> i32 {
    let mut res = 0;
    let text = super::super::utils::read_file_content_as_string(
        super::super::utils::get_path("day03.txt").as_str(),
    )
    .expect("failed to read file content");

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let groups_re = Regex::new(r"(\d+),(\d+)").unwrap();
    for m in re.find_iter(&text) {
        let m_str = m.as_str();
        if let Some(caps) = groups_re.captures(m_str) {
            let num1: i32 = caps[1].parse().expect("failed to parse string");
            let num2: i32 = caps[2].parse().expect("failed to parse string");
            res += num1 * num2;
        }
    }

    res
}

fn part2() -> i32 {
    let text = super::super::utils::read_file_content_as_string(
        super::super::utils::get_path("day03.txt").as_str(),
    )
    .expect("failed to read file content");

    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut res = 0;
    let mut enabled = true;
    for caps in re.captures_iter(&text) {
        if let Some(_cap) = caps.get(3) {
            enabled = true;
            continue;
        } else if let Some(_cap) = caps.get(4) {
            enabled = false;
            continue;
        }

        if !enabled {
            continue;
        }

        let num1: i32 = caps
            .get(1)
            .expect("failed to get first number")
            .as_str()
            .parse()
            .expect("failed to parse string");

        let num2: i32 = caps
            .get(2)
            .expect("failed to get first number")
            .as_str()
            .parse()
            .expect("failed to parse string");

        res += num1 * num2;
    }

    res
}
