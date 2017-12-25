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

    let mut a_prev = 0;
    let mut b_prev = 0;

    for l in contents.split("\n") {
        if l.contains("A") {
            let v = l.split_whitespace().collect::<Vec<_>>();
            a_prev = v[v.len() - 1].parse::<u64>().unwrap();
        } else if l.contains("B") {
            let v = l.split_whitespace().collect::<Vec<_>>();
            b_prev = v[v.len() - 1].parse::<u64>().unwrap();
        }
    }

    let mut generator_a = Generator {
        previous: a_prev,
        factor: 16807,
    };
    let mut generator_b = Generator {
        previous: b_prev,
        factor: 48271,
    };


    let mask = (1 << 16) - 1;
    let mut value = 0;
    let mut i = 0;
    while i < 40000000 {

        if (generator_a.generate() & mask) == (generator_b.generate() & mask) {
            value += 1;
        }

        i += 1
    }

    println!("matches: {}", value);
}

struct Generator {
    previous: u64,
    factor: u64,
}

impl Generator {
    fn generate(&mut self) -> u64 {
        let next = (self.previous * self.factor) % 2147483647;
        self.previous = next;
        next
    }
}

#[test]
fn test_generator() {
    let mut genA = Generator { previous: 65, factor: 16807 };
    let mut genB = Generator { previous: 8921, factor: 48271 };
    assert_eq!(genA.generate(), 1092455);
    assert_eq!(genB.generate(), 430625591);

    assert_eq!(genA.generate(), 1181022009);
    assert_eq!(genB.generate(), 1233683848);

    assert_eq!(genA.generate(), 245556042);
    assert_eq!(genB.generate(), 1431495498);

    assert_eq!(genA.generate(), 1744312007);
    assert_eq!(genB.generate(), 137874439);

    assert_eq!(genA.generate(), 1352636452);
    assert_eq!(genB.generate(), 285222916);


}
