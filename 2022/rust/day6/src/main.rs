use common::read_puzzle;

const PACKET_LEN: usize = 4;
const MESSAGE_LEN: usize = 14;

fn is_marker(packet: &str, len: usize) -> bool {
    assert_eq!(packet.len(), len);
    
    let mut v: Vec<_> = packet.chars().collect();
    v.sort();
    v.dedup();
    
    v.len() == len
}

fn solve_part_1(puzzle: &str) -> usize {
    for i in PACKET_LEN .. puzzle.len() {
        if is_marker(&puzzle[i-PACKET_LEN .. i], PACKET_LEN) {
            return i
        }
    }
    
    0
}

fn solve_part_2(puzzle: &str) -> usize {
    for i in MESSAGE_LEN .. puzzle.len() {
        if is_marker(&puzzle[i-MESSAGE_LEN .. i], MESSAGE_LEN) {
            return i
        }
    }
    
    0
}

fn main() {
    let file_name = "../puzzle/6_input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_marker() {
        assert!(is_marker("jpqm", 4));
        assert!(!is_marker("mjqj", 4));
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(7, solve_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, solve_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, solve_part_1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, solve_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, solve_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(19, solve_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, solve_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, solve_part_2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, solve_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, solve_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}