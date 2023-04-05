use common::read_puzzle;

fn get_items(puzzle: &String) -> Vec<u32> {
    let items: Vec<u32> = puzzle
        .lines()
        .map(|line| line.parse::<u32>().unwrap_or_default())
        .collect();
    items
}

fn solve_part_1(puzzle: &String) -> u32 {
    let items = get_items(puzzle);

    let largest: u32 = items
        .split(|n| *n == 0)
        .map(|elf| elf.iter().sum::<u32>())
        .max()
        .unwrap();
    
    largest
}

fn solve_part_2(puzzle: &String) -> u32 {
    let items = get_items(puzzle);
    
    let mut sums: Vec<u32> = items
        .split(|n| *n == 0)
        .map(|elf| elf.iter().sum::<u32>())
        .collect();
    
    sums.sort_by(|a, b| b.cmp(a));
    
    sums[..3].iter().sum()
}

fn main() {
    let file_name = "../puzzle/1_input.txt";
    let puzzle = read_puzzle(file_name).unwrap();
    
    println!("Answer for part 1 is {}", solve_part_1(&puzzle));
    println!("Answer for part 2 is {}", solve_part_2(&puzzle));
}