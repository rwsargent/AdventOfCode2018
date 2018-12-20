use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet};
use util::input::get_input;

pub fn run() {
    println!("Part one: {}", part_one());
}

fn part_one() -> String {
    //    let input = get_input("inputs/day07.txt").as_strings();
    let input = get_input("../node/day07/input.txt").as_strings();
    let (graph, nodes) = build_graph(input);
    let rev_topo_order = topo_sort(&graph, &nodes);
    rev_topo_order.iter().rev().map(|s| s.to_string()).collect()
}

fn build_graph(lines: Vec<String>) -> (HashMap<String, BTreeSet<String>>, BTreeSet<String>) {
    let mut graph = HashMap::new();
    let mut nodes = BTreeSet::new();
    let RE = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin").unwrap();
    for line in lines {
        let caps = RE.captures(&line).unwrap();
        graph
            .entry(String::from(caps[1].to_string()))
            .or_insert(BTreeSet::new())
            .insert(caps[2].to_string());
        nodes.insert(caps[2].to_string());
        nodes.insert(caps[1].to_string());
    }
    (graph, nodes)
}

fn topo_sort(graph: &HashMap<String, BTreeSet<String>>, nodes: &BTreeSet<String>) -> Vec<String> {
    let mut topo_order = Vec::new();
    let mut perm_visited = HashSet::new();
    for node in nodes.iter().rev() {
        // reverse order to make sure we visit in correct order.
        dfs(
            &node,
            &mut perm_visited,
            &mut HashSet::new(),
            &graph,
            &mut topo_order,
        );
    }
    topo_order
}

fn dfs(
    node: &String,
    perm_visited: &mut HashSet<String>,
    temp_visited: &mut HashSet<String>,
    graph: &HashMap<String, BTreeSet<String>>,
    topo_order: &mut Vec<String>,
) {
    if perm_visited.contains(node) {
        return;
    }
    if temp_visited.contains(&node.to_string()) {
        panic!("This isn't a DAG, you scoundral!")
    }
    temp_visited.insert(node.to_string());
    let edges = graph.get(node);
    if edges.is_some() {
        for edge in edges.unwrap().iter().rev() {
            dfs(edge, perm_visited, temp_visited, graph, topo_order);
        }
    }
    perm_visited.insert(node.to_string());
    topo_order.push(node.to_string());
}

fn do_work(graph: HashSet<String, BTreeSet<String>>, nodes: BTreeSet<String>) {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = vec![
            String::from("Step C must be finished before step A can begin."),
            String::from("Step C must be finished before step F can begin."),
            String::from("Step A must be finished before step B can begin."),
            String::from("Step A must be finished before step D can begin."),
            String::from("Step B must be finished before step E can begin."),
            String::from("Step D must be finished before step E can begin."),
            String::from("Step F must be finished before step E can begin."),
        ];
        let (graph, nodes) = build_graph(input);
        let order = topo_sort(&graph, &nodes);
        assert_eq!(order, vec!["E", "F", "D", "B", "A", "C"]);
    }
}
