#![allow(dead_code, unused)]

use common::read_puzzle;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        if end < start {
            panic!("End cannot be less than start for ranges");
        }

        Self { start, end }
    }

    fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    fn contains(&self, num: u64) -> bool {
        self.start <= num && num <= self.end
    }

    fn union_ranges(a: &Range, b: &Range) -> Option<Range> {
        if a.contains(b.start) && a.contains(b.end) {
            return Some(a.clone());
        } else if b.contains(a.start) && b.contains(a.end) {
            return Some(b.clone());
        } else if a.contains(b.start) && !a.contains(b.end) {
            return Some(Range::new(a.start, b.end));
        } else if b.contains(a.start) && !b.contains(a.end) {
            return Some(Range::new(b.start, a.end));
        } else if a.contains(b.end) && !a.contains(b.start) {
            return Some(Range::new(b.start, a.end));
        } else if b.contains(a.end) && !b.contains(a.start) {
            return Some(Range::new(a.start, b.end));
        } else {
            return None;
        }
    }

    fn union(&self, rhs: &Range) -> Option<Range> {
        Self::union_ranges(self, rhs)
    }
}

#[derive(Debug, PartialEq)]
enum Record {
    Range(Range),
    Ingredient(u64),
}

fn parse_range(line: &str) -> Option<Record> {
    match line.split_once('-') {
        Some((start, end)) => {
            let start: u64 = start.parse().expect("Error parsing start");
            let end: u64 = end.parse().expect("Error parsing end");

            Some(Record::Range(Range::new(start, end)))
        }
        None => None,
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
    let mut ranges: Vec<Range> = vec![];
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
            if range.contains(id) {
                result += 1;
                continue 'outer;
            }
        }
    }

    result
}

fn solve_part_2(puzzle: &str) -> u64 {
    let mut ranges: Vec<Range> = vec![];

    for line in puzzle.lines() {
        match parse_line(line) {
            Some(Record::Ingredient(n)) => (),
            Some(Record::Range(range)) => ranges.push(range),
            None => (),
        }
    }

    ranges.sort();

    let mut i: usize = 0;
    while i < ranges.len() - 1 {
        let a = ranges[i];
        let b = ranges[i + 1];

        match a.union(&b) {
            Some(r) => {
                ranges[i] = r;
                ranges.remove(i + 1);
            }
            None => i += 1,
        }
    }

    ranges.iter().map(|r| r.len()).sum()
}

fn main() {
    let file_name = "../puzzle/5/input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("3-5"), Some(Record::Range(Range::new(3, 5))));
        assert_eq!(
            parse_range("12-18"),
            Some(Record::Range(Range::new(12, 18)))
        );
        assert_eq!(
            parse_range("7992555747716-8240215261980"),
            Some(Record::Range(Range::new(7992555747716, 8240215261980)))
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("123"), Some(Record::Ingredient(123)));
        assert_eq!(
            parse_line("123-321"),
            Some(Record::Range(Range::new(123, 321)))
        );
        assert_eq!(parse_line(""), None);
        assert_eq!(parse_line("\n"), None);
    }

    #[test]
    fn test_range_union() {
        assert_eq!(Range::new(3, 4).union(&Range::new(10, 14)), None);
        assert_eq!(
            Range::new(10, 14).union(&Range::new(12, 18)),
            Some(Range::new(10, 18))
        );
        assert_eq!(
            Range::new(12, 18).union(&Range::new(16, 20)),
            Some(Range::new(12, 20))
        );
        assert_eq!(
            Range::new(12, 20).union(&Range::new(16, 18)),
            Some(Range::new(12, 20))
        );
        assert_eq!(
            Range::new(16, 18).union(&Range::new(12, 20)),
            Some(Range::new(12, 20))
        );
        assert_eq!(
            Range::new(16, 18).union(&Range::new(18, 20)),
            Some(Range::new(16, 20))
        );
        assert_eq!(
            Range::new(18, 20).union(&Range::new(16, 18)),
            Some(Range::new(16, 20))
        );
    }

    #[test]
    fn test_ranges() {
        assert_eq!(Range::new(3, 5), Range::new(3, 5));
        assert_ne!(Range::new(3, 7), Range::new(3, 6));
        assert!(Range::new(1, 2) < Range::new(3, 4));
        assert!(Range::new(3, 6) < Range::new(4, 5));
        assert!(Range::new(3, 6) < Range::new(4, 7));
        assert!(Range::new(3, 7) > Range::new(2, 6));
    }

    fn test_range_len() {
        assert_eq!(Range::new(3, 5).len(), 3);
        assert_eq!(Range::new(4, 4).len(), 1);
    }
}
