use utils::*;

pub fn react(input: StringInput) -> PuzzleResult {
  let mut target: Vec<char> = Vec::with_capacity(input.content.len());
  for c in input.content.chars() {
    match target.pop() {
      Some(last) => {
        if last.is_uppercase() != c.is_uppercase()
            && last.to_ascii_lowercase() == c.to_ascii_lowercase()
            {} else {
          target.push(last);
          target.push(c);
        }
      }
      None => {
        target.push(c);
      }
    }
  }
  Ok(PuzzleSolution::usize(target.len()))
}

pub fn react_smallest(input: StringInput) -> PuzzleResult {
  "abcdefghijklmnopqrstuvwxyz"
      .chars()
      .map(|c| {
        let mut result = String::with_capacity(input.content.capacity());
        for x in input.content.chars() {
          if x.to_ascii_lowercase() != c {
            result.push(x);
          }
        }
        react(StringInput::from_string(result))
      })
      .min_by_key(|x| match x {
        Ok(PuzzleSolution::usize(x)) => *x,
        _ => 0 as usize,
      })
      .unwrap()
}
