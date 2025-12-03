use std::fs;
use std::io;

pub fn read_puzzle(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
