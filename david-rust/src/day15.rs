use std::{thread, time};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::io::{Read, stdin, stdout, Write};

use utils::*;

pub fn do_battle(input: StringInput) -> PuzzleResult {
  let mut board = parse_board(input)?;
  board.print();
  let mut buf = [0];
  while let Ok(_) = stdin().read_exact(&mut buf) {
    match buf[0] {
      b'q' => {
        return Ok(PuzzleSolution::unit);
      }
      b'f' => break,
      b'\n' => {
        board.step();
        board.print();
      }
      _ => {}
    }
  }
  let mut game_status = GameStatus::Continue;
  while game_status != GameStatus::Over {
    game_status = board.step();
    board.print();
    thread::sleep(time::Duration::from_millis(2));
  }

  let board_turn: usize = board.units.values().map(|u| u.turns).min().unwrap();
  println!("turn: {}", board_turn);

  let mut leftover_hitpoints: usize = board.units.values().map(|u| u.hit_points).sum();
  Ok(PuzzleSolution::Triplet(
    board_turn,
    leftover_hitpoints,
    board_turn * leftover_hitpoints,
  ))
}

pub fn battle_outcome(input: StringInput) -> PuzzleResult {
  let mut board = parse_board(input)?;
  board.print();
  //  let mut buf = [0];
  //  while let Ok(_) = stdin().read_exact(&mut buf) {
  //    match buf[0] {
  //      b'q' => {
  //        break;
  //      }
  //      _ => {
  //        board.step();
  //        board.print();
  //      }
  //    }
  //  }
  let mut game_status = GameStatus::Continue;
  while game_status != GameStatus::Over {
    game_status = board.step();
    board.print();
    thread::sleep(time::Duration::from_millis(2));
  }

  let board_turn: usize = board.units.values().map(|u| u.turns).min().unwrap();
  println!("turn: {}", board_turn);

  let mut leftover_hitpoints: usize = board.units.values().map(|u| u.hit_points).sum();
  Ok(PuzzleSolution::Triplet(
    board_turn,
    leftover_hitpoints,
    board_turn * leftover_hitpoints,
  ))
}

