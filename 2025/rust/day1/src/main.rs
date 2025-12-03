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
    fn left(&mut self, num: i32) -> &Self {
        let diff = (self.pos - num) % 100;
        if diff < 0 {
            self.pos = 100 + diff;
        } else {
            self.pos = diff;
        }
        self
    }

    fn right(&mut self, num: i32) -> &Self {
        let diff = self.pos + num;
        if diff >= 100 {
            self.pos = (diff - 100) % 100;
        } else {
            self.pos = diff;
        }
        self
    }
}


fn main() {
    let file_name = "../puzzle/1_test.txt";
    let puzzle = read_puzzle(file_name).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial_left() {
        let mut dial = Dial {
            pos: 50,
        };
        dial.left(10);
        assert_eq!(dial.pos, 40);

        dial.left(45);
        assert_eq!(dial.pos, 95);

        dial.left(200);
        assert_eq!(dial.pos, 95);
    }

    #[test]
    fn test_dial_right() {
        let mut dial = Dial {
            pos: 50,
        };
        dial.right(10);
        assert_eq!(dial.pos, 60);

        dial.right(45);
        assert_eq!(dial.pos, 5);

        dial.right(200);
        assert_eq!(dial.pos, 5);
    }
}