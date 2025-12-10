#![allow(dead_code, unused)]

fn reminders() {
    println!("-1 / 100 = {}", -1 / 100);
    println!("-1 % 100 = {}", -1 % 100);
    println!("1 / 100 = {}", 1 / 100);
    println!("1 % 100 = {}", 1 % 100);
    println!("-101 / 100 = {}", -101 / 100);
    println!("101 / 100 = {}", 101 / 100);
    println!("199 / 100 = {}", 199 / 100);
    println!("-101 % 100 = {}", -101 % 100);
    println!("-18 % 100 = {}", -18 % 100);
    println!("-100 % 100 = {}", -100 % 100);
}

fn ranges() {
    for s in '1'..='5' {
        println!("{:?}", s);
    }
}

fn chunks() {
    let s = "112233".to_string();
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    println!("{:?}", chars);
    for i in 1..=n/2 {
        println!("i = {i}");
        let mut chunks = chars.chunks(i);
        let first = chunks.next().unwrap();
        for rest in chunks {
            if first != rest {
                return false;
            }
        }
    }
}

fn slices() {
    let vec = vec![1, 2, 3, 4];
    let int_slice = &vec[..2];

    for i in int_slice {
        println!("{}", i);
    }
}


fn main() {
    chunks();
}
