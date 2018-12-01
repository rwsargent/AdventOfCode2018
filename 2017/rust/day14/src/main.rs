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

    let mut rows = Vec::new();

    let mut num_ones: u32 = 0;
    for x in 0..128 {
        let string = format!("{}-{}", input, x);
        let row = hash_string(string.as_bytes().to_vec());
        num_ones += row.iter().map(|x| (*x).count_ones()).sum();
        row.iter().for_each(|b| print!("{:08b}", b));
        println!();
        rows.push(row);
    }
    println!("num ones {}", num_ones);

    let mut groups = 0;

    struct Grid {
        rows: Vec<Vec<u8>>
    }

    let mut grid = Grid { rows };
    impl Grid {
        fn is_used(&self, x: usize, y: usize) -> bool {
            (self.rows[y][x >> 3] & (1 << (7 - (x & 7)))) > 0
        }
    }

    for y in 0..128 {
        for x in 0..128 {
            if grid.is_used(x, y) {
                groups += 1;

                let mut to_consider = vec![(x, y)];
                let mut group_size = 0;
                while let Some((x, y)) = to_consider.pop() {
                    group_size += 1;
                    {
                        let p = &mut grid.rows[y][x >> 3];
                        let little_x = 7 - (x & 7);
                        *p = (*p) & !(1 << little_x);
                    }
                    let mut neighbors = vec![];
                    if y > 0 {
                        neighbors.push((x, y - 1));
                    }
                    if y < 127 {
                        neighbors.push((x, y + 1));
                    }
                    if x > 0 {
                        neighbors.push((x - 1, y));
                    }
                    if x < 127 {
                        neighbors.push((x + 1, y));
                    }
                    for (x, y) in neighbors.into_iter() {
                        if grid.is_used(x, y) {
                            to_consider.push((x, y));
                        }
                    }
                }
            }
        }
    }
    println!("num groups {}", groups);
}

fn hash_string(lengths: Vec<u8>) -> Vec<u8> {
    let sparse_hash = hash(lengths);

    let mut dense_hash = Vec::with_capacity(16);
    for x in 0..16 {
        let mut r = 0;
        for i in (x * 16)..((x + 1) * 16) {
            r ^= sparse_hash[i];
        }
        dense_hash.push(r as u8);
    }
    dense_hash
}

fn hash(lengths: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::with_capacity(256);
    for i in 0..256 {
        result.push(i as u8);
    }
    let mut skip: u8 = 0;
    let mut current_position: u8 = 0;
    for _ in 0..64 {
        let mut l = lengths.clone();
        l.append(&mut vec![17, 31, 73, 47, 23]);
        l.into_iter().for_each(|length| {
            for i in 0..(length >> 1) {
                let l = ((current_position.wrapping_add(i))) as usize;
                let r = ((current_position.wrapping_add(length - 1 - i))) as usize;
                let temp = result[l];
                result[l] = result[r];
                result[r] = temp;
            }
            current_position = (current_position.wrapping_add(length).wrapping_add(skip));
            skip = skip.wrapping_add(1);
        });
    }
    result
}