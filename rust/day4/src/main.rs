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

    let num_valid = contents.split("\n").filter(|x| x.len() > 0).map(|x| {
        if valid_anagram(x) {
            1
        } else {
            0
        }
    }).fold(0, |a, b| a + b);
    println!("num valid passphrases: {}", num_valid);
}

fn valid_anagram(input: &str) -> bool {
    let vec = input.split_whitespace().collect::<Vec<_>>();
    let set = input.split_whitespace()
        .map(|s| {
            let mut sorted = s.chars().collect::<Vec<_>>();
            sorted.sort();
            String::from_iter(sorted.iter())
        }).collect::<HashSet<_>>();
    set.len() == vec.len()
}

#[test]
fn anagram_test() {
    assert_eq!(valid_anagram(&"abcde fghij".to_string()), true);
    assert_eq!(valid_anagram(&"abcde xyz ecdab".to_string()), false);
    assert_eq!(valid_anagram(&"aa bb cc dd aaa".to_string()), true);
    assert_eq!(valid_anagram(&"iiii oiii ooii oooi oooo".to_string()), true);
    assert_eq!(valid_anagram(&"oiii ioii iioi iiio".to_string()), false);
}

fn valid_passphrase(input: &str) -> bool {
    let vec = input.split_whitespace().collect::<Vec<_>>();
    let set = input.split_whitespace().collect::<HashSet<_>>();
    set.len() == vec.len()
}

#[test]
fn passphrase_test() {
    assert_eq!(valid_passphrase(&"aa bb cc dd ee".to_string()), true);
    assert_eq!(valid_passphrase(&"aa bb cc dd aa".to_string()), false);
    assert_eq!(valid_passphrase(&"aa bb cc dd aaa".to_string()), true);
}
