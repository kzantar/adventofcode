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

fn main() {
    vec_extends();
    vec_reduce();
}
