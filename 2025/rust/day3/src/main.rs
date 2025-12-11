#![allow(dead_code, unused)]

use std::collections::VecDeque;

use common::read_puzzle;

fn max(value: &[u32]) -> (usize, u32) {
    let mut max_idx = 0;
    let mut max_val = value[0];

    for (i, &val) in value.iter().enumerate().skip(1) {
        if val > max_val {
            max_val = val;
            max_idx = i;
        }
    }

    (max_idx, max_val)
}

fn find_max_joltage_1(value: &str) -> u32 {
    let bank: Vec<u32> = value.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let first_bat = max(&bank);
    let second_bat: (usize, u32);

    let (left, right) = bank.split_at(first_bat.0);
    if right.len() == 1 {
        // only first battery in right part
        let second_bat = max(left);

        return second_bat.1 * 10 + first_bat.1;
    } else {
        let (_, right) = right.split_first().unwrap();
        let second_bat = max(right);

        return first_bat.1 * 10 + second_bat.1;
    }
}

fn find_max_joltage_2(value: &str, cnt: usize) -> u32 {
    let mut bank: VecDeque<u32> = value.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut rest = bank.split_off(cnt);

    while rest.len() > 0 {
        println!("bank => {:?}", bank);
        println!("rest => {:?}", rest);
        let val = rest.pop_front().unwrap();
        println!("val => {:?}", val);
        // Result consists values less then i
        if bank.iter().any(|&x| x < val) {
            bank.retain(|&x| x >= val);
            bank.push_back(val);
        }
        println!("bank => {:?}", bank);
        println!();
    }

    0

}

fn solve_part_1(puzzle: &str) -> u32 {
    puzzle.lines().map(find_max_joltage_1).sum()
}

fn solve_part_2(puzzle: &str) -> u32 {
    0
}

fn main() {
    let file_name = "../puzzle/3_input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    find_max_joltage_2("234234234234278", 4);

    //    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    //    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_joltage_1() {
        assert_eq!(find_max_joltage_1("987654321111111"), 98);
        assert_eq!(find_max_joltage_1("811111111111119"), 89);
        assert_eq!(find_max_joltage_1("234234234234278"), 78);
        assert_eq!(find_max_joltage_1("818181911112111"), 92);

        assert_eq!(find_max_joltage_1("818181911912111"), 99);
        assert_eq!(find_max_joltage_1("918181191112119"), 99);
        assert_eq!(find_max_joltage_1("918181911112111"), 99);
    }

    #[test]
    fn test_find_max_joltage_2() {
        assert_eq!(find_max_joltage_2("987654321111111"), 987654321111);
        assert_eq!(find_max_joltage_2("811111111111119"), 811111111119);
        assert_eq!(find_max_joltage_2("234234234234278"), 434234234278);
        assert_eq!(find_max_joltage_2("818181911112111"), 888911112111);

        assert_eq!(find_max_joltage_2("818181911912111"), 99);
        assert_eq!(find_max_joltage_2("918181191112119"), 99);
        assert_eq!(find_max_joltage_2("918181911112111"), 99);
    }
}
