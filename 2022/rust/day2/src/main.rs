use common::read_puzzle;

fn score_1(round: &str) -> u32 {
    match round {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

fn score_2(round: &str) -> u32 {
    match round {
        "A X" => 3 + 0,
        "A Y" => 1 + 3,
        "A Z" => 2 + 6,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 2 + 0,
        "C Y" => 3 + 3,
        "C Z" => 1 + 6,
        _ => 0,
    }
}

fn solve_part_1(puzzle: &str) -> u32 {
    let result = puzzle.lines().map(|line| score_1(line)).sum();
    
    result
}

fn solve_part_2(puzzle: &str) -> u32 {
    let result = puzzle.lines().map(|line| score_2(line)).sum();

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
    fn test_score_1() {
        assert_eq!(score_1("A Y"), 8);
        assert_eq!(score_1("B X"), 1);
        assert_eq!(score_1("C Z"), 6);
    }

    #[test]
    fn test_score_2() {
        assert_eq!(score_2("A Y"), 4);
        assert_eq!(score_2("B X"), 1);
        assert_eq!(score_2("C Z"), 7);
    }
}