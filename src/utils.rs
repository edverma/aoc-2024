use std::fs;

const INPUT_DIR_PATH: &str = "/Users/edverma/Development/aoc-2024/inputs/";

pub fn get_path(filename: &str) -> String {
    format!("{}{}", INPUT_DIR_PATH, filename)
}

pub fn read_file_content_as_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let string_content: String = fs::read_to_string(path)?;
    Ok(string_content)
}
