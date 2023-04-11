#![allow(dead_code, unused)]

use regex::Regex;
use common::read_puzzle;

type Stack = Vec<char>;

fn get_test_stacks() -> Vec<Stack> {
    vec![
        vec!['Z', 'N'],
        vec!['M', 'C', 'D'],
        vec!['P'],
    ]
}

fn get_input_stacks() -> Vec<Stack> {
    vec![
        vec!['W', 'D', 'G', 'B', 'H', 'R', 'V'],
        vec!['J', 'N', 'G', 'C', 'R', 'F'],
        vec!['L', 'S', 'F', 'H', 'D', 'N', 'J'],
        vec!['J', 'D', 'S', 'V'],
        vec!['S', 'H', 'D', 'R', 'Q', 'W', 'N', 'V'],
        vec!['P', 'G', 'H', 'C', 'M'],
        vec!['F', 'J', 'B', 'G', 'L', 'Z', 'H', 'C'],
        vec!['S', 'J', 'R'],
        vec!['L', 'G', 'S', 'R', 'B', 'N', 'V', 'M'],
    ]
}

fn parse_command(cmd: &str) -> (usize, usize, usize) {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let caps = re.captures(cmd).unwrap();
    let count: usize = usize::from_str_radix(&caps[1], 10).unwrap();
    let src: usize = usize::from_str_radix(&caps[2], 10).unwrap();
    let dst: usize = usize::from_str_radix(&caps[3], 10).unwrap();
    
    (count, src, dst)
}

fn procedure_1(stacks: &mut Vec<Stack>, cmd: &str) {
    let (count, src, dst) = parse_command(cmd);
    
    for _ in 0..count {
        let tmp = stacks[src-1].pop().unwrap();
        stacks[dst-1].push(tmp);
    }
    
}

fn procedure_2(stacks: &mut Vec<Stack>, cmd: &str) {
    let (count, src, dst) = parse_command(cmd);
    
    let mut tmp = vec![];
    for _ in 0..count {
        tmp.push(stacks[src-1].pop().unwrap());
    }
    tmp.reverse();
    
    stacks[dst-1].append(&mut tmp);
}

fn read_top(stacks: &Vec<Stack>) -> String {
    let mut top = String::new();
    
    for s in stacks {
        top.push(*s.last().unwrap());
    }
    
    top
}

fn solve_part_1(puzzle: &str) -> String {
    let mut stacks = get_input_stacks();
    
    for command in puzzle.lines() {
        procedure_1(&mut stacks, command);
    }
    
    read_top(&stacks)
}

fn solve_part_2(puzzle: &str) -> String {
    let mut stacks = get_input_stacks();
    
    for command in puzzle.lines() {
        procedure_2(&mut stacks, command);
    }

    read_top(&stacks)
}

fn main() {
    let file_name = "../puzzle/5_input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}
