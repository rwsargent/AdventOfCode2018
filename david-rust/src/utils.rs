use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Read;
use std::iter;

pub type Result<T> = std::result::Result<T, Box<error::Error>>;
pub type PuzzleResult = Result<PuzzleSolution>;

#[derive(Debug, PartialEq, Eq)]
pub enum PuzzleSolution {
  i64(i64),
  u64(u64),
  String(String),
  usize(usize),
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
pub struct InvalidLine {
  pub line: String
}

impl error::Error for InvalidLine {}

impl fmt::Display for InvalidLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Input line defied expectations {}", self.line)
  }
}

#[derive(Debug)]
pub struct CouldNotFindSolution;

impl error::Error for CouldNotFindSolution {}

impl fmt::Display for CouldNotFindSolution {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "A solution could not be found. Is there a bug? Is your input bad?")
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
  pub fn fromFile(path: String) -> Result<StringInput> {
    let mut file = File::open(path.clone())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(StringInput {
      path,
      content,
    })
  }

  pub fn fromString(content: String) -> StringInput {
    StringInput {
      path: "".to_string(),
      content,
    }
  }

  pub fn lines(&self) -> Vec<&str> {
    self.content.lines().collect()
  }
}

/// An expandable zero indexed matrix.
pub struct ExpandableMatrix<T: Clone> {
  zero: T,
  currentWidth: usize,
  data: Vec<Vec<T>>,
}

impl<T: Clone> ExpandableMatrix<T> {
  pub fn new(zero: T) -> ExpandableMatrix<T> {
    ExpandableMatrix {
      zero,
      currentWidth: 0,
      data: vec![],
    }
  }

  pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
    if self.data.len() <= y {
      let num_extra = y - self.data.len() + 1;
      self.data.extend(iter::repeat(vec![]).take(num_extra));
    }
    let mut vec = self.data.get_mut(y).unwrap();
    if vec.len() <= x {
      let num_extra = x - vec.len() + 1;
      vec.extend(iter::repeat(self.zero.clone()).take(num_extra));
    }
    vec.get_mut(x).unwrap()
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point {
  pub x: i64,
  pub y: i64,
}

impl Point {
  pub fn manhattan_dist(&self, other: &Point) -> i64 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}

pub struct MultiMap<K: Eq + Hash, V> {
  map: HashMap<K, Vec<V>>
}

impl<K: Eq + Hash, V> MultiMap<K, V> {
  pub fn new() -> MultiMap<K, V> {
    MultiMap { map: HashMap::new() }
  }

  pub fn insert(&mut self, key: K, value: V) {
    self.map.entry(key).or_insert(Vec::with_capacity(1)).push(value)
  }

  pub fn get(&self, key: &K) -> Option<&Vec<V>> {
    self.map.get(key)
  }
}
