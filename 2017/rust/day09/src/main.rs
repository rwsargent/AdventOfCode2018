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

    let group_score = score(&contents);

    println!("(score, garbage) = {:?}", group_score);
}

fn score(input: &String) -> (u64, u64) {
    let mut current_depth = 0;
    let mut total_score = 0;
    let mut in_garbage = false;
    let mut skip = false;
    let mut garbage_score = 0;
    input.chars().for_each(|c| {
        if skip {
            skip = false;
        } else {
            if in_garbage {
                match c {
                    '>' => {
                        in_garbage = false;
                    }
                    '!' => {
                        skip = true;
                    }
                    _ => {
                        garbage_score += 1;
                        // ignored
                    }
                }
            } else {
                match c {
                    '{' => {
                        current_depth += 1;
                    }
                    '<' => {
                        in_garbage = true;
                    }
                    '}' => {
                        total_score += current_depth;
                        current_depth -= 1;
                    }
                    _ => {
                        // do nothing
                    }
                }
            }
        }
    });
    (total_score, garbage_score)
}

#[test]
fn scoring_test() {
    assert_eq!(score(&"{}".to_string()).0, 1);
    assert_eq!(score(&"{{{}}}".to_string()).0, 6);
    assert_eq!(score(&"{{},{}}".to_string()).0, 5);
    assert_eq!(score(&"{{{},{},{{}}}}".to_string()).0, 16);
    assert_eq!(score(&"{<a>,<a>,<a>,<a>}".to_string()).0, 1);
    assert_eq!(score(&"{{<ab>},{<ab>},{<ab>},{<ab>}}".to_string()).0, 9);
    assert_eq!(score(&"{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string()).0, 9);
    assert_eq!(score(&"{{<a!>},{<a!>},{<a!>},{<ab>}}".to_string()).0, 3);
}
