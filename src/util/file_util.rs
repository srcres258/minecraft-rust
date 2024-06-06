use std::fs;

pub fn get_file_contents(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(
        format!("Unable to open file: {}", file_path).as_str())
}