use std::fs;

fn main() {
    let line = fs::read_to_string("input.txt").expect("Unable to read the file!");

    let banks: Vec<Vec<u32>> = line
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|numchar| numchar.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let ans_1: u32 = banks
        .iter()
        .map(|bank| bank_jolts_p1(bank))
        .sum();

    println!("Answer For Part 1: {}", ans_1)

}

fn bank_jolts_p1(bank: &[u32]) -> u32 {
    let mut current_highest = 0;
    for i in 0..bank.len()-1 {
        for j in i+1..bank.len() {
            let jolt = get_jolt_p1(bank[i], bank[j]);
            if jolt > current_highest {
                current_highest = jolt;
            }
        }
    }

    current_highest
}

fn get_jolt_p1(a: u32, b: u32) -> u32 {
    (a * 10) + b
}