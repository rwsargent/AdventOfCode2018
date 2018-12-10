use std::collections::HashSet;
use std::iter::FromIterator;

use utils::*;

fn get_coordinates(input: StringInput) -> Result<Vec<Point>> {
  let lines = input.lines();
  let mut result = Vec::with_capacity(lines.len());
  for line in lines {
    let split = line.split(", ").collect::<Vec<_>>();
    result.push(Point {
      x: split[0].parse()?,
      y: split[1].parse()?,
    });
  }
  Ok(result)
}

fn total_distance(p: &Point, coordinates: &Vec<Point>) -> i64 {
  coordinates.iter().map(|c| c.manhattan_dist(p)).sum()
}

pub fn total_distance_under(input: StringInput, max_dist: usize) -> PuzzleResult {
  let coordinates = get_coordinates(input)?;
  let average = {
    let x: i64 = coordinates.iter().map(|p| p.x).sum();
    let y: i64 = coordinates.iter().map(|p| p.y).sum();
    Point { x: x / coordinates.len() as i64, y: y / coordinates.len() as i64 }
  };

  let mut considered = HashSet::new();
  considered.insert(average.clone());
  let mut queue = vec![average];
  let mut num_safe = 0;
  while let Some(curr) = queue.pop() {
    let dist = total_distance(&curr, &coordinates) as usize;
//    println!("dist: {}", dist);
    if dist < max_dist {
      num_safe += 1;
      let neighboors: Vec<_> = vec![
        Point { x: curr.x, y: curr.y + 1 },
        Point { x: curr.x, y: curr.y - 1 },
        Point { x: curr.x - 1, y: curr.y },
        Point { x: curr.x - 1, y: curr.y + 1 },
        Point { x: curr.x - 1, y: curr.y - 1 },
        Point { x: curr.x + 1, y: curr.y + 1 },
        Point { x: curr.x + 1, y: curr.y + -1 },
        Point { x: curr.x + 1, y: curr.y },
      ]
          .into_iter()
          .filter(|p| !considered.contains(&p))
          .collect();
      for n in neighboors {
        considered.insert(n.clone());
        queue.push(n);
      }
    }
  }
  Ok(PuzzleSolution::usize(num_safe))
}

pub fn largest_finite(input: StringInput) -> PuzzleResult {
  let coordinates = get_coordinates(input)?;
  // find the bounds of all coordinates
  let mut min = coordinates[0].clone();
  let mut max = min.clone();
  for p in &coordinates {
    if min.x > p.x {
      min.x = p.x;
    }
    if max.x < p.x {
      max.x = p.x;
    }
    if min.y > p.y {
      min.y = p.y;
    }
    if max.y < p.y {
      max.y = p.y;
    }
  }

  // The infinite span coordinates are the ones that are closest to one of the boundary points.
  let mut finite_coordinates: HashSet<usize> = HashSet::from_iter(0..coordinates.len());
  for x in min.x..=max.x {
    for y in vec![min.y, max.y] {
      let p = Point { x, y };
      let min_coor = find_min_coordinate(&p, &coordinates);
      match min_coor {
        Some(coor) => {
          finite_coordinates.remove(&coor);
        }
        _ => {}
      }
    }
  }
  for x in vec![min.x, max.x] {
    for y in min.y..=max.y {
      let p = Point { x, y };
      let min_coor = find_min_coordinate(&p, &coordinates);
      match min_coor {
        Some(coor) => {
          finite_coordinates.remove(&coor);
        }
        _ => {}
      }
    }
  }
  let mut max_span = 0;
  for idx in finite_coordinates.into_iter() {
    let coor: Point = coordinates.get(idx).unwrap().clone();
    // find the size of the span
    let mut size = 0;
    let mut considered = HashSet::new();
    considered.insert(coor.clone());
    let mut queue = vec![coor];
    while let Some(curr) = queue.pop() {
      match find_min_coordinate(&curr, &coordinates) {
        Some(closest) if closest == idx => {
          size += 1;
          let neighboors: Vec<_> = vec![
            Point { x: curr.x, y: curr.y + 1 },
            Point { x: curr.x, y: curr.y - 1 },
            Point { x: curr.x - 1, y: curr.y },
            Point { x: curr.x - 1, y: curr.y + 1 },
            Point { x: curr.x - 1, y: curr.y - 1 },
            Point { x: curr.x + 1, y: curr.y + 1 },
            Point { x: curr.x + 1, y: curr.y + -1 },
            Point { x: curr.x + 1, y: curr.y },
          ]
              .into_iter()
              .filter(|p| !considered.contains(&p))
              .collect();
          for n in neighboors {
            considered.insert(n.clone());
            queue.push(n);
          }
        }
        _ => {}
      }
    }
    if size > max_span {
      max_span = size;
    }
  }
  Ok(PuzzleSolution::usize(max_span))
}

fn find_min_coordinate(point: &Point, coordinates: &Vec<Point>) -> Option<usize> {
  let mut result = None;
  let mut is_tie = false;
  let mut min_dist = 0;
  for (i, coordinate) in coordinates.iter().enumerate() {
    let dist = point.manhattan_dist(coordinate);
    match result {
      Some(_) => {
        if dist < min_dist {
          is_tie = false;
          result = Some(i);
          min_dist = dist;
        } else if dist == min_dist {
          is_tie = true;
        }
      }
      None => {
        min_dist = dist;
        result = Some(i);
      }
    }
  }
  if is_tie {
    None
  } else {
    result
  }
}


