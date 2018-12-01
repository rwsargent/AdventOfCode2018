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
    let input = env::args().nth(1).unwrap();
    let iterations = env::args().nth(2).map(|x| x.parse::<usize>().unwrap()).unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let rules = parse_rules(&contents);
    println!("numRules: {}", rules.len());
//    for (x, y) in &rules {
//        print_img_flat(x);
//        print_img_flat(&y);
//        println!("!")
//    }

    let mut image = ".#...####".to_string().into_bytes();

    print_img(&image);
    let mut total_used_rules = HashSet::new();
    for i in 0..iterations {
        let (used_rules, new_image) = iterate(&image, &rules);
        image = new_image;
        for r in used_rules {
            total_used_rules.insert(r.clone());
        }
    }

    let num_ones: usize = image.iter().map(|x| {
        if *x == '#' as u8 {
            1
        } else {
            0
        }
    }).sum();
    println!("num ones: {}", num_ones);
}

fn print_img(image: &Vec<u8>) {
    let size = (image.len() as f64).sqrt() as usize;
    for i in 0..size {
        let start = (i * size);
        println!("{}", String::from_utf8_lossy(&image[start..(start + size)]));
    }
}

fn print_img_flat(image: &Vec<u8>) {
    println!("{}", String::from_utf8(image.clone()).unwrap());
}

fn iterate(image: &Vec<u8>, rules: &HashMap<CanonicalSquare, Vec<u8>>) -> (HashSet<CanonicalSquare>, Vec<u8>) {
    let mut used_rules = HashSet::new();
    let size = (image.len() as f64).sqrt() as usize;
    let block_size = if (size & 1) == 0 {
        2
    } else {
        3
    };
    let block_size_sqrd = block_size * block_size;

    let num_blocks = size / block_size;

    let new_block_size = block_size + 1;
    let new_block_size_sqrd = new_block_size * new_block_size;
    let new_size = num_blocks * new_block_size;

    let mut result = {
        let s = new_size * new_size;
        let mut r = Vec::with_capacity(s);
        unsafe {
            r.set_len(s);
        }
        r
    };


    for y in 0..num_blocks {
        for x in 0..num_blocks {
            let mut block = Vec::with_capacity(block_size_sqrd);
            unsafe {
                block.set_len(block_size_sqrd);
            }
            for i in 0..block_size_sqrd {
                block[i] = image[((y * block_size + (i / block_size)) * size) + (x * block_size) + i % block_size];
            }
            let canonical_square = CanonicalSquare::new(&block);
            used_rules.insert(canonical_square.clone());
            let new_block = rules.get(&canonical_square).expect(format!("can't find canonical square {:?}", &canonical_square).as_str());
            for i in 0..new_block_size_sqrd {
                result[(x * new_block_size + (i % new_block_size)) + (((y * new_block_size) + (i / new_block_size)) * new_size)] = new_block[i];
            }
        }
    }

    (used_rules, result)
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct CanonicalSquare {
    data: Vec<u8>
}

impl std::fmt::Debug for CanonicalSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&String::from_utf8(self.data.clone()).unwrap(), f)
    }
}

// 0 1 2
// 3 4 5
// 6 7 8

static THREE_BY_THREE_PERMUTATIONS: [usize; 72] = [
    4, 5, 2, 1, 0, 3, 6, 7, 8,
    4, 5, 8, 7, 6, 3, 0, 1, 2,
    4, 1, 2, 5, 8, 7, 6, 3, 0,
    4, 1, 0, 3, 6, 7, 8, 5, 2,
    4, 3, 0, 1, 2, 5, 8, 7, 6,
    4, 3, 6, 7, 8, 5, 2, 1, 0,
    4, 7, 8, 5, 2, 1, 0, 3, 6,
    4, 7, 6, 3, 0, 1, 2, 5, 8, ];
static TWO_BY_TWO_PERMUTATIONS: [usize; 32] = [
    0, 1, 3, 2,
    1, 3, 2, 0,
    3, 2, 0, 1,
    2, 0, 1, 3,
    0, 2, 3, 1,
    1, 0, 2, 3,
    3, 1, 0, 2,
    2, 3, 1, 0,
];

