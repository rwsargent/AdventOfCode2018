use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::num;
use std::ops::Index;
use std::slice::Split;

extern crate bit_set;

use bit_set::BitSet;

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct Tape {
    neg: BitSet,
    pos: BitSet,
    cur: i64,
    state: State,
}

fn main() {
    let input = env::args().skip(1).next().unwrap();
    let mut iterations: u64 = input.parse().unwrap();

    let mut tape = Tape {
        neg: BitSet::new(),
        pos: BitSet::new(),
        cur: 0,
        state: State::A,
    };

    while iterations > 0 {
        step(&mut tape);
        iterations -= 1;
    }
    println!("checksum: {}", tape.neg.len() + tape.pos.len());
}

enum Direction { Left, Right }

fn step(tape: &mut Tape) {
    let neg = tape.cur < 0;
    let cur = if neg {
        tape.neg.contains((-tape.cur) as usize)
    } else {
        tape.pos.contains(tape.cur as usize)
    };
    use Direction::*;
    use State::*;
    let (output, dir, state) = match tape.state {
        State::A =>
            if !cur {
                (1, Right, B)
            } else {
                (0, Right, C)
            },
        State::B =>
            if !cur {
                (0, Left, A)
            } else {
                (0, Right, D)
            },
        State::C =>
            if !cur {
                (1, Right, D)
            } else {
                (1, Right, A)
            },
        State::D =>
            if !cur {
                (1, Left, E)
            } else {
                (0, Left, D)
            },
        State::E =>
            if !cur {
                (1, Right, F)
            } else {
                (1, Left, B)
            },
        State::F =>
            if !cur {
                (1, Right, A)
            } else {
                (1, Right, E)
            }
    };
    if neg {
        if output == 1 {
            tape.neg.insert((-tape.cur) as usize);
        } else {
            tape.neg.remove((-tape.cur) as usize);
        }
    } else {
        if output == 1 {
            tape.pos.insert(tape.cur as usize);
        } else {
            tape.pos.remove(tape.cur as usize);
        }
    }
    tape.state = state;
    tape.cur = match dir {
        Right => tape.cur + 1,
        Left => tape.cur - 1
    };
}


