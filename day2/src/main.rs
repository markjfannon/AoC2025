#![feature(iter_map_windows)]

use std::fs;

fn main() {
    let line = fs::read_to_string("input.txt").expect("Unable to read the file!");
    let ans_1: u64 = line
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map_windows(|[x, y]| (x.parse().unwrap(), y.parse().unwrap()))
                .map(|(x, y): (u64, u64)| (x..y).filter(|x| is_invalid_p1(*x)).sum::<u64>())
                .sum::<u64>()
        })
        .sum();

    let ans_2: u64 = line
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map_windows(|[x, y]| (x.parse().unwrap(), y.parse().unwrap()))
                .map(|(x, y): (u64, u64)| (x..y).filter(|x| is_invalid_p2(*x)).sum::<u64>())
                .sum::<u64>()
        })
        .sum();

    println!("First answer: {ans_1}");
    println!("Second answer: {ans_2}");

}

fn is_invalid_p1(num: u64) -> bool {
    let num_str = num.to_string();
    let mid = num_str.len() / 2;
    let (l, r) = num_str.split_at(mid);

    l == r
}

fn is_invalid_p2(num: u64) -> bool {
    let s = num.to_string();
    let ss = format!("{s}{s}");
    ss[1..ss.len() - 1].contains(&s)
}
