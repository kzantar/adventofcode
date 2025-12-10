#![allow(dead_code, unused)]

use common::read_puzzle;

fn get_items(puzzle: &String) -> Vec<u32> {
    let items: Vec<u32> = puzzle
        .lines()
        .map(|line| line.parse::<u32>().unwrap_or_default())
        .collect();
    items
}

#[derive(Debug)]
struct Dial {
    pos: i32,
}

impl Dial {
    fn new(pos: i32) -> Self {
        Dial { pos: pos }
    }

    fn left(&mut self, num: i32) -> usize {
        let diff = self.pos - num;
        let mut zeros = 0;

        if diff < 0 {
            if self.pos != 0 {
                zeros += 1;
            }
            if diff.abs() >= 100 {
                zeros += diff.abs() / 100;
            }

            self.pos = diff % 100;
            if self.pos < 0 {
                self.pos += 100;
            }
            if self.pos == 0 {
                zeros -= 1;
            }
        } else {
            self.pos = diff;
        }

        zeros as usize
    }

    fn right(&mut self, num: i32) -> usize {
        let diff = self.pos + num;
        let mut zeros = 0;

        if diff >= 100 {
            zeros += diff / 100;
            self.pos = diff % 100;
            if self.pos == 0 {
                zeros -= 1;
            }
        } else {
            self.pos = diff;
        }

        zeros as usize
    }

    fn step(&mut self, cmd: &str, second: bool) -> usize {
        let (dir, num) = cmd.split_at(1);

        let n: i32 = num.parse().unwrap();

        let zeros_inside = match dir {
            "L" => self.left(n),
            "R" => self.right(n),
            _ => 0,
        };

        let zeros_at_end = if (self.pos == 0) { 1 } else { 0 };

        if second {
            zeros_inside + zeros_at_end
        } else {
            zeros_at_end
        }
    }
}

fn solve_part_1(puzzle: &str) -> usize {
    let mut d = Dial::new(50);

    let mut zeros = 0;
    for line in puzzle.lines() {
        zeros += d.step(line, false);
    }

    zeros
}

fn solve_part_2(puzzle: &str) -> usize {
    let mut d = Dial::new(50);

    let mut zeros = 0;
    for line in puzzle.lines() {
        let prev_pos = d.pos;
        zeros += d.step(line, true);
    }

    zeros
}

fn main() {
    let file_name = "../puzzle/1_input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial_left() {
        let mut dial = Dial::new(50);
        dial.left(10);
        assert_eq!(dial.pos, 40);

        dial.left(45);
        assert_eq!(dial.pos, 95);

        dial.left(200);
        assert_eq!(dial.pos, 95);
    }

    #[test]
    fn test_dial_left_bug() {
        let mut dial = Dial::new(1);
        let zeros = dial.left(2);
        assert_eq!(dial.pos, 99);
        assert_eq!(zeros, 1);
    }

    #[test]
    fn test_dial_right() {
        let mut dial = Dial::new(50);
        let zeros = dial.right(10);
        assert_eq!(dial.pos, 60);
        assert_eq!(zeros, 0);

        let zeros = dial.right(45);
        assert_eq!(dial.pos, 5);
        assert_eq!(zeros, 1);

        let zeros = dial.right(200);
        assert_eq!(dial.pos, 5);
        assert_eq!(zeros, 2);
    }

    #[test]
    fn test_boundaries() {
        let mut dial = Dial::new(1);
        let zeros = dial.step("L1", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        dial.pos = 1;
        let zeros = dial.step("L101", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 2);

        dial.pos = 99;
        let zeros = dial.step("R1", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        dial.pos = 99;
        let zeros = dial.step("R101", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 2);

        dial.pos = 0;
        let zeros = dial.step("R100", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        dial.pos = 0;
        let zeros = dial.step("L100", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        dial.pos = 0;
        let zeros = dial.step("R200", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 2);

        dial.pos = 0;
        let zeros = dial.step("L200", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 2);
    }

    #[test]
    fn test_dial_step_1() {
        let mut dial = Dial::new(50);

        let zeros = dial.step("R10", false);
        assert_eq!(dial.pos, 60);
        assert_eq!(zeros, 0);

        let zeros = dial.step("L100", false);
        assert_eq!(dial.pos, 60);
        assert_eq!(zeros, 0);

        let zeros = dial.step("L200", false);
        assert_eq!(dial.pos, 60);
        assert_eq!(zeros, 0);

        let zeros = dial.step("L60", false);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);
    }

    #[test]
    fn test_dial_step_2() {
        let mut dial = Dial::new(50);

        let zeros = dial.step("R10", true);
        assert_eq!(dial.pos, 60);
        assert_eq!(zeros, 0);

        let zeros = dial.step("R41", true);
        assert_eq!(dial.pos, 1);
        assert_eq!(zeros, 1);

        let zeros = dial.step("R200", true);
        assert_eq!(dial.pos, 1);
        assert_eq!(zeros, 2);

        let zeros = dial.step("L2", true);
        assert_eq!(dial.pos, 99);
        assert_eq!(zeros, 1);

        let zeros = dial.step("L200", true);
        assert_eq!(dial.pos, 99);
        assert_eq!(zeros, 2);
    }

    #[test]
    fn test_solution_1() {
        let mut dial = Dial::new(50);

        let zeros = dial.step("L68", false);
        assert_eq!(dial.pos, 82);
        assert_eq!(zeros, 0);

        let zeros = dial.step("L30", false);
        assert_eq!(dial.pos, 52);
        assert_eq!(zeros, 0);

        let zeros = dial.step("R48", false);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        let zeros = dial.step("L5", false);
        assert_eq!(dial.pos, 95);
        assert_eq!(zeros, 0);

        let zeros = dial.step("R60", false);
        assert_eq!(dial.pos, 55);
        assert_eq!(zeros, 0);

        let zeros = dial.step("L55", false);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        let zeros = dial.step("L1", false);
        assert_eq!(dial.pos, 99);
        assert_eq!(zeros, 0);

        let zeros = dial.step("L99", false);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        let zeros = dial.step("R14", false);
        assert_eq!(dial.pos, 14);
        assert_eq!(zeros, 0);

        let zeros = dial.step("L82", false);
        assert_eq!(dial.pos, 32);
        assert_eq!(zeros, 0);
    }

    #[test]
    fn test_solution_2() {
        let mut dial = Dial::new(50);

        let zeros = dial.step("L68", true);
        assert_eq!(dial.pos, 82);
        assert_eq!(zeros, 1);

        let zeros = dial.step("L30", true);
        assert_eq!(dial.pos, 52);
        assert_eq!(zeros, 0);

        let zeros = dial.step("R48", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        let zeros = dial.step("L5", true);
        assert_eq!(dial.pos, 95);
        assert_eq!(zeros, 0);

        let zeros = dial.step("R60", true);
        assert_eq!(dial.pos, 55);
        assert_eq!(zeros, 1);

        let zeros = dial.step("L55", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        let zeros = dial.step("L1", true);
        assert_eq!(dial.pos, 99);
        assert_eq!(zeros, 0);

        let zeros = dial.step("L99", true);
        assert_eq!(dial.pos, 0);
        assert_eq!(zeros, 1);

        let zeros = dial.step("R14", true);
        assert_eq!(dial.pos, 14);
        assert_eq!(zeros, 0);

        let zeros = dial.step("L82", true);
        assert_eq!(dial.pos, 32);
        assert_eq!(zeros, 1);
    }
}
