use std::collections::HashSet;
use util::input::get_input;

pub fn part_one() -> i32 {
    let input = get_input("inputs/day01.txt").as_ints();
    input.iter().sum()
}

pub fn part_two() -> i32 {
    let input = get_input("inputs/day01.txt").as_ints();
    let mut freq = 0;
    let mut freqs = HashSet::new();
    loop {
        for change in &input {
            freq += change;
            if !freqs.insert(freq) {
                return freq;
            }
        }
    }
}

pub fn run() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