fn parse_board(input: StringInput) -> Result<Board> {
  let lines = input.lines();
  let mut board = Vec::with_capacity(lines.len());
  let mut units = HashMap::new();

  let mut y = 0;
  for line in lines {
    let mut x = 0;
    let mut row = Vec::with_capacity(line.len());
    for c in line.chars() {
      match c {
        '#' => {
          row.push(Square::Wall);
        }
        '.' => {
          row.push(Square::Open);
        }
        'E' => {
          units.insert(
            Position { x, y },
            Unit {
              tpe: UnitType::Elf,
              turns: 0,
              hit_points: 200,
              attack_power: 3,
              position: Position { x, y },
            },
          );
          row.push(Square::Elf);
        }
        'G' => {
          units.insert(
            Position { x, y },
            Unit {
              tpe: UnitType::Goblin,
              turns: 0,
              hit_points: 200,
              attack_power: 3,
              position: Position { x, y },
            },
          );
          row.push(Square::Goblin);
        }
        _ => {
          return Err(Box::new(InvalidLine(line.to_string())));
        }
      }
      x += 1;
    }
    board.push(row);
    y += 1;
  }

  Ok(Board {
    board,
    units,
  })
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Square {
  Wall,
  Open,
  Elf,
  Goblin,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
  x: usize,
  y: usize,
}

impl Position {
  fn new(x: usize, y: usize) -> Position {
    Position { x, y }
  }
}

impl fmt::Debug for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl PartialOrd for Position {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Position {
  fn cmp(&self, other: &Self) -> Ordering {
    other.y.cmp(&self.y).then(other.x.cmp(&self.x))
  }
}

#[derive(PartialEq, Eq, Debug)]
struct Unit {
  tpe: UnitType,
  turns: usize,
  hit_points: usize,
  attack_power: usize,
  position: Position,
}

impl PartialOrd for Unit {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Unit {
  fn cmp(&self, other: &Self) -> Ordering {
    other
        .turns
        .cmp(&self.turns)
        .then(self.position.cmp(&other.position))
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum UnitType {
  Goblin,
  Elf,
}

impl UnitType {
  fn square(&self) -> Square {
    match self {
      UnitType::Elf => Square::Elf,
      UnitType::Goblin => Square::Goblin,
    }
  }
  fn other(&self) -> UnitType {
    match self {
      UnitType::Elf => UnitType::Goblin,
      UnitType::Goblin => UnitType::Elf,
    }
  }
}

struct Board {
  board: Vec<Vec<Square>>,
  units: HashMap<Position, Unit>,
}

impl Board {
  fn print(&self) {
    let mut output = Vec::with_capacity(self.board.iter().map(|v| v.len() + 1).sum());
    let mut y = 0;
    for v in &self.board {
      let mut x = 0;
      for s in v {
        match s {
          Square::Wall => output.push('#' as u8),
          Square::Open => output.push('.' as u8),
          Square::Elf => output.push('E' as u8),
          Square::Goblin => output.push('G' as u8),
        }
        x += 1;
      }
      output.push('\n' as u8);
      y += 1;
    }
    stdout().write(&output);
  }

  fn get(&self, x: usize, y: usize) -> Square {
    if y < self.board.len() {
      let v = &self.board[y];
      if x < v.len() {
        return v[x];
      }
    }
    Square::Wall
  }

  fn set(&mut self, x: usize, y: usize, square: Square) {
    if y < self.board.len() {
      let mut v = &mut self.board[y];
      if x < v.len() {
        v[x] = square;
      }
    }
  }

  fn step(&mut self) -> GameStatus {
    let mut opt_current = {
      let k = self
          .units
          .iter()
          .max_by(|(k, v), (k1, v1)| v1.turns.cmp(&v.turns).then(k.cmp(k1)))
          .map(|(k, v)| k.clone());
      k.and_then(|k| self.units.remove(&k))
    };
    if let Some(mut current_unit) = opt_current {
      let enemy_tpe = current_unit.tpe.other().square();
      if self.units
          .values()
          .filter(|u| u.tpe != current_unit.tpe)
          .count() == 0 {
        self.units.insert(current_unit.position, current_unit);
        return GameStatus::Over;
      }
      if !self
          .surrounding_squares(current_unit.position)
          .into_iter()
          .any(|p| self.get(p.x, p.y) == enemy_tpe) {
        let goals = self
            .units
            .iter()
            .flat_map(|(_, unit)| {
              if unit.tpe != current_unit.tpe {
                self.open_surrounding_squares(unit.position)
              } else {
                vec![]
              }
            })
            .collect();
        //        println!("goals: {:?}", goals);
        if let Some(path) = self.find_shortest_path_to(current_unit.position, goals) {
          //          println!("path: {:?}", path);
          if path.len() > 0 {
            let next = path[0];
            self.set(
              current_unit.position.x,
              current_unit.position.y,
              Square::Open,
            );
            self.set(next.x, next.y, current_unit.tpe.square());
            current_unit.position = next;
          }
        }
      }
      // after moving, if we can attack, do it.
      let targets: Vec<_> = self
          .surrounding_squares(current_unit.position)
          .into_iter()
          .filter(|p| self.get(p.x, p.y) == enemy_tpe)
          .collect();
      if targets.len() > 0 {
        let mut min_health = usize::max_value();
        let mut pos = Position { x: 0, y: 0 };
        for tp in targets {
          if let Some(target) = self.units.get(&tp) {
            if target.hit_points < min_health {
              min_health = target.hit_points;
              pos = tp;
            }
          }
        }
        if min_health <= current_unit.attack_power {
          // target unit is dead
          let target = self.units.remove(&pos).unwrap();
          self.set(target.position.x, target.position.y, Square::Open);
        } else {
          let mut target = self.units.get_mut(&pos).unwrap();
          target.hit_points -= current_unit.attack_power;
        }
      }
      current_unit.turns += 1;
      self.units.insert(current_unit.position, current_unit);
    }

    GameStatus::Continue
  }

  fn find_shortest_path_to(
    &self,
    start: Position,
    goals: HashSet<Position>,
  ) -> Option<Vec<Position>> {
    let mut queue = BinaryHeap::new();
    let mut considered = HashSet::new();
    queue.push((0, start, vec![]));
    considered.insert(start);

    while let Some((dist, next, path)) = queue.pop() {
      if goals.contains(&next) {
        return Some(path);
      }
      for next_move in self.open_surrounding_squares(next) {
        if considered.insert(next_move) {
          let mut path = path.clone();
          path.push(next_move);
          queue.push((dist - 1, next_move, path));
        }
      }
    }

    None
  }
  fn surrounding_squares(&self, start: Position) -> Vec<Position> {
    let mut result = vec![
      Position::new(start.x + 1, start.y),
      Position::new(start.x, start.y + 1),
    ];
    let xgz = start.x > 0;
    let ygz = start.y > 0;
    if xgz {
      result.push(Position::new(start.x - 1, start.y));
    }
    if ygz {
      result.push(Position::new(start.x, start.y - 1));
    }
    result
  }

  fn open_surrounding_squares(&self, start: Position) -> Vec<Position> {
    self.surrounding_squares(start)
        .into_iter()
        .filter(|p| self.get(p.x, p.y) == Square::Open)
        .collect()
  }
}

#[derive(Debug, Eq, PartialEq)]
enum GameStatus {
  Continue,
  Over,
}
