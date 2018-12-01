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


    println!("hash: {:?}", hash_string(contents.into_bytes()));
}

fn hash_string(lengths: Vec<u8>) -> String {
    let sparse_hash = hash(lengths);

    let mut dense_hash = Vec::with_capacity(16);
    for x in 0..16 {
        let mut r = 0;
        for i in (x * 16)..((x + 1) * 16) {
            r ^= sparse_hash[i];
        }
        dense_hash.push(r as u8);
    }

    format!("{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            dense_hash[0], dense_hash[1] as usize, dense_hash[2] as usize, dense_hash[3],
            dense_hash[4], dense_hash[5], dense_hash[6], dense_hash[7],
            dense_hash[8], dense_hash[9], dense_hash[10], dense_hash[11],
            dense_hash[12], dense_hash[13], dense_hash[14], dense_hash[15])
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

#[test]
fn test_hash(){
    assert_eq!(hash_string("".as_bytes().to_vec()), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(hash_string("AoC 2017".as_bytes().to_vec()), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(hash_string("1,2,3".as_bytes().to_vec()), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(hash_string("1,2,4".as_bytes().to_vec()), "63960835bcdc130f0b66d7ff4f6a5a8e");
}
