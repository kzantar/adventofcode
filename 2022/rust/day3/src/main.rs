use common::read_puzzle;

fn priority(item: char) -> usize {
    match item {
        'a'..='z' => ('a'..='z').position(|ch| ch == item).unwrap() + 1,
        'A'..='Z' => ('A'..='Z').position(|ch| ch == item).unwrap() + 27,
        _ => 0,
    }
}

fn split(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn find_item(first: &str, second: &str) -> Option<char> {
    for ch in first.chars() {
        if second.contains(ch) {
            return Some(ch);
        }
    }

    None
}

fn find_item_in_group(first: &str, second: &str, third: &str) -> Option<char> {
    for ch in first.chars() {
        if second.contains(ch) && third.contains(ch) {
            return Some(ch);
        }
    }

    None
}

fn solve_part_1(puzzle: &str) -> usize {
    puzzle
        .lines()
        .map(split)
        .map(|(f, s)| find_item(f, s).unwrap_or('_'))
        .map(priority)
        .sum()
}

fn solve_part_2(puzzle: &str) -> usize {
    let lines: Vec<&str> = puzzle.lines().collect();
    let mut sum: usize = 0;
    for i in (0..lines.len()).step_by(3) {
        let (f, s, t) = (lines[i], lines[i + 1], lines[i + 2]);
        let item = find_item_in_group(f, s, t).unwrap_or('_');
        sum += priority(item);
    }

    sum
}

fn main() {
    let file_name = "../puzzle/3_input.txt";
    let puzzle = read_puzzle(file_name).unwrap();

    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('L'), 38);
        assert_eq!(priority('P'), 42);
    }

    #[test]
    fn test_split() {
        assert_eq!(split("aabb"), ("aa", "bb"));
        assert_eq!(split("aabbcc"), ("aab", "bcc"));
    }

    #[test]
    fn test_find_item() {
        assert_eq!(find_item("abc", "ade"), Some('a'));
        assert_eq!(find_item("abc", "bre"), Some('b'));
        assert_eq!(find_item("abc", "abd"), Some('a'));
        assert_eq!(find_item("abc", "def"), None);
    }

    #[test]
    fn test_find_item_in_group() {
        assert_eq!(find_item_in_group("abc", "ade", "aqw"), Some('a'));
        assert_eq!(find_item_in_group("abcf", "zxcf", "qwef"), Some('f'));
        assert_eq!(find_item_in_group("abcf", "zxcf", "qwe"), None);
        assert_eq!(find_item_in_group("abc", "zxc", "qwe"), None);
    }
}
