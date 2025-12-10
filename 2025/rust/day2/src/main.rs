#![allow(dead_code, unused)]

use common::read_puzzle;

fn check_value_1(val: &str) -> bool {
    if val.len() % 2 != 0 {
        return false;
    }

    let (part_1, part_2) = val.split_at(val.len() / 2);

    part_1.eq(part_2)
}

fn check_value_2(val: &str) -> bool {
    let s = val.to_string();
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    for i in 1..=n/2 {
        let mut chunks = chars.chunks(i);
        let first = chunks.next().unwrap();
        if chunks.all(|x| x == first) {
            return true;
        }
    }
    // We tried all variants and not found invalid pattern
    false
}

fn find_invalid(range: &str, cheker: fn(&str) -> bool) -> Vec<i64> {
    let (start, end) = range.split_once('-').expect("Invalid range");
    let start: i64 = start.parse().expect("Invalid start of range");
    let end: i64 = end.parse().expect("Invalid end of range");

    let mut result = Vec::new();

    for val in start..=end {
        if cheker(&val.to_string()) {
            result.push(val);
        }
    }

    result
}

fn solve_part_1(puzzle: &str) -> i64 {
    let mut result: i64 = 0;
    for range in puzzle.split(',') {
        result += find_invalid(range, check_value_1).iter().sum::<i64>();
    }

    result
}

fn solve_part_2(puzzle: &str) -> i64 {
    let mut result: i64 = 0;
    for range in puzzle.split(',') {
        result += find_invalid(range, check_value_2).iter().sum::<i64>();
    }

    result
}

fn main() {
    let file_name = "../puzzle/2_input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_value_1() {
        assert_eq!(check_value_1("11"), true);
        assert_eq!(check_value_1("22"), true);
        assert_eq!(check_value_1("1010"), true);

        assert_eq!(check_value_1("111"), false);
        assert_eq!(check_value_1("2122"), false);
        assert_eq!(check_value_1("221122"), false);
    }

    #[test]
    fn test_check_value_2() {
        assert_eq!(check_value_2("11"), true);
        assert_eq!(check_value_2("22"), true);
        assert_eq!(check_value_2("111"), true);
        assert_eq!(check_value_2("999"), true);
        assert_eq!(check_value_2("1010"), true);
        assert_eq!(check_value_2("2121212121"), true);
        assert_eq!(check_value_2("1188511885"), true);
        assert_eq!(check_value_2("824824824"), true);

        assert_eq!(check_value_2("2122"), false);
        assert_eq!(check_value_2("221122"), false);
    }

    #[test]
    fn test_find_invalid_1() {
        assert_eq!(find_invalid("11-22", check_value_1), vec![11, 22]);
        assert_eq!(find_invalid("95-115", check_value_1), vec![99]);
        assert_eq!(find_invalid("998-1012", check_value_1), vec![1010]);
        assert_eq!(find_invalid("1188511880-1188511890", check_value_1), vec![1188511885]);
        assert_eq!(find_invalid("222220-222224", check_value_1), vec![222222]);
        assert_eq!(find_invalid("1698522-1698528", check_value_1), vec![]);
        assert_eq!(find_invalid("446443-446449", check_value_1), vec![446446]);
        assert_eq!(find_invalid("38593856-38593862", check_value_1), vec![38593859]);
        assert_eq!(find_invalid("565653-565659", check_value_1), vec![]);
        assert_eq!(find_invalid("824824821-824824827", check_value_1), vec![]);
        assert_eq!(find_invalid("2121212118-2121212124", check_value_1), vec![]);
    }

    #[test]
    fn test_find_invalid_2() {
        assert_eq!(find_invalid("11-22", check_value_2), vec![11, 22]);
        assert_eq!(find_invalid("95-115", check_value_2), vec![99, 111]);
        assert_eq!(find_invalid("998-1012", check_value_2), vec![999, 1010]);
        assert_eq!(find_invalid("1188511880-1188511890", check_value_2), vec![1188511885]);
        assert_eq!(find_invalid("222220-222224", check_value_2), vec![222222]);
        assert_eq!(find_invalid("1698522-1698528", check_value_2), vec![]);
        assert_eq!(find_invalid("446443-446449", check_value_2), vec![446446]);
        assert_eq!(find_invalid("38593856-38593862", check_value_2), vec![38593859]);
        assert_eq!(find_invalid("565653-565659", check_value_2), vec![565656]);
        assert_eq!(find_invalid("824824821-824824827", check_value_2), vec![824824824]);
        assert_eq!(find_invalid("2121212118-2121212124", check_value_2), vec![2121212121]);
    }
}
