#![allow(dead_code, unused)]

use std::fmt::Display;
use std::fmt::Formatter;

use common::read_puzzle;

type Int = u64;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Kind {
    Start,
    Space,
    Splitter,
    Beam
}

impl Kind {
    fn from_char(ch: char) -> Kind {
        match ch {
            '.' => Kind::Space,
            'S' => Kind::Start,
            '^' => Kind::Splitter,
            '|' => Kind::Beam,
            _ => Kind::Space,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
struct Cell {
    r: usize,
    c: usize,
    kind: Kind,
    is_split: bool,
}

impl Cell {
    fn new(r: usize, c: usize, ch: char) -> Cell {
        Cell {
            r,
            c,
            kind: Kind::from_char(ch),
            is_split: false,
        }
    }

    fn change(&mut self, kind: Kind) {
        self.kind = kind;
    }

    fn beam(&mut self) {
        if self.kind == Kind::Space {
            self.kind = Kind::Beam;
        }
    }

    fn split(&mut self) {
        self.is_split = true;
    }

    fn is_space(&self) -> bool {
        self.kind == Kind::Space
    }

    fn is_beam(&self) -> bool {
        self.kind == Kind::Beam
    }

    fn is_splitter(&self) -> bool {
        self.kind == Kind::Splitter
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ch: char = match self.kind {
            Kind::Space => '.',
            Kind::Start => 'S',
            Kind::Splitter => '^',
            Kind::Beam => '|',
        };
        write!(f, "{}", ch);

        Ok(())
    }
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<Cell>,
}

impl Grid {
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.cols + c
    }

    fn get(&self, r: usize, c: usize) -> Option<&Cell> {
        if r < self.rows && c < self.cols {
            let i = self.idx(r, c);
            Some(&self.data[self.idx(r, c)])
        } else {
            None
        }
    }

    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Cell> {
        if r < self.rows && c < self.cols {
            let i = self.idx(r, c);
            Some(&mut self.data[i])
        } else {
            None
        }
    }

    fn new(rows: usize, cols: usize) -> Self {
        let mut data: Vec<Cell> = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                let cell = Cell::new(r, c, '.');
                data.push(cell);
            }
        }

        Self {
            rows,
            cols,
            data,
        }
    }

    fn from_string(s: &str) -> Self {
        let rows = s.lines().count();
        let cols = s.lines().next().unwrap().chars().count();
        let mut data: Vec<Cell> = Vec::with_capacity(rows * cols);

        for (i, line) in s.lines().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let cell = Cell::new(i, j, ch);
                data.push(cell);
            }
        }

        Self { rows, cols, data }
    }

    fn split_count(&self) -> Int {
        let mut result: Int = 0;
        for cell in &self.data {
            if cell.is_split {
                result += 1;
            }
        }

        result
    }

    fn run_1(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let cell = self.get(r, c).unwrap();
                match cell.kind {
                    Kind::Start => {
                        let mut next_cell = self.get_mut(r+1, c).unwrap();
                        if next_cell.is_space() {
                            next_cell.change(Kind::Beam);
                        }
                    },
                    Kind::Beam => {
                        let Some(next_cell) = self.get_mut(r+1, c) else {
                            continue;
                        };
                        if next_cell.is_space() {
                            next_cell.change(Kind::Beam);
                        }
                    },
                    Kind::Splitter => {
                        let Some(up_r) = r.checked_sub(1) else {
                            continue;
                        };
                        let up_cell = self.get(up_r, c).unwrap();
                        if up_cell.is_beam() {
                            self.get_mut(r, c-1).unwrap().beam();
                            self.get_mut(r, c+1).unwrap().beam();
                            self.get_mut(r, c).unwrap().split();
                        }
                    },
                    Kind::Space => {
                        let Some(up_r) = r.checked_sub(1) else {
                            continue;
                        };
                        let Some(up_cell) = self.get(up_r, c) else {
                            continue;
                        };
                        if up_cell.is_beam() {
                            self.get_mut(r, c).unwrap().beam();
                        }
                    }
                }
            }
        }
    }
}

impl Display for Grid {
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


fn solve_part_1(puzzle: &str) -> Int {
    let mut grid = Grid::from_string(&puzzle);
    // println!("{grid}");

    grid.run_1();
    // println!("{grid}");

    grid.split_count()
}

fn solve_part_2(puzzle: &str) -> Int {
    0
}


fn main() {
    let file_name = "../puzzle/7/input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_from_char() {
        assert_eq!(Kind::from_char('.'), Kind::Space);
        assert_eq!(Kind::from_char('S'), Kind::Start);
        assert_eq!(Kind::from_char('^'), Kind::Splitter);
        assert_eq!(Kind::from_char('|'), Kind::Beam);
    }

    #[test]
    fn test_cell_new() {
        assert_eq!(Cell::new(0, 0, '.'), Cell {r: 0, c: 0, kind: Kind::Space, is_split: false });
        assert_eq!(Cell::new(0, 0, 'S'), Cell {r: 0, c: 0, kind: Kind::Start, is_split: false });
        assert_eq!(Cell::new(0, 0, '^'), Cell {r: 0, c: 0, kind: Kind::Splitter, is_split: false });
        assert_eq!(Cell::new(0, 0, '|'), Cell {r: 0, c: 0, kind: Kind::Beam, is_split: false });
        assert_eq!(Cell::new(0, 0, 'x'), Cell {r: 0, c: 0, kind: Kind::Space, is_split: false });
    }
}