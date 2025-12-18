#![allow(dead_code, unused)]

use common::read_puzzle;

fn main() {
    let file_name = "../puzzle/5/test.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    // println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    // println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;
}