impl CanonicalSquare {
    fn rotate(tile: &Vec<u8>, new_start: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(tile.len());
        unsafe {
            result.set_len(tile.len());
        }
        for i in 0..tile.len() {
            result[i] = tile[(new_start + i) % tile.len()]
        }
        result
    }
    pub fn new(tile: &Vec<u8>) -> CanonicalSquare {
        match tile.len() {
            4 => {
                let mut max_p = CanonicalSquare::make_permutation(&tile, &TWO_BY_TWO_PERMUTATIONS[0..4]);
                let mut max = CanonicalSquare::score(&max_p);
                for x in 1..8 {
                    let i = x * 4;
                    let p = CanonicalSquare::make_permutation(&tile, &TWO_BY_TWO_PERMUTATIONS[i..(i + 4)]);
                    let score = CanonicalSquare::score(&p);
                    if score > max {
                        max = score;
                        max_p = p;
                    }
                }
                CanonicalSquare { data: max_p }
            }
            9 => {
                let mut max_p = CanonicalSquare::make_permutation(&tile, &THREE_BY_THREE_PERMUTATIONS[0..9]);
                let mut max = CanonicalSquare::score(&max_p);
                for x in 1..8 {
                    let i = x * 9;
                    let p = CanonicalSquare::make_permutation(&tile, &THREE_BY_THREE_PERMUTATIONS[i..(i + 9)]);
                    let score = CanonicalSquare::score(&p);
                    if score > max {
                        max = score;
                        max_p = p;
                    }
                }
                CanonicalSquare { data: max_p }
            }
            _ => unimplemented!()
        }
    }
    fn make_permutation(tile: &Vec<u8>, permutation: &[usize]) -> Vec<u8> {
        let mut result = Vec::with_capacity(tile.len());
        unsafe {
            result.set_len(tile.len());
        }
        for i in 0..permutation.len() {
            result[i] = tile[permutation[i]];
        }
        result
    }
    fn score(tile: &Vec<u8>) -> usize {
        let mut current = '#' as u8;
        let mut count = 1;
        let mut score = 0;
        let mut i = 1;
        while i < tile.len() {
            if current == tile[i] {
                count += 1;
            } else {
                score += count * count * i;
                count = 1;
                current = tile[i];
            }
            i += 1;
        }
        score + (count * count * i)
    }
}

fn parse_rules(input: &String) -> HashMap<CanonicalSquare, Vec<u8>> {
    input.split("\n").map(|x| {
        let tiles = x.split("=>").map(|x| {
            x.trim().replace("/", "").into_bytes()
        }).collect::<Vec<_>>();
        let c = CanonicalSquare::new(&tiles[0]);
        let v = tiles[1].clone();
        (c, v)
    }).collect()
}

fn parse_rules2(input: &String) -> HashMap<Vec<u8>, Vec<u8>> {
    input.split("\n").flat_map(|x| {
        let tiles = x.split("=>").map(|x| {
            x.trim().replace("/", "").into_bytes()//.filter(|x| x != "/").collect::<String>()
        }).collect::<Vec<_>>();
        if tiles.len() == 4 {}
        let c = CanonicalSquare::new(&tiles[0]);
        let v = tiles[1].clone();
//        println!("{:?} -> {:?}", &c, String::from_utf8(v.clone()).unwrap());
        let variations = variations(&tiles[0]);
        variations.into_iter().map(|v2| {
            (v2, v.clone())
        }).collect::<Vec<_>>().into_iter()
    }).collect()
}

fn variations2(img: &Vec<u8>) -> HashSet<Vec<u8>> {

    let mut result = HashSet::new();

    for rotate in 0..4 {
        let mut v = img.clone();
        let mut i = 0;
        while i < rotate {
            v = rotate_left(&v);
            i += 1;
        }
        result.insert(flip_horiz(&v));
        result.insert(flip_virt(&v));
        result.insert(flip_virt(&flip_horiz(&v)));
        result.insert(v);
    }
    result
}

fn variations(img: &Vec<u8>) -> HashSet<Vec<u8>> {

    let size = if img.len() == 4 { 2 } else { 3 };

    let transforms: &[&Fn(usize, usize) -> (usize, usize)] = &[
        &|i, j| (i, j),
        &|i, j| (j, i),
        &|i, j| (size - j - 1, i),
        &|i, j| (j, size - i - 1),
        &|i, j| (size - i - 1, j),
        &|i, j| (i, size - j - 1),
        &|i, j| (size - i - 1, size - j - 1),
        &|i, j| (size - j - 1, size - i - 1),
    ];

    let mut result = HashSet::new();

    for trans in transforms {
        let mut v = img.clone();
        for j in 0..size {
            for i in 0..size {
                let (i2, j2) = trans(i, j);
                v[j * size + i] = img[j2 * size + i2];
            }
        }
        result.insert(v);
    }

    result
}

