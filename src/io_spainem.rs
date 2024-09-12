use std::fs;

pub fn read_from_file(file_path: &str) -> Vec<u8> {
    fs::read(file_path).expect("Could not read from file.")
}
