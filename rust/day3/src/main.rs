use std::env;
use std::num;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = env::args().skip(1).next().unwrap().parse::<u64>().unwrap();
    println!("{}", spiral_dist(input));
    println!("id > input {}", find_id_whos_sum_is_greater_than(input));
}

/*

which ring are we in?
5 5 5 5 5
5 3 3 3 5
5 3 1 3 5
5 3 3 3 5
5 5 5 5 5


6  5  4  3  2
7           1
8           0
9           15
10 11 12 13 14

2  1  0  1  2
1           1
0           0
1           1
2  1  0  1  2

37  36  35  34  33  32  31
38  17  16  15  14  13  30
39  18   5   4   3  12  29
40  19   6   1   2  11  28
41  20   7   8   9  10  27
42  21  22  23  24  25  26
43  44  45  46  47  48  49


36  35  34  33  32  31  30
37  16  15  14  13  12  29
38  17   4   3   2  11  28
39  18   5   0   1  10  27
40  19   6   7   8  9   26
41  20  21  22  23  24  25
42  43  44  45  46  47  48

*/

fn find_id_whos_sum_is_greater_than(x: u64) -> u64 {
    let mut map = HashMap::new();
    #[derive(Debug)]
    struct State {
        coor: (i32, i32),
        index: u64,
        value: u64
    }
    let mut current = State { coor: (0, 0), index: 1, value: 1 };
    map.insert(current.coor.clone(), current.value.clone());
    #[derive(Debug)]
    enum Direction {
        Up,
        Left,
        Down,
        Right
    }
    let mut dir = Direction::Right;
    let mut leftBound = 0;
    let mut rightBound = 0;
    let mut topBound = 0;
    let mut bottomBound = 0;
    while current.value < x {
        // set current to the next one and update direction and bounds if necessary
        match dir {
            Direction::Right => {
                current.coor = (current.coor.0 + 1, current.coor.1);
                if current.coor.0 > rightBound {
                    rightBound += 1;
                    dir = Direction::Up;
                }
            }
            Direction::Up => {
                current.coor = (current.coor.0, current.coor.1 + 1);
                if current.coor.1 > topBound {
                    topBound += 1;
                    dir = Direction::Left;
                }
            }
            Direction::Left => {
                current.coor = (current.coor.0 - 1, current.coor.1);
                if current.coor.0 < leftBound {
                    leftBound -= 1;
                    dir = Direction::Down;
                }
            }
            Direction::Down => {
                current.coor = (current.coor.0, current.coor.1 - 1);
                if current.coor.1 < bottomBound {
                    bottomBound -= 1;
                    dir = Direction::Right;
                }
            }
        }
        let (xi, yi) = current.coor;
        let mut value = 0;
        for x in -1 .. 2 {
            for y in -1 .. 2 {
                value += map.get(&(xi + x, yi + y)).map(|x|*x).unwrap_or_default();
            }
        }
        current.value = value;
        current.index += 1;
        map.insert(current.coor.clone(), current.value);
    }
    current.value
}

fn spiral_dist(x: u64) -> u64 {
    match get_ring(x) {
        1 => 0,
        ring => {
            let prev_ring = ring - 2;
            // now we have the ring we are in
            // find where we are along the perimeter
            let perimeter_size = (ring * ring) - (prev_ring * prev_ring);
            let edge_size = (perimeter_size / 4);
            let x_offset = x - (prev_ring * prev_ring);
            let pos = (perimeter_size + x_offset - (edge_size / 2)) % perimeter_size;
            let dist_to_center_edge = side_dist(pos, edge_size);
            dist_to_center_edge + (ring / 2)
        }
    }
}

fn side_dist(position: u64, side_len: u64) -> u64 {
    let d = position % side_len;
    let half_len = side_len >> 1;
    if d > half_len {
        side_len - d
    } else {
        d
    }
}

fn get_ring(x: u64) -> u64 {
    let sqr = ((x - 1) as f64).sqrt() as u64;
    if (sqr & 1) != 1 {
        // even ring, add one
        sqr + 1
    } else {
        sqr + 2
    }
}

#[test]
fn side_test() {
    assert_eq!(side_dist(0, 4), 0);
    assert_eq!(side_dist(1, 4), 1);
    assert_eq!(side_dist(2, 4), 2);
    assert_eq!(side_dist(3, 4), 1);
    assert_eq!(side_dist(4, 4), 0);
    assert_eq!(side_dist(5, 4), 1);
    assert_eq!(side_dist(6, 4), 2);
    assert_eq!(side_dist(7, 4), 1);
    assert_eq!(side_dist(8, 4), 0);
    assert_eq!(side_dist(9, 4), 1);
    assert_eq!(side_dist(10, 4), 2);
    assert_eq!(side_dist(11, 4), 1);
    assert_eq!(side_dist(12, 4), 0);
    assert_eq!(side_dist(13, 4), 1);
    assert_eq!(side_dist(14, 4), 2);
    assert_eq!(side_dist(15, 4), 1);
}

#[test]
fn ring_test() {
    assert_eq!(get_ring(1), 1);
    assert_eq!(get_ring(2), 3);
    assert_eq!(get_ring(3), 3);
    assert_eq!(get_ring(4), 3);
    assert_eq!(get_ring(8), 3);
    assert_eq!(get_ring(9), 3);
    assert_eq!(get_ring(10), 5);
    assert_eq!(get_ring(25), 5);
    assert_eq!(get_ring(26), 7);
}

#[test]
fn dist_tests() {
    assert_eq!(spiral_dist(1), 0);
    assert_eq!(spiral_dist(12), 3);
    assert_eq!(spiral_dist(23), 2);
    assert_eq!(spiral_dist(14), 3);
    assert_eq!(spiral_dist(15), 2);
    assert_eq!(spiral_dist(17), 4);
    assert_eq!(spiral_dist(1024), 31);
    assert_eq!(spiral_dist(28), 3);
    assert_eq!(spiral_dist(27), 4);
    assert_eq!(spiral_dist(49), 6);
    assert_eq!(spiral_dist(48), 5);
}

