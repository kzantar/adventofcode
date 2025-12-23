#![allow(dead_code, unused)]

use std::collections::HashSet;

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

fn slices() {
    let vec = vec![1, 2, 3, 4];
    let int_slice = &vec[..2];

    for i in int_slice {
        println!("{}", i);
    }
}

fn vec_extends() {
    let mut dst = Vec::<u32>::new();
    let src = vec![1, 2, 3, 4, 5];

    dst.extend_from_slice(&src[..2]);

    println!("src = {:?}", src);
    println!("dst = {:?}", dst);
}

fn vec_reduce() {
    let mut v = vec![1, 0, 1, 0, 1, 0, 0, 1];

    let res = v.iter().all(|&x| x == 1);
    println!("{:?}", res);
}

fn vec_splits() {
    let v = vec![1, 2, 3, 4];

    println!("split_at(0) -> {:?}", v.split_at(0));
    println!("split_at(2) -> {:?}", v.split_at(2));
    println!("split_at(3) -> {:?}", v.split_at(3));

    println!("&v[..2] -> {:?}", &v[..2]);
    println!("&v[..3] -> {:?}", &v[..3]);
    println!("&v[3..] -> {:?}", &v[3..]);
    println!("&v[3..3] -> {:?}", &v[3..3]);
}

fn sets() {
    let mut nums: HashSet<u32> = HashSet::new();

    nums.extend([3, 4, 5].iter());
    nums.extend([4, 5, 6].iter());

    println!("{nums:?}");
    println!("Len of nums is {}", nums.len());
}

fn ranges() {
    let mut range = 1..=5;
    let col: Vec<u8> = range.collect();

    println!("{col:?}");
}

fn windows() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    for w in v.windows(2) {
        println!("{w:?}");
    }
}

fn main() {
    windows();
}
