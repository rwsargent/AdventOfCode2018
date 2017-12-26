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
    let input = env::args().skip(1).next().unwrap().parse::<usize>().unwrap();

    let result = spinlock(input);

    println!("next value: {}", result);

    let result = spinlock2(input);

    println!("value after 0 after 50000000: {}", result);
}

fn spinlock(steps: usize) -> i32 {
    let mut buffer = vec![0];
    let mut idx = 0;
    for i in 0..2017 {
        idx = (idx + steps) % buffer.len();
        buffer.insert(idx + 1, i + 1);
        idx += 1;
    }
    buffer[(idx + 1) % buffer.len()]
}

fn spinlock2(steps: usize) -> usize {
    let mut after_zero = 0;
    let mut idx = 0;
    for i in 1..50000001 {
        idx = (idx + steps) % i;
        if idx == 0 {
            after_zero = i;
        }
        idx += 1;
    }
    after_zero
}
