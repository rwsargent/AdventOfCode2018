use std::env;

use utils::*;

mod day01;
mod day02;
mod utils;

fn main() {
  let day = env::args().skip(1).take(1).next().unwrap();
  let args: Vec<String> = env::args().skip(2).collect();
  println!("{:?}", solve(day.as_ref(), args));
}

fn solve(day: &str, args: Vec<String>) -> PuzzleResult {
  match day {
    "1" | "1a" => {
      day01::adjust_frequency(args[0].clone())
    }
    "1b" => {
      day01::find_duplicate_frequency(args[0].clone())
    }
    "2" | "2a" => {
      day02::checksum(args[0].clone())
    }
    "2b" => {
      day02::correct_common_letters(args[0].clone())
    }
    x => {
      Err(Box::new(InvalidDay))
    }
  }
}


#[cfg(test)]
mod test {
  use super::*;

  fn expected_results() -> Vec<(&'static str, Vec<String>, PuzzleSolution)> {
    vec![
      ("1a", vec!["data/day01.txt".to_string()], PuzzleSolution::Day01(533)),
      ("1b", vec!["data/day01.txt".to_string()], PuzzleSolution::Day01(73272)),
      ("2a", vec!["data/day02.txt".to_string()], PuzzleSolution::Day02a(8820)),
      ("2b", vec!["data/day02.txt".to_string()], PuzzleSolution::Day02b("bpacnmglhizqygfsjixtkwudr".to_string()))
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
