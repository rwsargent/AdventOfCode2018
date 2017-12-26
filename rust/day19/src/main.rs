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

    let plain: Plain = Plain {
        rows: contents.split("\n").filter(|x| !x.is_empty()).map(|x| x.chars().collect()).collect()
    };
    let (x, _) = plain.rows[0].iter().enumerate().find(|x| *x.1 == '|').unwrap();
    let mut location = Point { x: x as i32, y: 0 };
    use Direction::*;
    let mut dir = &Up;
    let mut result = vec![];
    let mut steps = 0;
    while plain.is_valid(&location) {
        steps += 1;
        println!("{:?} - {:?} - {:?}", location, dir, plain.at(&location));
        match plain.at(&location) {
            '+' => {
                let opposite = Point { x: -dir.x, y: -dir.y };
                let new_dir = AllLocations.iter().filter(|x| ***x != opposite).find(|x| {
                    plain.at(&location.plus(x)) != ' '
                }).unwrap();
                dir = *new_dir;
            }
            '|' | '-' => {}
            x => {
                result.push(x)
            }
        }
        location = location.plus(&dir);
    }
    let result: String = result.iter().collect();
    println!("path: {:?}", result);
    println!("steps: {:?}", steps);
}

pub struct Plain {
    rows: Vec<Vec<char>>
}

impl Plain {
    fn at(&self, p: &Point) -> char {
        self.rows[p.y as usize][p.x as usize]
    }
    fn is_valid(&self, p: &Point) -> bool {
        p.y >= 0 && p.y < self.rows.len() as i32 && p.x >= 0 && p.x < self.rows[0].len() as i32 && self.at(p) != ' '
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn plus(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

mod Direction {
    use super::Point;

    pub static Up: Point = Point { x: 0, y: 1 };
    pub static Down: Point = Point { x: 0, y: -1 };
    pub static Left: Point = Point { x: -1, y: 0 };
    pub static Right: Point = Point { x: 1, y: 0 };
    pub static AllLocations: [&Point; 4] = [&Up, &Left, &Down, &Right];
}

#[test]
fn graph_test() {
    let mut g = construct_graph(&"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5".to_string());
    assert_eq!(g.nodes[g.root].name, "tknk".to_string());
    assert_eq!(g.find_unbalanced_child(), ("ugml".to_string(), 60));
}
