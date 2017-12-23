use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::slice::Split;
use std::num;
use std::cmp;

fn main() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("spreadsheet: {}\n", &contents);
    println!("checksum: {}", checksum(&contents));
    println!("checksum2: {}", checksum2(&contents));
}

fn checksum(input: &String) -> u64 {
    input.split("\n").map(|line| {
        let (min, max) = line.split_whitespace().map(|x| x.parse::<u64>().unwrap_or_else(|_| 0)).fold((u64::max_value(), u64::min_value()), |(min, max), i| {
            (cmp::min(min, i), cmp::max(max, i))
        });
        if max > min {
            max - min
        } else {
            0
        }
    }).fold(0, |a, b| a + b)
}

#[test]
fn test_checksum() {
    assert_eq!(checksum(&"5 1 9 5
7 5 3
2 4 6 8".to_string()), 18);
}

fn checksum2(input: &String) -> u64 {
    input.split("\n").map(|line| {
        let nums = line.split_whitespace().map(|x| x.parse::<u64>().unwrap_or_else(|_| 0)).collect::<Vec<_>>();
        if let Some((x, y)) = nums.clone().into_iter().fold(None, |r, x| {
            match r {
                Some(r) => Some(r),
                None => {
                    match nums.iter().find(|y| **y != x && x % *y == 0) {
                        Some(y) => Some((x, y)),
                        None => None
                    }
                }
            }
        }) {
            if x > *y {
                x / *y
            } else {
                *y / x
            }
        } else {
            0
        }
    }).fold(0, |a, b| a + b)
}

#[test]
fn test_checksum2() {
    assert_eq!(checksum2(&"5 9 2 8
9 4 7 3
3 8 6 5".to_string()), 9);
}


