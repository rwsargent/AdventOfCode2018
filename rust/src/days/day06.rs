use util::cursor::{Cursor, Bounds};
use std::collections::{HashMap, VecDeque};
use std::cmp::PartialEq;

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

fn fill_spheres_of_influence<'m, 'a>(island : &'a Island, bounds : Bounds, map : &'m mut HashMap<Cursor, Owner>) {
    let queue = VecDeque::new();
    queue.push_back(island.cursor.up()); 
    queue.push_back(island.cursor.left()); 
    queue.push_back(island.cursor.right()); 
    queue.push_back(island.cursor.down());
    while queue.len() != 0 {
        let curr = queue.pop_front().unwrap();
        if !curr.in_bounds(bounds) {
            continue;
        }
        let dist = island.manh_dist_to(curr);
        let owner = map.entry(curr)
            .or_insert(Owner{island : island, dist : dist});
        if owner.island == island || owner.dist >= dist  {
            continue;
        }
        if owner.dist > dist {
            owner.dist = dist;
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
    island : &'a Island,
    dist : i32,
}

struct Island {
    id : i32,
    cursor : Cursor,
    size : i32,
}
impl Island {
    fn manh_dist_to(&self, point : Cursor) -> i32 {
        (self.cursor.x - point.x).abs() + 
            (self.cursor.y - point.y).abs()
    }
}

impl PartialEq for Island {

    fn eq(&self, other : &Island) -> bool {
        return self.id == other.id;
    }
}
