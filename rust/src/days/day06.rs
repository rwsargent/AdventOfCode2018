/*use std::cmp::{max, min, PartialEq};
use std::collections::{HashMap, VecDeque};
use util::cursor::{Bounds, Cursor};
use std::borrow::BorrowMut;*/

pub fn run() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> String {
    return String::from("");
}

fn part_two() -> String {
    return String::from("");
}
/*
fn find_largest_sphere(input : Vec<String>) -> i32 {
    let mut islands = &mut parse_islands(input);
    let bounds = find_bounds(&islands);
    let mut board = HashMap::new();
    let island_keys : Vec<&Cursor>;
    {
        island_keys = islands.borrow_mut().keys().collect();
    }
    for island_key in island_keys {
        fill_spheres_of_influence(island_key, &bounds, &mut board, &mut islands);
    }
    return 0i32;
}

fn fill_spheres_of_influence (
    island_key: &Cursor,
    bounds: &Bounds,
    board : &mut HashMap<Cursor, Owner>,
    islands : &mut HashMap<Cursor, Island>
) {
    // Standard breath first search;
    let mut queue = VecDeque::new();
    // add neighbors to queue;
    let island = islands.get(island_key).unwrap();
    queue.push_back(island.cursor.up());
    queue.push_back(island.cursor.left());
    queue.push_back(island.cursor.right());
    queue.push_back(island.cursor.down());
    while queue.len() != 0 {
        let curr = queue.pop_front().unwrap();
        let dist = island.manh_dist_to(curr);
        let owner = board.entry(curr).or_insert(Owner {
            island_key : curr,
            dist: dist,
        });
        if !curr.in_bounds(*bounds) { // If tending toward infinity
            continue;
        }
        if owner.island_key == island.cursor || owner.dist >= dist { // if already seen, or distance doesn't matter
            continue;
        }
        // Update the current owner of the square
        if owner.dist > dist {
            islands.entry(owner.island_key).or_default().size -= 1;
            owner.dist = dist;

            islands.entry(*island_key).or_default().size += 1;
            owner.island_key = island.cursor;
        }
        // push neighbors
        queue.push_back(curr.up());
        queue.push_back(curr.down());
        queue.push_back(curr.left());
        queue.push_back(curr.right());
    }
}

fn parse_islands(input: Vec<String>) -> HashMap<Cursor, Island> {
    let mut islands = HashMap::new();
    input
        .iter()
        .map(|line| Island {
            cursor: parse_cursor(&line),
            size: 0,
        })
        .for_each(|island| {islands.insert(island.cursor, island);});
    islands
}

fn find_bounds(islands: &HashMap<Cursor, Island>) -> Bounds {
    let mut bound = Bounds {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    for island in islands.keys() {
        bound.top = min(bound.top, island.y);
        bound.bottom = max(bound.bottom, island.y);

        bound.left = min(bound.left, island.x);
        bound.right = max(bound.right, island.x);
    }
    bound
}

fn parse_cursor(line: &String) -> Cursor {
    let split: Vec<i32> = line.split(", ").map(|num| num.parse().unwrap()).collect();
    Cursor {
        x: split[0],
        y: split[1],
    }
}

struct Owner {
    island_key : Cursor,
    dist: i32,
}

#[derive(Default)]
struct Island {
    cursor: Cursor,
    size: i32,
}

impl Island {
    fn manh_dist_to(&self, point: Cursor) -> i32 {
        (self.cursor.x - point.x).abs() + (self.cursor.y - point.y).abs()
    }
}

impl PartialEq for Island {
    fn eq(&self, other: &Island) -> bool {
        return self.cursor == other.cursor;
    }
}
*/
