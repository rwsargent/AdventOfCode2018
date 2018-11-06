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


fn main() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    //
    let mut id_map = HashMap::new();
    // map from port to other ports
    let mut port_map: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    let component = contents.split("\n").map(|line| {
        let ports = line.split("/").map(|str| { str.parse::<usize>().unwrap() })
            .collect::<Vec<_>>();
        (ports[0], ports[1])
    })
        .enumerate()
        .for_each(|(i, (p1, p2))| {
            id_map.insert(i, (p1, p2));
            if port_map.contains_key(&p1) {
                port_map.get_mut(&p1).unwrap().push((i, p2));
            } else {
                port_map.insert(p1, vec![(i, p2)]);
            }

            if port_map.contains_key(&p2) {
                port_map.get_mut(&p2).unwrap().push((i, p1));
            } else {
                port_map.insert(p2, vec![(i, p1)]);
            }
        });

    let mut sorted = port_map.clone().into_iter().collect::<Vec<_>>();
    sorted.sort();
    for x in sorted {
        println!("{:?}", x);
    }
    println!("{:?}", id_map);

    let state = State { port_map, id_map: id_map.clone() };
    let (max, maxState) = state.recurse(&mut BitSet::new(), 0);

    let ms = maxState.iter().map(|i| id_map.get(i).unwrap().clone())
        .collect::<Vec<_>>();
    println!("max State: {:?}", maxState);
    println!("max ports: {:?}", ms);
    println!("max: {}", max);
}

struct State {
    port_map: HashMap<usize, Vec<(usize, usize)>>,
    id_map: HashMap<usize, (usize, usize)>,
}

impl State {
    fn recurse(&self, state: &mut BitSet, node: usize) -> (usize, Vec<usize>) {
//        println!("considering {}", node);
        let nodes = self.port_map.get(&node).map(|x| x.clone()).unwrap_or_else(|| { vec![] })
            .into_iter()
            .filter(|(i, p)| { !state.contains(*i) })
            .collect::<Vec<_>>();
        let mut max = 0;
        let mut max_i = -1;
        let mut maxState = vec![];
        for (i, p) in nodes {
            state.insert(i);
            let (c, c_state) = self.recurse(state, p);
            if c > max {
                max = c;
                max_i = i as i32;
                maxState = c_state;
            }
            state.remove(i);
        }
        if max_i >= 0 {
            maxState.push(max_i as usize);
            max += node;
        }
        (max + node, maxState)
    }
}

