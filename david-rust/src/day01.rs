use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use utils::*;

pub fn adjust_frequency(file: String) -> PuzzleResult {
  let mut result = 0;
  let file = BufReader::new(File::open(file)?);
  for line in file.lines() {
    let line = line?;
    if line.starts_with("+") {
      result += line.chars().skip(1).collect::<String>().parse::<i64>()?;
    } else {
      result += line.parse::<i64>()?;
    }
  }
  Ok(PuzzleSolution::Day01(result))
}

pub fn find_duplicate_frequency(file: String) -> PuzzleResult {
  let mut result = 0;
  let file = BufReader::new(File::open(file)?);
  let mut nums = Vec::new();
  for line in file.lines() {
    let line = line?;
    let x = if line.starts_with("+") {
      line.chars().skip(1).collect::<String>().parse::<i64>()?
    } else {
      line.parse::<i64>()?
    };
    nums.push(x);
  }
  let mut already_seen = HashSet::new();
  already_seen.insert(result);
  let mut should_continue = true;
  while should_continue {
    for x in nums.iter() {
      result += x;
      if !already_seen.insert(result) {
        return Ok(PuzzleSolution::Day01(result));
      }
    }
  }
  Err(Box::new(InvalidInput))
}
