use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use utils::*;

fn get_nums(input: StringInput) -> Result<Vec<i64>> {
  let lines = input.lines();
  let mut result = Vec::with_capacity(lines.len());
  for line in input.lines() {
    if line.starts_with("+") {
      result.push(line.chars().skip(1).collect::<String>().parse::<i64>()?);
    } else {
      result.push(line.parse::<i64>()?);
    }
  }
  Ok(result)
}

pub fn adjust_frequency(input: StringInput) -> PuzzleResult {
  let result = (get_nums(input)?).iter().sum();
  Ok(PuzzleSolution::i64(result))
}

pub fn find_duplicate_frequency(input: StringInput) -> PuzzleResult {
  let mut result = 0;
  let mut nums = get_nums(input)?;
  let mut already_seen = HashSet::new();
  already_seen.insert(result);
  let mut should_continue = true;
  while should_continue {
    for x in nums.iter() {
      result += x;
      if !already_seen.insert(result) {
        return Ok(PuzzleSolution::i64(result));
      }
    }
  }
  Err(Box::new(CouldNotFindSolution))
}
