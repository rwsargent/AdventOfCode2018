use std::error;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Read;

pub type Result<T> = std::result::Result<T, Box<error::Error>>;
pub type PuzzleResult = Result<PuzzleSolution>;

#[derive(Debug, PartialEq, Eq)]
pub enum PuzzleSolution {
  Day01(i64),
  Day02a(u64),
  Day02b(String),
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
pub struct InvalidDay;

impl error::Error for InvalidDay {}

impl fmt::Display for InvalidDay {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Unexpected day specified")
  }
}

pub struct InputFile {
  pub path: String,
  pub content: String,
}

impl InputFile {
  pub fn new(path: String) -> Result<InputFile> {
    let mut file = File::open(path.clone())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(InputFile {
      path,
      content,
    })
  }

  pub fn lines(&self) -> Vec<&str> {
    self.content.lines().collect()
  }
}