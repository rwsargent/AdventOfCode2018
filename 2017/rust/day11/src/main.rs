use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::num;
use std::ops::Index;
use std::slice::Split;
use std::cmp::max;

fn main() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");


    println!("hash: {:?}", steps_from_center(&contents));
}

fn steps_from_center(path: &String) -> (i64, i64) {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;
    let mut max_dist = 0;

    path.split(",").for_each(|dir| {
        match dir.trim() {
            "nw" => {
                x -= 1;
                y += 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            "n" => {
                z -= 1;
                y += 1;
            }
            "s" => {
                y -= 1;
                z += 1;
            }
            "sw" => {
                x -= 1;
                z += 1;
            }
            "ne" => {
                x += 1;
                z -= 1;
            }
            x => {
                println!("x: {:?}", x);
                unimplemented!()
            }
        }
        if x.abs() > max_dist {
            max_dist = x.abs();
        }
        if z.abs() > max_dist {
            max_dist = z.abs();
        }
        if y.abs() > max_dist {
            max_dist = y.abs();
        }
    });

    (max(max(x.abs(), y.abs()), z.abs()), max_dist)
}

#[test]
fn test_steps() {
    assert_eq!(steps_from_center(&"ne,ne,ne".to_string()).1, 3);
    assert_eq!(steps_from_center(&"ne,ne,sw,sw".to_string()).1, 0);
    assert_eq!(steps_from_center(&"ne,ne,s,s".to_string()).1, 2);
    assert_eq!(steps_from_center(&"se,sw,se,sw,sw".to_string()).1, 3);
}
