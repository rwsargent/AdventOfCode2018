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

    let mut g = construct_graph(&contents);

    println!("root node: {:?}", g.nodes[g.root].name);

    let root = g.root;
    g.set_weights(root);
    println!("node to change: {:?}", g.find_unbalanced_child());
}

#[derive(Clone)]
struct Node {
    children: Vec<usize>,
    name: String,
    weight: u64,
    total_weight: u64,
    balanced: bool
}

struct Graph {
    nodes: Vec<Node>,
    root: usize
}

impl Graph {
    pub fn find_unbalanced_child(&mut self) -> (String, u64) {
        let mut current = self.root;
        self.set_weights(current);

        let mut last: usize = self.root;
        let mut peer_size: u64 = 0;

        let mut cont = true;
        while cont {
            let node = &self.nodes[current];
            let child_count = node.children.len();
            assert!(child_count == 0 || child_count > 2);
            // find unbalanced one and set to current
            let histogram = node.children.iter().map(|c| {
                (*c, self.nodes[*c].total_weight)
            }).fold(HashMap::new(), |mut m, x| {
                let v = m.remove(&x.1).unwrap_or_else(|| 0) + 1;
                m.insert(x.1, v);
                m
            });
            match histogram.iter()
                .find(|x| *x.1 == 1) {
                Some((unbalanced_weight, _)) => {
                    current = *node.children.iter().find(|c|{
                        self.nodes[**c].total_weight == *unbalanced_weight
                    }).unwrap();
                    peer_size = *histogram.iter().find(|x| *(*x).1 != 1).unwrap().0;
                }
                None => {
                    cont = false
                }
            }
        }

        let n = &self.nodes[current];
        println!("peer: {}", peer_size);
        println!("node: {:?}", (n.name.clone(), n.total_weight));
        (n.name.clone(), n.weight + peer_size - n.total_weight)
    }

    fn set_weights<'a>(&'a mut self, idx: usize) {
        let node = self.nodes[idx].clone();
//        let children = self.nodes[idx].children.clone();
        for c in node.children.iter() {
            self.set_weights(*c);
        }

//        let node = &mut self.nodes[idx as usize];
        if node.total_weight < node.weight {
            let mut sum = 0;
            for c in node.children {
//            let child_weight: u64 = node.children.iter().map(|c| {
//                self.set_weights(*c);
                sum += self.nodes[c].total_weight
            }

            self.nodes[idx as usize].total_weight = node.weight + sum;
        }
    }
}

fn construct_graph(input: &String) -> Graph {
    let mut nodes = HashMap::new();
    let mut graph = Graph { nodes: vec![], root: 0 };
    let mut temp_children = HashMap::new();
    let mut indices = HashSet::new();

    input.split("\n").filter(|x| x.len() > 0).for_each(|line| {
        let mut children = Vec::new();
        let mut idx = 0;
        let mut node_idx = 0;
        line.split_whitespace().for_each(|item| {
            match idx {
                0 => {
                    node_idx = graph.nodes.len();
                    graph.nodes.push(Node { children: vec![], name: item.to_string(), weight: 0, total_weight: 0, balanced: false });
                    nodes.insert(item.to_string(), node_idx);
                    indices.insert(node_idx);
                }
                1 => {
                    graph.nodes[node_idx].weight = item[1..(item.len() - 1)].parse::<u64>().unwrap();
                }
                2 => {}
                _ => {
                    let child = if (*item).to_string().as_bytes()[(item.len() - 1)] == ',' as u8 {
                        item[0..(item.len() - 1)].to_string()
                    } else {
                        item[..].to_string()
                    };
                    children.push(child);
                }
            }
            idx += 1;
        });
        temp_children.insert(node_idx, children);
    });
    // rebuild child links
    temp_children.iter().for_each(|(idx, children)| {
        let node = &mut graph.nodes[*idx];
        (*node).children = children.iter().map(|s| {
            let r = *nodes.get(s).unwrap();
            indices.remove(&r);
            r
        }).collect();
    });
    graph.root = *indices.iter().next().unwrap();
    graph
}

#[test]
fn graph_test() {
    let mut g = construct_graph(&"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)".to_string());
    assert_eq!(g.nodes[g.root].name, "tknk".to_string());
    assert_eq!(g.find_unbalanced_child(), ("ugml".to_string(), 60));
}
