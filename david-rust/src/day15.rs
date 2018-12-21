use std::{thread, time};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt;
use std::io::{BufRead, stdin, stdout, Write};

use utils::*;

pub fn do_battle(input: StringInput) -> PuzzleResult {
  let mut board = parse_board(input)?;
  board.print();
  let mut buf = String::new();
  let stdin = stdin();
  for line in stdin.lock().lines() {
    let line = line.unwrap();
    match &line[..] {
      "q" => {
        return Ok(PuzzleSolution::unit);
      }
      "f" => break,
      "" => {
        board.step();
        board.print();
      }
      x => {
        match x.parse::<usize>() {
          Ok(steps) => {
            for _ in 0..steps {
              board.step();
              board.print();
            }
          }
          _ => ()
        }
      }
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
    board.completed_rounds,
    leftover_hitpoints,
    board.completed_rounds * leftover_hitpoints,
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
    board.completed_rounds,
    leftover_hitpoints,
    board.completed_rounds * leftover_hitpoints,
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
              id,
              tpe: UnitType::Elf,
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
              id,
              tpe: UnitType::Goblin,
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

// We implement Ord for Position so that we can use a the std BinaryHeap which is max heap.
// We want the heap to return the Position that is first in 'reading order'.
// Position is x to the right, y down. Reading order is lowest y first, otherwise lowest x.
impl Ord for Position {
  fn cmp(&self, other: &Self) -> Ordering {
    other.y.cmp(&self.y).then(other.x.cmp(&self.x))
  }
}

#[derive(PartialEq, Eq, Debug)]
struct Unit {
  id: UnitId,
  tpe: UnitType,
  hit_points: usize,
  attack_power: usize,
  position: Position,
}

impl Unit {
  fn is_enemy(&self, square: Square) -> bool {
    match self.tpe {
      UnitType::Elf => {
        match square {
          Square::Goblin(_) => true,
          _ => false,
        }
      }
      UnitType::Goblin => {
        match square {
          Square::Elf(_) => true,
          _ => false,
        }
      }
    }
  }
  fn square(&self) -> Square {
    match self.tpe {
      UnitType::Elf => Square::Elf(self.id),
      UnitType::Goblin => Square::Goblin(self.id),
    }
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

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct UnitId(usize);

struct Board {
  map: Vec<Square>,
  width: usize,
  completed_rounds: usize,
  units: HashMap<UnitId, Unit>,
  round: Round,
}

struct Round {
  unit_order: VecDeque<UnitId>
}


impl Board {
  fn new(map: Vec<Square>, width: usize, units: HashMap<UnitId, Unit>) -> Board {
    let mut round = Board::get_fresh_round(&map);
    Board {
      map,
      width,
      units,
      completed_rounds: 0,
      round,
    }
  }
  fn get_fresh_round(map: &Vec<Square>) -> Round {
    Round {
      unit_order: map.iter()
          .flat_map(|s| match s {
            Square::Elf(id) => Some(*id),
            Square::Goblin(id) => Some(*id),
            _ => None
          })
          .collect()
    }
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
          health.push(u.hit_points);
          output.push('E' as u8);
        }
        Square::Goblin(id) => {
          let u = self.units.get(&id).unwrap();
          health.push(u.hit_points);
          output.push('G' as u8);
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

  fn set(&mut self, pos: Position, square: Square) {
    if pos.x < self.width {
      let i = pos.x + self.width * pos.y;
      if i < self.map.len() {
        self.map[i] = square;
        return;
      }
    }
  }

  fn step(&mut self) -> GameStatus {
    if let Some(id) = self.round.unit_order.pop_front() {
      // ** Combat proceeds in rounds; in each round, each unit that is still alive takes a turn,
      // ** resolving all of its actions before the next unit's turn begins. On each unit's turn,
      // ** it tries to move into range of an enemy (if it isn't already) and then attack (if it is in range).
      if let Some(mut current_unit) = self.units.remove(&id) {

        // ** Each unit begins its turn by identifying all possible targets (enemy units).
        // ** If no targets remain, combat ends.
        let target_units: Vec<_> = self.units.values()
            .filter(|u| u.tpe != current_unit.tpe)
            .map(|u| u.id)
            .collect();
        if target_units.len() == 0 {
          // to leave the board as it was found.
          self.units.insert(id, current_unit);
          return GameStatus::Over;
        }
        // ** Perform actions: Move and Attack

        // ** If the unit is already in range of a target, it does not move, but continues its turn
        // ** with an attack. Otherwise, since it is not in range of a target, it moves.
        if !self.surrounding_squares(current_unit.position)
            .into_iter()
            .any(|p| current_unit.is_enemy(self.get(p.x, p.y))) {
          // The unit should try to move
          // ** To move, the unit first considers the squares that are in range and determines which
          // ** of those squares it could reach in the fewest steps. A step is a single movement to
          // ** any adjacent (immediately up, down, left, or right) open (.) square. Units cannot move
          // ** into walls or other units. The unit does this while considering the current positions
          // ** of units and does not do any prediction about where units will be later. If the unit
          // ** cannot reach (find an open path to) any of the squares that are in range, it ends its
          // ** turn. If multiple squares are in range and tied for being reachable in the fewest steps,
          // ** the square which is first in reading order is chosen.
          // **
          // ** Then, the unit identifies all of the open squares (.) that are in range of each target;
          // ** these are the squares which are adjacent (immediately up, down, left, or right) to any
          // ** target and which aren't already occupied by a wall or another unit. Alternatively, the
          // ** unit might already be in range of a target. If the unit is not already in range of a
          // ** target, and there are no open squares which are in range of a target, the unit ends its turn.
          let target_positions = target_units.iter().flat_map(|uid| {
            self.open_surrounding_squares(self.units.get(uid).unwrap().position)
          }).collect();
          if let Some(path) = self.find_best_path(current_unit.position, target_positions) {
            let next = path[0];
            self.set(
              current_unit.position,
              Square::Open,
            );
            self.set(next, current_unit.square());
            current_unit.position = next;
          }
        }
        // ** After moving (or if the unit began its turn in range of a target), the unit attacks.

        // Now the unit should try to attack.
        // ** To attack, the unit first determines all of the targets that are in range of it by being
        // ** immediately adjacent to it. If there are no such targets, the unit ends its turn. Otherwise,
        // ** the adjacent target with the fewest hit points is selected; in a tie, the adjacent target
        // ** with the fewest hit points which is first in reading order is selected.
        if let Some((id, _, min_health)) = self
            .surrounding_squares(current_unit.position)
            .into_iter()
            .map(|p| self.get(p.x, p.y))
            .filter(|s| current_unit.is_enemy(*s))
            .map(|s| match s {
              Square::Elf(id) => {
                let u = self.units.get(&id).unwrap();
                (u.id, u.position, u.hit_points)
              }
              Square::Goblin(id) => {
                let u = self.units.get(&id).unwrap();
                (u.id, u.position, u.hit_points)
              }
              _ => unreachable!()
            })
            .min_by(|(_, lp, lh), (_, rp, rh)| {
              lh.cmp(rh)
                  .then_with(|| rp.cmp(lp))
            }) {
          // ** The unit deals damage equal to its attack power to the selected target, reducing its hit
          // ** points by that amount. If this reduces its hit points to 0 or fewer, the selected target
          // ** dies: its square becomes . and it takes no further turns.
          if min_health <= current_unit.attack_power {
            // target unit is dead
            let target = self.units.remove(&id).unwrap();
            self.round.unit_order.retain(|uid| *uid != id);
            self.set(target.position, Square::Open);
          } else {
            let mut target = self.units.get_mut(&id).unwrap();
            target.hit_points -= current_unit.attack_power;
          }
        }
        self.units.insert(id, current_unit);
      }
    }

    if self.round.unit_order.len() == 0 {
      // start a new round
      self.round = Board::get_fresh_round(&self.map);
      self.completed_rounds += 1;
    }

    GameStatus::Continue
  }

  fn find_best_path(
    &self,
    start: Position,
    goals: HashSet<Position>,
  ) -> Option<Vec<Position>> {
    // this is a max heap
    let mut queue = BinaryHeap::new();
    let mut considered = HashSet::new();
    queue.push((0, start, vec![]));
    considered.insert(start);

    let mut results: Vec<Vec<Position>> = vec![];
    let mut best_distance = 0;

    while let Some((dist, next, path)) = queue.pop() {
      // if we have found valid paths, but we are now only considering paths > the best distance
      // break.
      if best_distance != 0 && path.len() > best_distance {
        break;
      }
      if goals.contains(&next) {
        if best_distance == 0 || best_distance == path.len() {
          best_distance = path.len();
          results.push(path);
          continue;
        }
      }
      for next_move in self.open_surrounding_squares(next) {
        if considered.insert(next_move) {
          let mut path = path.clone();
          path.push(next_move);
          queue.push((dist - 1, next_move, path));
        }
      }
    }
    if results.len() > 1 {
      // return the best path

      println!("choosing from paths: {:?}", results);
      let r = results.into_iter().max_by(|l, r| {
        l.last().unwrap().cmp(r.last().unwrap())
            .then_with(|| l.first().unwrap().cmp(r.first().unwrap()))
      });
      println!("best: {:?}", r);
      return r;
    }
    results.pop()
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
