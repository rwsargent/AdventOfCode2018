use std::collections::HashMap;

use utils::*;

pub fn checksum(path: &String) -> PuzzleResult {
  let file = StringInput::from_file(path)?;
  let mut has_2_chars = 0;
  let mut has_3_chars = 0;
  for line in file.lines() {
    let mut char_count = HashMap::new();
    for c in line.chars() {
      let entry = char_count.entry(c).or_insert(0);
      *entry += 1
    }
    let mut found_2 = false;
    let mut found_3 = false;
    for (_, v) in char_count {
      found_2 = found_2 || (v == 2);
      found_3 = found_3 || (v == 3);
    }
    if found_2 {
      has_2_chars += 1;
    }
    if found_3 {
      has_3_chars += 1;
    }
  }

  let result = has_2_chars * has_3_chars;
  Ok(PuzzleSolution::u64(result))
}

pub fn correct_common_letters(path: &String) -> PuzzleResult {
  let file = StringInput::from_file(path)?;
  for line in file.lines() {
    for line2 in file.lines() {
      let dist = distance(line, line2);
      if dist == 1 {
        let mut result = String::new();
        for (l, r) in line.chars().zip(line2.chars()) {
          if l == r {
            result.push(l);
          }
        }
        return Ok(PuzzleSolution::String(result));
      }
    }
  }
  Err(Box::new(CouldNotFindSolution))
}

// will return the number of characters that do not match
fn distance(left: &str, right: &str) -> usize {
  // technically smaller could be the same length as larger
  let (smaller, larger) = if left.len() > right.len() {
    (right, left)
  } else {
    (left, right)
  };
  let mut result = larger.len() - smaller.len();
  for (l, r) in smaller.chars().zip(larger.chars()) {
    if l != r {
      result += 1;
    }
  }

  result
}
