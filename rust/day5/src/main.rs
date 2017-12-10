use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::slice::Split;
use std::num;
use std::cmp;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("escape in {} jumps", count_jumps(&contents));
    println!("escape in {} jumps 2", count_jumps2(&contents));
}

fn count_jumps(input: &String) -> u64 {
    let mut jumps = input.split("\n").filter(|x| x.len() > 0).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut index: i64 = 0;
    let mut steps: u64 = 0;
    while index < jumps.len() as i64 {
        let step = jumps[index as usize];
        jumps[index as usize]    = step + 1;
        index += step;
        steps += 1;
    }
    steps
}

#[test]
fn jump_test() {
    assert_eq!(count_jumps(&"0
3
0
1
-3".to_string()), 5);
}



fn count_jumps2(input: &String) -> u64 {
    let mut jumps = input.split("\n").filter(|x| x.len() > 0).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut index: i64 = 0;
    let mut steps: u64 = 0;
    while index < jumps.len() as i64 {
        let step = jumps[index as usize];
        if step >= 3 {
            jumps[index as usize] = step - 1;
        }  else {
            jumps[index as usize] = step + 1;
        }
        index += step;
        steps += 1;
    }
    steps
}

#[test]
fn jump_test2() {
    assert_eq!(count_jumps2(&"0
3
0
1
-3".to_string()), 10);
}
