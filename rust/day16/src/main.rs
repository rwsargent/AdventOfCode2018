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
    let starting_program = (('a' as u8)..('q' as u8)).collect();
    let mut result = dance(&starting_program, &contents);

    // find permutations
    let mut index = 1;
    while result != starting_program {
        result = dance(&result, &contents);
        index += 1;
    }

    println!("index {}", index);

    let mut extra = 1000000000 % index;

    while extra > 0 {
        result = dance(&result, &contents);
        extra -= 1;
    }

    println!("finished: {:?}", String::from_utf8(result));
}

fn dance(in_programs: &Vec<u8>, dance: &String) -> Vec<u8> {
    let mut programs = in_programs.clone();

    for dance_move in dance.split(",") {
        let c = dance_move.chars().nth(0).unwrap();
        match c {
            's' => {
                let mut new_programs = programs.clone();
                let index = programs.len() - dance_move[1..].parse::<usize>().expect("spin");
                let mut i = 0;
                while i < programs.len() {
                    new_programs[i] = programs[(index + i) % programs.len()];
                    i += 1;
                }
                programs = new_programs;
            }
            'x' => {
                let inputs = dance_move[1..].split("/").map(|x| x.trim().parse::<usize>().expect("X-change")).collect::<Vec<_>>();
                let temp = programs[inputs[0]];
                programs[inputs[0]] = programs[inputs[1]];
                programs[inputs[1]] = temp;
            }
            'p' => {
                let inputs = dance_move[1..].split("/").map(|x| x.as_bytes()[0]).collect::<Vec<_>>();
                let mut swapped = 0;
                let mut i = 0;
                while swapped < 2 && i < programs.len() {
                    if programs[i] == inputs[0] {
                        programs[i] = inputs[1];
                        swapped += 1;
                    } else if programs[i] == inputs[1] {
                        programs[i] = inputs[0];
                        swapped += 1;
                    }
                    i += 1;
                }
            }
            _ => unimplemented!()
        }
    }

    programs
}


#[test]
fn graph_test() {
    assert_eq!(
        dance(&(('a' as u8)..('f' as u8)).collect(), &"s1,x3/4,pe/b".to_string()),
        "baedc".as_bytes().to_vec()
    )
}
