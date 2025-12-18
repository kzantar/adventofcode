#![allow(dead_code, unused)]

use std::ops::RangeInclusive;

use common::read_puzzle;

#[derive(Debug, PartialEq)]
enum Record {
    Range(RangeInclusive<u64>),
    Ingredient(u64)
}


fn parse_range(line: &str) -> Option<Record> {
    match line.split_once('-') {
        Some((start, end)) => {
            let start: u64 = start.parse().expect("Error parsing start");
            let end: u64 = end.parse().expect("Error parsing end");
            
            Some(Record::Range(start..=end))
        }
        None => None
    }
}

fn parse_line(line: &str) -> Option<Record> {
    match line {
        s if s.contains('-') => parse_range(s),
        s if s.trim().len() > 0 => {
            let id = line.parse::<u64>().expect("Fail to parse id");
            Some(Record::Ingredient(id))
        }
        _ => None,
    }
}

fn solve_part_1(puzzle: &str) -> u32 {
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut ingredients: Vec<u64> = vec![];

    for line in puzzle.lines() {
        match parse_line(line) {
            Some(Record::Ingredient(n)) => ingredients.push(n),
            Some(Record::Range(range)) => ranges.push(range),
            None => (),
        }
    }

    let mut result = 0u32;
    'outer: for id in ingredients {
        for range in &ranges {
            if range.contains(&id) {
                result += 1;
                continue 'outer;
            }
        }
    }

    result
}

fn solve_part_2(puzzle: &str) -> u64 {
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];

    for line in puzzle.lines() {
        match parse_line(line) {
            Some(Record::Ingredient(n)) => (),
            Some(Record::Range(range)) => ranges.push(range),
            None => (),
        }
    }

    let mut result = 0u64;

    result
}

fn main() {
    let file_name = "../puzzle/5/input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    // println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_inclusive() {
        let range = RangeInclusive::new(1, 5);
        assert!(range.contains(&3));

        let range: RangeInclusive<u64> = RangeInclusive::new(1469391402608, 1671922872849);
        assert!(range.contains(&1469391402610));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("3-5"), Some(Record::Range(3..=5)));
        assert_eq!(parse_range("12-18"), Some(Record::Range(12..=18)));
        assert_eq!(parse_range("7992555747716-8240215261980"), Some(Record::Range(7992555747716..=8240215261980)));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("123"), Some(Record::Ingredient(123)));
        assert_eq!(parse_line("123-321"), Some(Record::Range(123..=321)));
        assert_eq!(parse_line(""), None);
        assert_eq!(parse_line("\n"), None);
    }
}
