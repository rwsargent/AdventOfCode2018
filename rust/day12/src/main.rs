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

    let (mut g, id_map) = construct_graph(&contents);

    println!("0 node group size: {:?}", g.count_group_size(*id_map.get(&"0".to_string()).unwrap()));
    println!("number of groups: {:?}", g.count_groups());
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
    fn get_group(&self, idx: usize) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut unvisited = vec![idx];
        while !unvisited.is_empty() {
            let next = unvisited.pop().unwrap();
            visited.insert(next);
            self.nodes[next].children.iter().for_each(|c| {
                if !visited.contains(c) {
                    unvisited.push(*c);
                }
            });
        }
        visited
    }

    fn count_group_size(&self, idx: usize) -> usize {
        self.get_group(idx).len()
    }

    fn count_groups(&self) -> usize {
        let mut groups: Vec<HashSet<usize>> = Vec::new();
        for i in 0..self.nodes.len() {
            let mut group_idx = 0;
            let mut found = false;
            while !found && group_idx < groups.len() {
                let g = &groups[group_idx];
                if g.contains(&i) {
                    found = true;
                } else {
                    group_idx += 1;
                }
            }
            if !found {
                let group = self.get_group(i);
                groups.push(group);
            }
        }
        groups.len()
    }
}

fn construct_graph(input: &String) -> (Graph, HashMap<String, usize>) {
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
//                1 => {
//                    graph.nodes[node_idx].weight = item[1..(item.len() - 1)].parse::<u64>().unwrap();
//                }
                1 => {}
                _ => {
                    let child = if (*item).to_string().as_bytes()[(item.len() - 1)] == ',' as u8 {
                        item[0..(item.len() - 1)].to_string().trim().to_string()
                    } else {
                        item[..].to_string().trim().to_string()
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
            let r = *nodes.get(s).expect(format!("expected {:?}", s).as_str());
            indices.remove(&r);
            r
        }).collect();
    });
//    graph.root = *indices.iter().next().expect("expected root");
    (graph, nodes)
}

#[test]
fn graph_test() {
    let (mut g, id_map) = construct_graph(&"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5".to_string());
    println!("{:?}", id_map);
    assert_eq!(g.count_group_size(*id_map.get(&"0".to_string()).unwrap()), 6);
}
