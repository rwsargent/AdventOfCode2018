use regex::Regex;

use std::io::{Read, stdin, stdout, Write};
use utils::*;

fn get_stars(input: StringInput) -> Result<Vec<Star>> {
  let re = Regex::new(r"position=< ?(-?\d+),\s*(-?\d+)> velocity=< ?(-?\d+),\s*(-?\d+)>.*")?;
  let mut result = vec![];
  for cap in re.captures_iter(&input.content) {
    result.push(Star {
      pos: Point {
        x: cap[1].parse()?,
        y: cap[2].parse()?,
      },
      vel: Point {
        x: cap[3].parse()?,
        y: cap[4].parse()?,
      },
    })
  }
  Ok(result)
}

#[derive(Debug)]
struct Star {
  pos: Point,
  vel: Point,
}

fn bounds(stars: &Vec<Star>) -> Option<(Point, Point)> {
  if stars.len() == 0 {
    None
  } else {
    let mut min = stars[0].pos.clone();
    let mut max = min.clone();
    for star in stars {
      if star.pos.x < min.x {
        min.x = star.pos.x;
      }
      if star.pos.x > max.x {
        max.x = star.pos.x;
      }

      if star.pos.y < min.y {
        min.y = star.pos.y;
      }
      if star.pos.y > max.y {
        max.y = star.pos.y;
      }
    }
    Some((min, max))
  }
}

fn get_area(stars: &Vec<Star>) -> usize {
  if let Some((min, max)) = bounds(stars) {
    ((max.x - min.x) * (max.y - min.y)) as usize
  } else {
    0
  }
}

fn next(stars: &mut Vec<Star>) {
  for mut star in stars {
    star.pos = &star.pos + &star.vel;
  }
}

fn prev(stars: &mut Vec<Star>) {
  for mut star in stars {
    star.pos = &star.pos - &star.vel;
  }
}

fn print_stars(stars: &Vec<Star>) {
  if let Some((min, max)) = bounds(stars) {
    println!("bounds: {:?}", (min, max));
    let width = (max.x - min.x + 3) as usize;
    let height = (max.y - min.y + 3) as usize;
    let area = width * height;
    let mut output = Vec::with_capacity((width + 2) * height);
    for h in 0..height {
      for w in 0..width {
        output.push('.' as u8);
      }
      output.push('\n' as u8);
      output.push('\r' as u8);
    }
    for star in stars {
      let x = (star.pos.x - min.x) as usize;
      let y = (star.pos.y - min.y) as usize;
      println!("({}, {})", x, y);
      let idx = x + 1 + ((y + 1 as usize) * (width + 2));
      if let Some(elem) = output.get_mut(idx) {
        *elem = '#' as u8;
      }
    }
    stdout().write(&output);
  }
}

pub fn main(input: StringInput) -> PuzzleResult {
  let mut stars = get_stars(input)?;
  if stars.len() == 0 {
    println!("Empty stars");
    return Ok(PuzzleSolution::unit);
  }

  // find smallest point
  let mut area = get_area(&stars);
  let mut last_area = area;

  let mut second = 0;
  while area <= last_area {
    last_area = area;
    next(&mut stars);
    second += 1;
    area = get_area(&stars);
  }

  prev(&mut stars);
  second -= 1;
  print_stars(&stars);
  println!("second: {}", second);

  let mut buf = [0];
  while let Ok(size) = stdin().read(&mut buf) {
    if size > 0 {
      match buf[0] {
        b'q' => {
          break;
        }
        b'n' => {
          second += 1;
          next(&mut stars);
          print_stars(&stars);
          println!("second: {}", second);
        }
        b'p' => {
          second -= 1;
          prev(&mut stars);
          print_stars(&stars);
          println!("second: {}", second);
        }
        _ => {}
      }
    }
  }

  Ok(PuzzleSolution::unit)
}
