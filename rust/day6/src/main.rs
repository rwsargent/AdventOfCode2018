use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::slice::Split;
use std::num;
use std::cmp;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("redistributions until cycle: {:?}", count_redistributions_until_cycle(&contents).unwrap());
}

fn count_redistributions_until_cycle(input: &String) -> Result<(u64, u64), &'static str> {
    let mut state = input.split_whitespace()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let len = state.len();
    let mut successful_add = None;
    let mut iterations: u64 = 0;
    let mut states = HashMap::new();
    states.insert(state.clone(), 0);
    while successful_add.is_none() {
        let mut max_index = 0;
        let mut max_value = state[0];
        for i in 1..len {
            if state[i] > max_value {
                max_value = state[i];
                max_index = i;
            }
        }
        state[max_index] = 0;
        while max_value > 0 {
            max_index = max_index + 1;
            if max_index == len {
                max_index = 0;
            }
            state[max_index] += 1;
            max_value -= 1;
        }
        iterations += 1;
        successful_add = states.insert(state.clone(), iterations);
    }
    Ok((iterations, iterations - successful_add.unwrap()))
}

#[test]
fn redistributions_test() {
    assert_eq!(count_redistributions_until_cycle(&"0 2 7 0".to_string()).unwrap(), (5, 4));
}
