use std::fs;

pub fn read_file_content_as_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let string_content: String = fs::read_to_string(path)?;
    Ok(string_content)
}