fn rotate_left(img: &Vec<u8>) -> Vec<u8> {
    let mut result = img.clone();
    result[0] = img[6];
    result[1] = img[3];
    result[2] = img[0];
    result[3] = img[7];
    result[5] = img[1];
    result[6] = img[8];
    result[7] = img[5];
    result[8] = img[2];
    result
}

fn flip_horiz(img: &Vec<u8>) -> Vec<u8> {
    let mut result = img.clone();
    result[0] = img[2];
    result[3] = img[5];
    result[6] = img[8];
    result[2] = img[0];
    result[5] = img[3];
    result[8] = img[6];
    result
}

fn flip_virt(img: &Vec<u8>) -> Vec<u8> {
    let mut result = img.clone();
    result[0] = img[6];
    result[1] = img[7];
    result[2] = img[8];
    result[6] = img[0];
    result[7] = img[1];
    result[8] = img[2];
    result
}


// 0 1 2
// 3 4 5
// 6 7 8
#[test]
fn canonical_test() {
//    // .#.
//    // ..#
//    // ###
//    let c1 = CanonicalSquare::new(&".#...####".to_string().into_bytes());
//    // #..
//    // #.#
//    // ##.
//    let c2 = CanonicalSquare::new(&"#..#.###.".to_string().into_bytes());
//    assert_eq!(c1, c2);
//    // ###
//    // #..
//    // .#.
//    let c2 = CanonicalSquare::new(&"####...#.".to_string().into_bytes());
//    assert_eq!(c1, c2);
//    // .##
//    // #.#
//    // ..#
//    let c2 = CanonicalSquare::new(&".###.#..#".to_string().into_bytes());
//    assert_eq!(c1, c2);
//    // ##.
//    // #.#
//    // #..
//    let c2 = CanonicalSquare::new(&"##.#.##..".to_string().into_bytes());
//    assert_eq!(c1, c2);
//    // #..
//    // #.#
//    // ##.
//    let c2 = CanonicalSquare::new(&"#..#.###.".to_string().into_bytes());
//    assert_eq!(c1, c2);


//    let mut i = 0;
    for i in 0..(1 << 9) {
        let mut img: Vec<u8> = vec![];//Vec::with_capacity(9);
        if (i & (1 << 8)) > 0 {
            img.push('#' as u8);
        } else {
            img.push('.' as u8);
        }
        if (i & (1 << 7)) > 0 {
            img.push('#' as u8);
        } else {
            img.push('.' as u8);
        }
        if (i & (1 << 6)) > 0 {
            img.push('#' as u8);
        } else {
            img.push('.' as u8);
        }
        if (i & (1 << 5)) > 0 {
            img.push('#' as u8);
        } else {
            img.push('.' as u8);
        }
        if (i & (1 << 4)) > 0 {
            img.push('#' as u8);
        } else {
            img.push('.' as u8);
        }
        if (i & (1 << 3)) > 0 {
            img.push('#' as u8);
        } else {
            img.push('.' as u8);
        }
        if (i & (1 << 2)) > 0 {
            img.push('#' as u8);
        } else {
            img.push('.' as u8);
        }
        if (i & (1 << 1)) > 0 {
            img.push('#' as u8);
        } else {
            img.push('.' as u8);
        }
        if (i & 1) > 0 {
            img.push('#' as u8);
        } else {
            img.push('.' as u8);
        }
//        let img = "........#".to_string().into_bytes();
        let cimg = CanonicalSquare::new(&img);
        println!("{} {:8b} {:?} {:?}", i, i, String::from_utf8(img.clone()).unwrap(), cimg);

        let set1 = variations(&img);
        let set2 = variations2(&img);
        println!("{:?}", set1);
        println!("{:?}", set2);
        assert_eq!(set1, set2);

        for v in set1 {
            assert_eq!(CanonicalSquare::new(&v), cimg);
        }
    }
}
