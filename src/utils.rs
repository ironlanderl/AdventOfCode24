use std::fs;
use std::io::Read;

pub fn read_file(filepath: String) -> String {
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    contents
}