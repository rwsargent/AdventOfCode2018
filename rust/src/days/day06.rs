use util::cursor::{Cursor, Bounds};
use std::collections::{HashMap, VecDeque};
use std::cmp::{min, max, PartialEq};

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

fn parse_islands(input : Vec<String>) -> Vec<Island> {
    input.iter().map(|line| Island{cursor : parse_cursor(&line), size : 0}).collect()
}

fn find_bounds(islands : Vec<Island>) -> Bounds {
    let mut bound = Bounds{ left : 0, top: 0, right: 0, bottom : 0};
    for island in islands {
        bound.top = min(bound.top, island.cursor.y);
        bound.bottom = max(bound.bottom, island.cursor.y);

        bound.left = min(bound.left, island.cursor.x);
        bound.right = max(bound.right, island.cursor.x);
    }
    bound
}

fn fill_spheres_of_influence<'a>(island : &'a mut Island, bounds : Bounds, map : &mut HashMap<Cursor, Owner<'a>>) {
    // Standard breath first search;
    let mut queue = VecDeque::new();
    queue.push_back(island.cursor.up()); 
    queue.push_back(island.cursor.left()); 
    queue.push_back(island.cursor.right()); 
    queue.push_back(island.cursor.down());
    while queue.len() != 0 {
        let curr = queue.pop_front().unwrap();
        let dist = island.manh_dist_to(curr);
        let owner = map.entry(curr)
            .or_insert(Owner{island : island, dist : dist});
        if !curr.in_bounds(bounds) {
            continue;
        }
        if owner.island == island || owner.dist >= dist {
            continue;
        }
        if owner.dist > dist {
            owner.island.size -= 1;
            owner.dist = dist;

            island.size += 1;
            owner.island = island;
        }
        queue.push_back(curr.up());
        queue.push_back(curr.down());
        queue.push_back(curr.left());
        queue.push_back(curr.right());
    }
}

fn parse_cursor(line : &String) -> Cursor {
    let split : Vec<i32> = line.split(", ").map(|num| num.parse().unwrap()).collect();
    Cursor {x : split[0], y: split[1]}
}


struct Owner<'a> {
    island : &'a mut Island,
    dist : i32,
}

struct Island {
    cursor : Cursor,
    size : i32,
}
impl Island {
    fn manh_dist_to(& mut self, point : Cursor) -> i32 {
        (self.cursor.x - point.x).abs() + 
            (self.cursor.y - point.y).abs()
    }
}

impl PartialEq for Island {
    fn eq(&self, other : &Island) -> bool {
        return self.cursor == other.cursor;
    }
}
