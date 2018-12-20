use std::cell::RefCell;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::iter;
use std::ops;
use std::rc::Rc;

use bit_set::BitSet;

pub type Result<T> = std::result::Result<T, Box<error::Error>>;
pub type PuzzleResult = Result<PuzzleSolution>;

#[derive(Debug, PartialEq, Eq)]
pub enum PuzzleSolution {
  i64(i64),
  i32(i32),
  u64(u64),
  String(String),
  usize(usize),
  Point(Point),
  PointSize(Point, usize),
  Triplet(usize, usize, usize),
  unit,
}

#[derive(Debug)]
pub struct InvalidInput;

impl error::Error for InvalidInput {}

impl fmt::Display for InvalidInput {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Input file defied expectations")
  }
}

#[derive(Debug)]
pub struct InvalidLine(pub String);

impl error::Error for InvalidLine {}

impl fmt::Display for InvalidLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Input line defied expectations {}", self.0)
  }
}

#[derive(Debug)]
pub struct CouldNotFindSolution;

impl error::Error for CouldNotFindSolution {}

impl fmt::Display for CouldNotFindSolution {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "A solution could not be found. Is there a bug? Is your input bad?"
    )
  }
}

#[derive(Debug)]
pub struct InvalidDay;

impl error::Error for InvalidDay {}

impl fmt::Display for InvalidDay {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Unexpected day specified")
  }
}

pub struct StringInput {
  pub path: String,
  pub content: String,
}

impl StringInput {
  pub fn from_file(path: &String) -> Result<StringInput> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(StringInput {
      path: path.to_string(),
      content,
    })
  }

  pub fn from_string(content: String) -> StringInput {
    StringInput {
      path: "".to_string(),
      content,
    }
  }

  pub fn from_str(contents: &str) -> StringInput {
    StringInput::from_string(contents.to_string())
  }

  pub fn lines(&self) -> Vec<&str> {
    self.content.lines().collect()
  }
}

/// An expandable zero indexed matrix.
pub struct ExpandableMatrix<T: Clone> {
  zero: T,
  data: Vec<Vec<T>>,
}

impl<T: Clone> ExpandableMatrix<T> {
  pub fn new(zero: T) -> ExpandableMatrix<T> {
    ExpandableMatrix { zero, data: vec![] }
  }

  pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
    if self.data.len() <= y {
      let num_extra = y - self.data.len() + 1;
      self.data.extend(iter::repeat(vec![]).take(num_extra));
    }
    let vec = self.data.get_mut(y).unwrap();
    if vec.len() <= x {
      let num_extra = x - vec.len() + 1;
      vec.extend(iter::repeat(self.zero.clone()).take(num_extra));
    }
    vec.get_mut(x).unwrap()
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
pub struct Point {
  pub x: i64,
  pub y: i64,
}

impl Point {
  pub fn manhattan_dist(&self, other: &Point) -> i64 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}

impl ops::Add<&Point> for &Point {
  type Output = Point;

