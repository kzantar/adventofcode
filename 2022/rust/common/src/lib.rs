use std::fs;
use std::io;

pub fn read_puzzle(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
