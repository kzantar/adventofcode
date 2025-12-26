#![allow(dead_code, unused)]

use common::read_puzzle;

type Int = u64; 

#[derive(Debug, PartialEq)]
enum Operation {
    Plus,
    Product,
}

impl Operation {
    fn eval(&self, args: &Vec<Int>) -> Int {
        let res: Int = match self {
            Self::Plus => args.iter().copied().sum(),
            Self::Product => args.iter().copied().product(),
        };

        res
    }
}

fn parse_num_line(line: &str) -> Vec<Int> {
    line
        .split_ascii_whitespace()
        .map(|n| n.parse::<Int>().unwrap())
        .collect()
}

fn parse_oper_line(line: &str) -> Vec<Operation> {
    line
        .split_ascii_whitespace()
        .map(|s| {
            match s {
                "*" => Operation::Product,
                "+" => Operation::Plus,
                _ => panic!("Found unsupported operation!"),
            }
        })
        .collect()
}

fn solve_part_1(puzzle: &str) -> Int {
    let mut nums: Vec<Vec<Int>> = vec![];
    let mut operations: Vec<Operation> = vec![];
    
    for line in puzzle.lines() {
        if line.contains('+') {
            operations = parse_oper_line(line);
        } else if line.trim().is_empty() {
            continue;
        } else {
            nums.push(parse_num_line(line));
        }
    }

    let num_length: Vec<usize> = nums.iter().map(|l| l.len()).collect();
    let oper_length = operations.len();
    for length in num_length {
        assert_eq!(length, oper_length);
    }

    let mut result: Int = 0;
    for i in 0..oper_length {
        let mut nums_in_column: Vec<Int> = vec![];
        for j in 0..nums.len() {
            nums_in_column.push(nums[j][i]);
        }

        result += operations[i].eval(&nums_in_column);
    }


    result
}

fn solve_part_2(puzzle: &str) -> Int {
    let mut lines: Vec<&str> = puzzle.lines().collect();
    let oper_line = lines.pop().unwrap();
    let mut operations: Vec<Operation> = parse_oper_line(oper_line);
    operations.reverse();
    
    let mut sheet: Vec<Vec<char>> = vec![];
    for line in lines {
        let mut row: Vec<char> = line.chars().collect();
        sheet.push(row);
    }

    let rows = sheet.len();
    let cols = sheet[0].len();

    let mut nums: Vec<Vec<Int>> = vec![];
    let mut nums_col: Vec<Int> = vec![];

    for j in (0..cols).rev() {
        let mut col: Vec<char> = vec![];
        // check entire col
        for i in 0..rows {
            let ch = sheet[i][j];
            col.push(ch);
        }

        // if col consists only spaces then it starts new num_col
        let is_separator = col.iter().all(|ch| ch == &' ');
        if !is_separator {
            let num = String::from_iter(col).trim().parse::<Int>().unwrap();
            nums_col.push(num);
        } else {
            nums.push(nums_col.clone());
            nums_col.clear();
        }
    }

    // Don't forget last column
    if !nums_col.is_empty() {
        nums.push(nums_col.clone());
    }

    assert_eq!(nums.len(), operations.len());

    let mut total: Int = 0;
    for (i, oper) in operations.iter().enumerate() {
        match oper {
            Operation::Plus => total += nums[i].iter().sum::<Int>(),
            Operation::Product => total += nums[i].iter().product::<Int>(), 
        }
    }

    total
}

fn main() {
    let file_name = "../puzzle/6/input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_num_line() {
        assert_eq!(parse_num_line("123 328  51 64"), vec![123, 328, 51, 64]);
        assert_eq!(parse_num_line(" 45 64  387 23"), vec![45, 64, 387, 23]);
        assert_eq!(parse_num_line("  6 98  215 314"), vec![6, 98, 215, 314]);
    }

    #[test]
    fn test_parse_oper_line() {
        assert_eq!(parse_oper_line("*    +    "), vec![Operation::Product, Operation::Plus]);
        assert_eq!(parse_oper_line(" * + "), vec![Operation::Product, Operation::Plus]);
    }

    #[test]
    fn test_operation_eval() {
        let v = vec![2, 3, 4];
        assert_eq!(Operation::Plus.eval(&v), 9);
        assert_eq!(Operation::Product.eval(&v), 24);

        let v: Vec<Int> = vec![];
        assert_eq!(Operation::Plus.eval(&v), 0);
        assert_eq!(Operation::Product.eval(&v), 1);
    }
}