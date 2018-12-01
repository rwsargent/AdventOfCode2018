use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::num;
use std::ops::Index;
use std::slice::Split;


extern crate regex;

use regex::Regex;


fn main() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let RE: Regex = Regex::new(r"p=<(\-?\d+),.*?(\-?\d+),.*?(\-?\d+)>.*?v=<(\-?\d+),.*?(\-?\d+),.*?(\-?\d+)>.*?a=<(\-?\d+),.*?(\-?\d+),.*?(\-?\d+)>").unwrap();

    let mut particles: Vec<Particle> = contents.split("\n").filter(|x| !x.is_empty()).map(|x| Particle::from_str(&RE, x)).collect();

    for u in 0..10000 {
        for i in 0..particles.len() {
            if particles[i].alive {
                particles[i].step();
            }
        }
        let mut map: HashMap<Vector, Vec<usize>> = HashMap::new();
        for i in 0..particles.len() {
            if particles[i].alive {
                if map.contains_key(&particles[i].position) {
                    map.get_mut(&particles[i].position).unwrap().push(i);
                } else {
                    map.insert(particles[i].position.clone(), vec![i]);
                }
            }
        }
        for (_, v) in map {
            if v.len() > 1 {
                for i in v {
                    particles[i].alive = false;
                }
            }
        }
    }

    let mut min = i64::max_value();
    let mut min_idx = 0;
    for i in 0..particles.len() {
        let d = particles[i].position.manhattan_mag();
        if d < min {
            min = d;
            min_idx = i;
        }
    }

    let alive_count = particles.iter().filter(|x| x.alive).count();

    println!("partilce {}", min_idx);
    println!("alive particles {}", alive_count);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Vector {
    x: i64,
    y: i64,
    z: i64
}

impl Vector {
    fn plus(&self, other: &Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
    fn zero() -> Vector {
        Vector { x: 0, y: 0, z: 0 }
    }
    fn manhattan_mag(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
    alive: bool
}


impl Particle {
    fn step(&mut self) {
        self.velocity = self.velocity.plus(&self.acceleration);
        self.position = self.position.plus(&self.velocity);
    }

    fn from_str(RE: &Regex, string: &str) -> Particle {
        let cap = RE.captures_iter(string).next().unwrap();
        Particle {
            position: Vector {
                x: cap[1].parse::<i64>().unwrap(),
                y: cap[2].parse::<i64>().unwrap(),
                z: cap[3].parse::<i64>().unwrap(),
            },
            velocity: Vector {
                x: cap[4].parse::<i64>().unwrap(),
                y: cap[5].parse::<i64>().unwrap(),
                z: cap[6].parse::<i64>().unwrap(),
            },
            acceleration: Vector {
                x: cap[7].parse::<i64>().unwrap(),
                y: cap[8].parse::<i64>().unwrap(),
                z: cap[9].parse::<i64>().unwrap(),
            },
            alive: true
        }
    }
}

#[test]
fn particle_test() {
    let p = Particle::from_str("p=<1230,-475,-2427>, v=<167,-65,-346>, a=<-16,1,24>");
    assert_eq!(p, Particle {
        position: Vector { x: 1230, y: -475, z: -2427 },
        velocity: Vector { x: 167, y: -65, z: -346 },
        acceleration: Vector { x: -16, y: 1, z: 24 }
    });
}
