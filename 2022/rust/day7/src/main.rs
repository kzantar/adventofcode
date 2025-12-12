use common::read_puzzle;
use regex::Regex;
use std::path::{PathBuf, Path};
use std::collections::HashMap;

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            files: vec![],
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Shell {
    root: HashMap<String, Dir>,
    path: PathBuf,
    cd_re: Regex,
    ls_re: Regex,
    dir_re: Regex,
    file_re: Regex,
}

impl Shell {
    fn new() -> Shell {
        Shell {
            cd_re: Regex::new(r"\$ cd ([/\w]+)").unwrap(),
            ls_re: Regex::new(r"\$ ls").unwrap(),
            dir_re: Regex::new(r"dir (\w+)").unwrap(),
            file_re: Regex::new(r"(\d+) (.+)").unwrap(),
            root: Dir::new("/".to_string()),
            path: PathBuf::new(),
        }
    }

    fn parse_line(&mut self, line: &str) {
        if self.cd_re.is_match(line) {
            self.cd(line);
        } else if self.ls_re.is_match(line) {
        } else if self.dir_re.is_match(line) {
            self.insert_dir(line);
        } else if self.file_re.is_match(line) {
            self.insert_file(line);
        }
    }

    fn cd(&mut self, line: &str) {
        let caps = self.cd_re.captures(line).unwrap();
        let dir_name = caps.get(1).unwrap().as_str();
        match dir_name {
            "/" => {
                self.path.push("/");
            }
            ".." => {
                self.path.pop();
            }
            name => {
                self.path.push(name);
            }
        }
        println!("Current path: {:?}", self.path);
    }

    fn insert_dir(&self, line: &str) {
        println!("Inserted dir {}", line);
    }

    fn insert_file(&self, line: &str) {
        println!("Inserted file {}", line);
    }
}

fn solve_part_1(puzzle: &str) -> usize {
    let mut shell = Shell::new();

    for line in puzzle.lines() {
        shell.parse_line(line);
    }

    0
}

fn solve_part_2(puzzle: &str) -> usize {
    0
}

fn main() {
    let file_name = "../puzzle/7_test.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    //    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;
}