  fn add(self, other: &Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl ops::Sub<&Point> for &Point {
  type Output = Point;

  fn sub(self, other: &Point) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

pub struct MultiMap<K: Eq + Hash, V> {
  pub map: HashMap<K, Vec<V>>,
}

impl<K: Eq + Hash, V> MultiMap<K, V> {
  pub fn new() -> MultiMap<K, V> {
    MultiMap {
      map: HashMap::new(),
    }
  }

  pub fn insert(&mut self, key: K, value: V) {
    self.map
        .entry(key)
        .or_insert(Vec::with_capacity(1))
        .push(value)
  }

  pub fn get(&self, key: &K) -> Option<&Vec<V>> {
    self.map.get(key)
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Tape {
  neg: BitSet<u64>,
  pos: BitSet<u64>,
  pub min: i32,
  pub max: i32,
}

impl Tape {
  pub fn new() -> Tape {
    Tape {
      neg: BitSet::default(),
      pos: BitSet::default(),
      min: 0,
      max: 0,
    }
  }
  pub fn contains(&self, idx: i32) -> bool {
    if idx < 0 {
      self.neg.contains((-idx) as usize)
    } else {
      self.pos.contains(idx as usize)
    }
  }
  pub fn insert(&mut self, idx: i32) -> bool {
    if idx < 0 {
      if idx < self.min {
        self.min = idx;
      }
      self.neg.insert(-idx as usize)
    } else {
      if idx > self.max {
        self.max = idx;
      }
      self.pos.insert(idx as usize)
    }
  }
  pub fn remove(&mut self, idx: i32) -> bool {
    if idx < 0 {
      self.neg.remove(-idx as usize)
    } else {
      self.pos.remove(idx as usize)
    }
  }
  pub fn iter(self) -> impl iter::Iterator<Item=i32> {
    (self.min..=self.max).filter(move |i| self.contains(*i))
  }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

impl Direction {
  pub fn left(&self) -> Direction {
    match self {
      Direction::North => Direction::West,
      Direction::West => Direction::South,
      Direction::South => Direction::East,
      Direction::East => Direction::North,
    }
  }
  pub fn right(&self) -> Direction {
    match self {
      Direction::North => Direction::East,
      Direction::West => Direction::North,
      Direction::South => Direction::West,
      Direction::East => Direction::South,
    }
  }
}

struct Node<T> {
  pub data: T,
  pub prev: Option<Rc<RefCell<Node<T>>>>,
  pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
  pub fn new(data: T) -> Self {
    Self {
      data,
      prev: None,
      next: None,
    }
  }

  pub fn split(
    &self,
  ) -> (
    T,
    Option<Rc<RefCell<Node<T>>>>,
    Option<Rc<RefCell<Node<T>>>>,
  )
    where
        T: Clone,
  {
    (self.data.clone(), self.prev.clone(), self.next.clone())
  }
}

pub struct CircularList<T> {
  len: usize,
  current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> CircularList<T> {
  pub fn new() -> Self {
    Self {
      len: 0,
      current: None,
    }
  }

  pub fn move_prev(&mut self) {
    let next = if let Some(ref cur) = self.current {
      cur.borrow().prev.clone()
    } else {
      None
    };
    self.current = next;
  }

  pub fn move_next(&mut self) {
    let prev = if let Some(ref cur) = self.current {
      cur.borrow().next.clone()
    } else {
      None
    };
    self.current = prev;
  }

  /// Removes the current pointer, returning the data there, and sets current to next.
  pub fn remove_current(&mut self) -> Option<T>
    where
        T: Clone,
  {
    if self.len == 0 {
      None
    } else {
      self.len -= 1;
      if self.len == 0 {
        let cur = self.current.take();
        let cur = cur.unwrap();
        let cur = cur.borrow();
        let data = cur.data.clone();
        Some(data)
      } else {
        if let Some(cur) = self.current.take() {
          let cur = cur.borrow();
          let (data, prev, next) = cur.split();
          if let Some(ref p) = prev {
            p.borrow_mut().next = cur.next.clone();
          }
          if let Some(ref n) = next {
            n.borrow_mut().prev = cur.prev.clone();
          }
          self.current = next;
          Some(data)
        } else {
          None
        }
      }
    }
  }

  /// Inserts data to the right (as next) of current and sets current to point to new data.
  pub fn insert(&mut self, data: T) {
    self.current = Some(if let Some(cur) = self.current.take() {
      let mut new_node = Node::new(data);
      new_node.next = cur.borrow().next.clone();
      new_node.prev = Some(cur.clone());

      let rc = Rc::new(RefCell::new(new_node));

      if self.len == 1 {
        let mut cur = cur.borrow_mut();
        cur.next = Some(rc.clone());
        cur.prev = Some(rc.clone());
      } else {
        if let Some(ref mut n) = cur.borrow_mut().next {
          n.borrow_mut().prev = Some(rc.clone());
        }
        cur.borrow_mut().next = Some(rc.clone());
      }
      rc
    } else {
      let mut node = Node::new(data);
      let rc = Rc::new(RefCell::new(node));
      rc.borrow_mut().next = Some(rc.clone());
      rc.borrow_mut().prev = Some(rc.clone());
      rc
    });
    self.len += 1;
  }
}

impl<T: Display> Display for CircularList<T> {
  fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(w, "{} [", self.len)?;
    let mut node = self.current.clone();
    let mut count = 0;
    while let Some(n) = node {
      count += 1;
      write!(w, "{}", n.borrow().data)?;
      node = if count < self.len {
        write!(w, ", ")?;
        n.borrow().next.clone()
      } else {
        None
      }
    }
    write!(w, "]")
  }
}
