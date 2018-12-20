use std::cmp::max;

use utils::*;

struct Battery {
  cells: [i64; 300 * 300],
}

impl Battery {
  fn new(input: i64) -> Battery {
    let mut battery = Battery {
      cells: [0; 300 * 300],
    };
    battery.set_power(input);
    battery
  }
  fn set_power(&mut self, input: i64) {
    for x in 1..=300 {
      for y in 1..=300 {
        let mut cell = self.get_mut(x, y).unwrap();
        let rack_id = x as i64 + 10;
        let power = rack_id * y as i64;
        let power = power + input;
        let power = power * rack_id;
        let power_string = power.to_string();
        let power = if power_string.len() < 3 {
          0
        } else {
          power_string[(power_string.len() - 3)..(power_string.len() - 2)]
              .parse()
              .unwrap()
        };
        let power = power - 5;
        *cell = power;
      }
    }
  }
  fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut i64> {
    if x >= 1 && x <= 300 && y >= 1 && y <= 300 {
      Some(&mut self.cells[(x - 1) + (y - 1) * 300])
    } else {
      None
    }
  }
  fn unsafe_get(&self, x: usize, y: usize) -> i64 {
    self.cells[(x - 1) + (y - 1) * 300]
  }
  fn get_power_of_square(&self, x: usize, y: usize, square_size: usize) -> Option<i64> {
    if x >= 1 && x <= (301 - square_size) && y >= 1 && y <= (301 - square_size) {
      let mut power: i64 = 0;
      for x in x..(x + square_size) {
        for y in y..(y + square_size) {
          power += self.unsafe_get(x, y);
        }
      }
      Some(power)
    } else {
      None
    }
  }
  fn unsafe_get_outline_power(&self, x: usize, y: usize, size: usize) -> i64 {
    let mut power = 0;
    for x in x..(x + size - 1) {
      power += self.unsafe_get(x, y + size - 1);
    }
    for y in y..(y + size - 1) {
      power += self.unsafe_get(x + size - 1, y);
    }
    power += self.unsafe_get(x + size - 1, y + size - 1);
    power
  }
  fn get_max_power_of_square(&self, x: usize, y: usize) -> Option<(i64, usize)> {
    if x >= 1 && x <= 300 && y >= 1 && y <= 300 {
      let mut max_power = 0;
      let mut max_square = 0;
      let mut power = 0;
      for square_size in 1..=(301 - max(x, y)) {
        power += self.unsafe_get_outline_power(x, y, square_size);

        if power > max_power {
          max_power = power;
          max_square = square_size;
        }
      }
      Some((max_power, max_square))
    } else {
      None
    }
  }
}

pub fn top_left_coor_of_most_powerful_cell(input: i64) -> PuzzleResult {
  let battery = Battery::new(input);

  let mut max_point = Point { x: 0, y: 0 };
  let mut max_power = 0;

  for x in 1..=298 {
    for y in 1..=298 {
      if let Some(power) = battery.get_power_of_square(x, y, 3) {
        if power > max_power {
          max_power = power;
          max_point = Point {
            x: x as i64,
            y: y as i64,
          };
        }
      }
    }
  }
  Ok(PuzzleSolution::Point(max_point))
}

pub fn most_powerful(input: i64) -> PuzzleResult {
  let battery = Battery::new(input);

  let mut max_point = Point { x: 0, y: 0 };
  let mut max_power = 0;
  let mut max_square = 0;

  for x in 1..=300 {
    for y in 1..=300 {
      if let Some((power, square_size)) = battery.get_max_power_of_square(x, y) {
        if power > max_power {
          max_power = power;
          max_point = Point {
            x: x as i64,
            y: y as i64,
          };
          max_square = square_size;
        }
      }
    }
  }

  Ok(PuzzleSolution::PointSize(max_point, max_square))
}
