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
    let input = env::args().nth(1).unwrap();
    let iterations = env::args().nth(2).map(|x| x.parse::<usize>().unwrap()).unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let side_length = (contents.len() as f64).sqrt() as i32;
    let mid = side_length / 2;
    // build state

    let infectedNodes =
        contents.into_bytes().into_iter().enumerate()
            .filter(|(i, x)| *x == '#' as u8)
            .map(|(i, x)| {
                let x = mid - ((i as i32) % side_length);
                let y = mid - ((i as i32) / side_length);
                (x, y)
            })
            .collect::<HashSet<_>>();


    let mut state = State {
        infectedNodes,
        virusPosition: (0, 0),
        virusDirection: Direction::Up,
    };

    let mut bursts = 0;
    for _ in 0..iterations {
        if state.burst() {
            bursts += 1;
        }
    }
    println!("Num bursts causing infection: {}", bursts);
}

#[derive(Debug, Clone)]
enum Direction { Up, Right, Down, Left }

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    virusPosition: (i32, i32),
    virusDirection: Direction,
    infectedNodes: HashSet<(i32, i32)>,
}

impl State {
    fn burst(&mut self) -> bool {
        let infected = self.infectedNodes.contains(&self.virusPosition);
        if infected {
            self.virusDirection = self.virusDirection.turn_right();
            self.infectedNodes.remove(&self.virusPosition);
        } else {
            self.virusDirection = self.virusDirection.turn_left();
            self.infectedNodes.insert(self.virusPosition.clone());
        }
        let (x, y) = self.virusPosition;
        self.virusPosition = match self.virusDirection {
            Direction::Up => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y - 1),
            Direction::Right => (x + 1, y),
        };
        !infected
    }
}

