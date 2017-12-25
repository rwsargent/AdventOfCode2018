use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::num;
use std::ops::Index;
use std::slice::Split;

fn main() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let score = scan_firewall(&contents);

    println!("score {:?}", score);
}

fn scan_firewall(firewall:&String) -> u64 {
    let mut score = 0;
    firewall.split("\n").filter(|x| !x.is_empty()).for_each(|l| {
        let nums = l.split(":").map(|x| x.trim().parse::<u64>().expect("failed")).collect::<Vec<_>>();

        if (nums[0] % ((nums[1] - 1) * 2)) == 0 {
            score += nums[0] * nums[1];
        }
    });
    score
}

#[test]
fn test_firewall() {
    assert_eq!(scan_firewall(&"0: 3
1: 2
4: 4
6: 4".to_string()), 24);
}