use std::collections::HashSet;

use regex::Regex;

use utils::*;

struct Claim {
  id: usize,
  start: Point,
  end: Point,
}

struct Point {
  x: usize,
  y: usize,
}

fn get_claims(input: StringInput) -> Result<Vec<Claim>> {
  let mut claims = vec![];

  let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")?;
  for cap in re.captures_iter(&input.content) {
    let id = cap[1].parse::<usize>()?;
    let start = Point {
      x: cap[2].parse::<usize>()?,
      y: cap[3].parse::<usize>()?,
    };
    let end = Point {
      x: start.x + cap[4].parse::<usize>()?,
      y: start.y + cap[5].parse::<usize>()?,
    };
    claims.push(Claim { id, start, end });
  }
  Ok(claims)
}

pub fn count_overlapping_squares(input: StringInput) -> PuzzleResult {
  let claims = get_claims(input)?;

  let mut count = 0;

  let mut claimed = ExpandableMatrix::new(0);

  for claim in claims {
    for y in claim.start.y..claim.end.y {
      for x in claim.start.x..claim.end.x {
        let val = claimed.get_mut(x, y);
        *val += 1;
        if *val == 2 {
          count += 1;
        }
      }
    }
  }

  Ok(PuzzleSolution::usize(count))
}

pub fn find_nonoverlapping_claim(input: StringInput) -> PuzzleResult {
  let claims = get_claims(input)?;

  let mut free_claims = HashSet::new();

  let mut claimed = ExpandableMatrix::new(0);

  for claim in claims {
    let mut untouched = true;
    for y in claim.start.y..claim.end.y {
      for x in claim.start.x..claim.end.x {
        let val = claimed.get_mut(x, y);
        if *val != 0 {
          untouched = false;
          free_claims.remove(val);
        } else {
          *val = claim.id;
        }
      }
    }
    if untouched {
      free_claims.insert(claim.id);
    }
  }

  Ok(PuzzleSolution::usize(
    free_claims.into_iter().next().ok_or(CouldNotFindSolution)?,
  ))
}
