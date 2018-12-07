extern crate regex;

use std::env;

use utils::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod utils;

fn main() {
  let day = env::args().skip(1).take(1).next().unwrap();
  let args: Vec<String> = env::args().skip(2).collect();
  println!("{:?}", solve(day.as_ref(), args));
}

fn solve(day: &str, args: Vec<String>) -> PuzzleResult {
  match day {
    "1" | "1a" => {
      day01::adjust_frequency(StringInput::fromFile(args[0].clone())?)
    }
    "1b" => {
      day01::find_duplicate_frequency(StringInput::fromFile(args[0].clone())?)
    }
    "2" | "2a" => {
      day02::checksum(args[0].clone())
    }
    "2b" => {
      day02::correct_common_letters(args[0].clone())
    }
    "3" | "3a" => {
      day03::count_overlapping_squares(StringInput::fromFile(args[0].clone())?)
    }
    "3b" => {
      day03::find_nonoverlapping_claim(StringInput::fromFile(args[0].clone())?)
    }
    "4" | "4a" => {
      day04::most_asleep_guard(StringInput::fromFile(args[0].clone())?)
    }
    "4b" => {
      day04::most_asleep_minute(StringInput::fromFile(args[0].clone())?)
    }
    x => {
      Err(Box::new(InvalidDay))
    }
  }
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn day03a() {
    assert_eq!(day03::count_overlapping_squares(StringInput::fromString(
      "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2".to_string()
    )).unwrap(), PuzzleSolution::Day03(4));
  }

  #[test]
  fn day03b() {
    assert_eq!(day03::find_nonoverlapping_claim(StringInput::fromString(
      "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2".to_string()
    )).unwrap(), PuzzleSolution::Day03(3));
  }

  fn expected_results() -> Vec<(&'static str, Vec<String>, PuzzleSolution)> {
    vec![
      ("1a", vec!["data/day01.txt".to_string()], PuzzleSolution::Day01(533)),
      ("1b", vec!["data/day01.txt".to_string()], PuzzleSolution::Day01(73272)),
      ("2a", vec!["data/day02.txt".to_string()], PuzzleSolution::Day02a(8820)),
      ("2b", vec!["data/day02.txt".to_string()], PuzzleSolution::Day02b("bpacnmglhizqygfsjixtkwudr".to_string())),
      ("3a", vec!["data/day03.txt".to_string()], PuzzleSolution::Day03(113576)),
      ("3b", vec!["data/day03.txt".to_string()], PuzzleSolution::Day03(825)),
      ("4a", vec!["data/day04.txt".to_string()], PuzzleSolution::Day04(140932)),
      ("4b", vec!["data/day04.txt".to_string()], PuzzleSolution::Day04(51232)),
    ]
  }

  #[test]
  fn check_expected() {
    for (day, args, expected) in expected_results() {
      print!("checking day {} ... ", day);
      match solve(day, args) {
        Ok(result) => {
          assert_eq!(result, expected);
          println!("passed!");
        }
        Err(e) => {
          assert_eq!(1, 0, "ERROR: {:?}", e);
        }
      }
    }
  }
}
