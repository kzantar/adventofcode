#![allow(dead_code, unused)]

use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Formatter;

use common::read_puzzle;

#[derive(Debug)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<char>,
}

impl Matrix {
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.cols + c
    }

    fn get(&self, r: usize, c: usize) -> Option<&char> {
        if r < self.rows && c < self.cols {
            let i = self.idx(r, c);
            Some(&self.data[self.idx(r, c)])
        } else {
            None
        }
    }

    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut char> {
        if r < self.rows && c < self.cols {
            let i = self.idx(r, c);
            Some(&mut self.data[i])
        } else {
            None
        }
    }

    fn new(rows: usize, cols: usize, fill: char) -> Self {
        Self {
            rows,
            cols,
            data: vec![fill; rows * cols],
        }
    }

    fn from_string(s: &str) -> Self {
        let rows = s.lines().count();
        let cols = s.lines().next().unwrap().chars().count();
        let mut data = Vec::with_capacity(rows * cols);

        for line in s.lines() {
            for c in line.chars() {
                data.push(c);
            }
        }

        Self { rows, cols, data }
    }

    fn neighbors(&self, r: usize, c: usize) -> u32 {
        let cell = self.get(r, c).unwrap();

        // We check only cell with roll of paper (@)
        if cell != &'@' {
            return 0;
        }

        let mut cnt = 0u32;
        let dirs: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (d_r, d_c) in dirs {
            let new_r = r.checked_add_signed(d_r);
            let new_c = c.checked_add_signed(d_c);
            // Skip invalid indexis
            if new_r.is_none() || new_c.is_none() {
                continue;
            }

            if let Some(c) = self.get(new_r.unwrap(), new_c.unwrap()) {
                if c == &'@' {
                    cnt += 1;
                }
            }
        }

        cnt
    }

    fn remove(&mut self, r: usize, c: usize) {
        if let Some(cell) = self.get_mut(r, c) {
            if *cell == '@' {
                *cell = '.';
            }
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                write!(f, "{}", self.data[self.idx(r, c)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve_part_1(puzzle: &str) -> u32 {
    let m = Matrix::from_string(puzzle);

    let mut result = 0u32;

    for r in 0..m.rows {
        for c in 0..m.cols {
            let Some(cell) = m.get(r, c) else {
                continue;
            };

            if *cell != '@' {
                continue;
            }

            if m.neighbors(r, c) < 4 {
                result += 1;
            }
        }
    }

    result
}

fn solve_part_2(puzzle: &str) -> u32 {
    let mut m = Matrix::from_string(puzzle);

    let mut total_result = 0u32;

    loop {
        let mut count = 0u32;

        for r in 0..m.rows {
            for c in 0..m.cols {
                let Some(cell) = m.get(r, c) else {
                    continue;
                };

                if *cell != '@' {
                    continue;
                }

                if m.neighbors(r, c) < 4 {
                    count += 1;
                    m.remove(r, c);
                }
            }
        }

        if count == 0 {
            break;
        } else {
            total_result += count;
        }
    }

    total_result
}

fn main() {
    let file_name = "../puzzle/4/input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    let mut m = Matrix::from_string(&puzzle);

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let m = Matrix::from_string(".@.\n.@.\n.@.");
        assert_eq!(m.neighbors(1, 1), 2);

        let m = Matrix::from_string("@@@\n@@@\n@@@");
        assert_eq!(m.neighbors(1, 1), 8);

        let m = Matrix::from_string("...\n.@.\n...");
        assert_eq!(m.neighbors(1, 1), 0);

        let m = Matrix::from_string("@@@\n@.@\n@@@");
        assert_eq!(m.neighbors(1, 1), 0);
    }

    #[test]
    fn test_remove() {
        let mut m = Matrix::from_string(".@.\n.@.\n.@.");
        m.remove(1, 1);
        assert_eq!(m.get(1, 1), Some(&'.'));

        m.remove(0, 1);
        assert_eq!(m.get(0, 1), Some(&'.'));

        // check out of the boundaries
        m.remove(10, 10);
        // expect silence
    }
}
