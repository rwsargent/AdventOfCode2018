use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use utils::*;

pub fn first_collision(input: StringInput) -> PuzzleResult {
  let (track, carts) = parse_track(input);
  let mut cart_positions = HashSet::new();
  for cart in carts.iter() {
    cart_positions.insert((cart.x, cart.y));
  }
  let mut carts = BinaryHeap::from(carts);

  loop {
    let mut next_carts = BinaryHeap::new();

    while let Some(mut cart) = carts.pop() {
      cart_positions.remove(&(cart.x, cart.y));
      let (x, y) = match cart.direction {
        Direction::North => (cart.x, cart.y - 1),
        Direction::West => (cart.x - 1, cart.y),
        Direction::South => (cart.x, cart.y + 1),
        Direction::East => (cart.x + 1, cart.y),
      };
      if !cart_positions.insert((x, y)) {
        return Ok(PuzzleSolution::Point(Point {
          x: x as i64,
          y: y as i64,
        }));
      }
      let direction = match track.get(x, y) as char {
        '|' | '-' => cart.direction,
        '\\' => match cart.direction {
          Direction::West => Direction::North,
          Direction::South => Direction::East,
          Direction::North => Direction::West,
          Direction::East => Direction::South,
        },
        '/' => match cart.direction {
          Direction::West => Direction::South,
          Direction::South => Direction::West,
          Direction::North => Direction::East,
          Direction::East => Direction::North,
        },
        '+' => {
          cart.turn_count += 1;
          match cart.turn_count % 3 {
            1 => cart.direction.left(),
            2 => cart.direction,
            0 => cart.direction.right(),
            _ => unreachable!(),
          }
        }
        x => {
          println!("unreachable!  ({})", x);
          unreachable!()
        }
      };
      cart.x = x;
      cart.y = y;
      cart.direction = direction;
      next_carts.push(cart);
    }
    carts = next_carts;
  }
}

pub fn last_cart(input: StringInput) -> PuzzleResult {
  let (track, carts) = parse_track(input);
  let mut carts = BinaryHeap::from(carts);

  loop {
    if carts.len() == 1 {
      let cart = carts.pop().unwrap();
      return Ok(PuzzleSolution::Point(Point {
        x: cart.x as i64,
        y: cart.y as i64,
      }));
    }
    let mut next_carts = BinaryHeap::new();

    while let Some(mut cart) = carts.pop() {
      let (x, y) = match cart.direction {
        Direction::North => (cart.x, cart.y - 1),
        Direction::West => (cart.x - 1, cart.y),
        Direction::South => (cart.x, cart.y + 1),
        Direction::East => (cart.x + 1, cart.y),
      };
      if carts
          .iter()
          .find(|c: &&Cart| c.x == x && c.y == y)
          .is_some()
          {
            carts = carts.into_iter().filter(|c| c.x != x || c.y != y).collect();
            continue;
          }
      if next_carts
          .iter()
          .find(|c: &&Cart| c.x == x && c.y == y)
          .is_some()
          {
            next_carts = next_carts
                .into_iter()
                .filter(|c| c.x != x || c.y != y)
                .collect();
            continue;
          }

      let direction = match track.get(x, y) as char {
        '|' | '-' => cart.direction,
        '\\' => match cart.direction {
          Direction::West => Direction::North,
          Direction::South => Direction::East,
          Direction::North => Direction::West,
          Direction::East => Direction::South,
        },
        '/' => match cart.direction {
          Direction::West => Direction::South,
          Direction::South => Direction::West,
          Direction::North => Direction::East,
          Direction::East => Direction::North,
        },
        '+' => {
          cart.turn_count += 1;
          match cart.turn_count % 3 {
            1 => cart.direction.left(),
            2 => cart.direction,
            0 => cart.direction.right(),
            _ => unreachable!(),
          }
        }
        x => {
          println!("unreachable!  ({})", x);
          unreachable!()
        }
      };
      cart.x = x;
      cart.y = y;
      cart.direction = direction;
      next_carts.push(cart);
    }
    carts = next_carts;
  }
}

fn parse_track(input: StringInput) -> (CartTrack, Vec<Cart>) {
  let mut data = vec![];
  let mut carts = vec![];
  let mut y = 0;
  for line in input.lines() {
    if line.len() > 0 {
      let mut line = line.as_bytes().to_vec();
      for (x, c) in line.iter_mut().enumerate() {
        if let Some((direction, replacement)) = match *c as char {
          '^' => Some((Direction::North, '|')),
          'v' => Some((Direction::South, '|')),
          '<' => Some((Direction::West, '-')),
          '>' => Some((Direction::East, '-')),
          _ => None,
        } {
          *c = replacement as u8;
          carts.push(Cart {
            direction,
            x,
            y,
            turn_count: 0,
          });
        }
      }
      data.push(line);
      y += 1;
    }
  }
  (CartTrack { data }, carts)
}

struct CartTrack {
  data: Vec<Vec<u8>>,
}

impl CartTrack {
  fn get(&self, x: usize, y: usize) -> u8 {
    if y < self.data.len() {
      unsafe {
        let vec = &self.data[y];
        if x < vec.len() {
          return vec[x];
        }
      }
    }
    return ' ' as u8;
  }
}

#[derive(Eq, PartialEq, Debug)]
struct Cart {
  x: usize,
  y: usize,
  direction: Direction,
  turn_count: usize,
}

impl PartialOrd for Cart {
  fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Cart {
  fn cmp(&self, other: &Self) -> Ordering {
    other.y.cmp(&self.y).then(other.x.cmp(&self.x))
  }
}
