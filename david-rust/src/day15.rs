use std::{thread, time};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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
    thread::sleep(time::Duration::from_millis(1));
  }

  let mut leftover_hitpoints: usize = board.units.values().map(|u| u.hit_points).sum();
  Ok(PuzzleSolution::Triplet(
    board.completed_turns,
    leftover_hitpoints,
    board.completed_turns * leftover_hitpoints,
  ))
}

pub fn battle_outcome(input: StringInput) -> PuzzleResult {
  let mut board = parse_board(input)?;
  board.print();

  let mut game_status = GameStatus::Continue;
  while game_status != GameStatus::Over {
    game_status = board.step();
  }

  let mut leftover_hitpoints: usize = board.units.values().map(|u| u.hit_points).sum();
  Ok(PuzzleSolution::Triplet(
    board.completed_turns,
    leftover_hitpoints,
    board.completed_turns * leftover_hitpoints,
  ))
}

fn parse_board(input: StringInput) -> Result<Board> {
  let lines = input.lines();
  let width = lines.iter().map(|l| l.len()).max().ok_or(Box::new(InvalidInput))?;
  let mut map = Vec::with_capacity(lines.len() * width);
  let mut units = HashMap::new();

  let mut y = 0;
  for line in lines {
    let mut x = 0;
    for c in line.chars() {
      match c {
        '#' => {
          map.push(Square::Wall);
        }
        '.' => {
          map.push(Square::Open);
        }
        'E' => {
          let id = UnitId(units.len());
          units.insert(
            id,
            Unit {
              tpe: UnitType::Elf,
              turns: 0,
              hit_points: 200,
              attack_power: 3,
              position: Position { x, y },
            },
          );
          map.push(Square::Elf(id));
        }
        'G' => {
          let id = UnitId(units.len());
          units.insert(
            id,
            Unit {
              tpe: UnitType::Goblin,
              turns: 0,
              hit_points: 200,
              attack_power: 3,
              position: Position { x, y },
            },
          );
          map.push(Square::Goblin(id));
        }
        _ => {
          return Err(Box::new(InvalidLine(line.to_string())));
        }
      }
      x += 1;
    }
    // not all rows are guaranteed to be the same length
    while x < width {
      x += 1;
      map.push(Square::Wall);
    }
    y += 1;
  }

  Ok(Board::new(map, width, units))
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Square {
  Wall,
  Open,
  Elf(UnitId),
  Goblin(UnitId),
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
        .then_with(|| other.position.y.cmp(&self.position.y))
        .then_with(|| other.position.x.cmp(&self.position.x))
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum UnitType {
  Goblin,
  Elf,
}

impl UnitType {
  fn other(&self) -> UnitType {
    match self {
      UnitType::Elf => UnitType::Goblin,
      UnitType::Goblin => UnitType::Elf,
    }
  }
}

#[derive(Hash, PartialEq, Eq, Debug, Copy)]
struct UnitId(usize);

struct Board {
  map: Vec<Square>,
  width: usize,
  completed_rounds: usize,
  units: HashMap<UnitId, Unit>,
  current_round: Round,
}

struct Round {
  unit_order: VecDeque<UnitId>
}


impl Board {
  fn new(map: Vec<Square>, width: usize, units: HashMap<UnitId, Unit>) -> Board {
    let mut unit_order = Board::get_unit_order(&map);
    Board {
      map,
      width,
      units,
      completed_rounds: 0,
      current_round: Round { unit_order },
    }
  }
  fn get_unit_order(map: &Vec<Square>) -> VecDequeu<UnitId> {
    map.iter().flat_map(|s| match s {
      Square::Elf(id) => Some(*id),
      Square::Goblin(id) => Some(*id),
      _ => None
    })
  }
  fn print(&self) {
    let mut output = Vec::with_capacity(self.map.len() + self.width);
    let mut i = 0;

    let mut health = vec![];

    for s in &self.map {
      match s {
        Square::Wall => output.push('#' as u8),
        Square::Open => output.push('.' as u8),
        Square::Elf(id) => {
          let u = self.units.get(&id).unwrap();
          health.push((u.turns, u.hit_points));
          if u.turns == self.completed_turns {
            output.push('e' as u8);
          } else {
            output.push('E' as u8);
          }
        }
        Square::Goblin(id) => {
          let u = self.units.get(&id).unwrap();
          health.push((u.turns, u.hit_points));
          if u.turns == self.completed_turns {
            output.push('g' as u8);
          } else {
            output.push('G' as u8);
          }
        }
      }
      i += 1;
      if i % self.width == 0 {
        let hitpoints = format!(" {:?}", health);
        health.clear();
        output.append(&mut hitpoints.into_bytes());
        output.push('\n' as u8);
      }
    }
    stdout().write(&output);
  }

  fn get(&self, x: usize, y: usize) -> Square {
    if x < self.width {
      let i = x + self.width * y;
      if i < self.map.len() {
        return self.map[i];
      }
    }
    Square::Wall
  }

  fn set(&mut self, x: usize, y: usize, square: Square) {
    if x < self.width {
      let i = x + self.width * y;
      if i < self.map.len() {
        self.map[i] = square;
        return;
      }
    }
  }

  fn step(&mut self) -> GameStatus {
    let Some()

    let mut opt_current = {
      let k = self
          .units
          .iter()
          .max_by_key(|(k, v)| *v)
          .map(|(k, v)| k.clone());
      k.and_then(|k| self.units.remove(&k))
    };
    if let Some(mut current_unit) = opt_current {
      self.completed_turns = current_unit.turns;
      let enemy_tpe = current_unit.tpe.other().square();

      if !self
          .surrounding_squares(current_unit.position)
          .into_iter()
          .any(|p| self.get(p.x, p.y) == enemy_tpe) {
        let goal_paths: Vec<_> = {
          let targets = self.units.values().filter(|u| u.tpe != current_unit.tpe).map(|u| u.position).collect::<Vec<_>>();
          if targets.len() == 0 {
            self.units.insert(current_unit.position, current_unit);
            return GameStatus::Over;
          }
          targets.into_iter().flat_map(|p| {
            self.open_surrounding_squares(p)
          })
              .flat_map(|goal| {
                self.find_shortest_path_to(current_unit.position, goal)
                    .map(|path| (goal, path))
              })
              .collect()
        };

        if let Some((_, path)) = goal_paths.iter().min_by(|(lg, lp), (rg, rp)| {
          lp.len().cmp(&rp.len())
              .then_with(|| rg.cmp(lg))
        }) {
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
        let (pos, min_health) = targets.into_iter()
            .map(|p| (p, self.units.get(&p).unwrap().hit_points))
            .min_by(|(lp, lh), (rp, rh)| {
              lh.cmp(rh)
                  .then_with(|| rp.cmp(lp))
            })
            .unwrap();
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

  fn find_shortest_path_to_multiple(
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
  fn find_shortest_path_to(
    &self,
    start: Position,
    target: Position,
  ) -> Option<Vec<Position>> {
    let mut queue = BinaryHeap::new();
    let mut considered = HashSet::new();
    queue.push((0, start, vec![]));
    considered.insert(start);

    while let Some((dist, next, path)) = queue.pop() {
      if target == next {
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
