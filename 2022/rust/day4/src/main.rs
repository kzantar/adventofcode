#![allow(dead_code, unused)]

use common::read_puzzle;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn new(range: &str) -> Section {
        let (start, end) = match range.split_once('-') {
            Some((start, end)) => (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap()),
            None => panic!("Wrong range {}", range),
        };

        Section { start, end }
    }

    fn contains(&self, other: &Section) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
    }

    fn overlap(&self, other: &Section) -> bool {
        !((self.end < other.start) || (other.end < self.start))
    }
}

fn solve_part_1(puzzle: &str) -> usize {
    puzzle
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(first, second)| {
            let first = Section::new(first);
            let second = Section::new(second);
            if first.contains(&second) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn solve_part_2(puzzle: &str) -> usize {
    puzzle
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(first, second)| {
            let first = Section::new(first);
            let second = Section::new(second);
            if first.overlap(&second) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let file_name = "../puzzle/4_input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_section() {
        let s = Section::new("1-5");
        assert_eq!(s.start, 1);
        assert_eq!(s.end, 5);
    }

    #[test]
    fn contains() {
        let s1 = Section::new("1-5");
        let s2 = Section::new("2-3");
        let s3 = Section::new("4-6");

        assert!(s1.contains(&s2));
        assert!(!s1.contains(&s3));
    }

    #[test]
    fn overlap() {
        let s1 = Section::new("2-4");
        let s2 = Section::new("6-8");
        let s3 = Section::new("5-7");
        let s4 = Section::new("7-9");

        assert!(!s1.overlap(&s2));
        assert!(s3.overlap(&s4));
    }
}
