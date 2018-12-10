extern crate regex;

use std::env;

use utils::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
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
    "5" | "5a" => {
      day05::react(StringInput::fromFile(args[0].clone())?)
    }
    "5b" => {
      day05::react_smallest(StringInput::fromFile(args[0].clone())?)
    }
    "6" | "6a" => {
      day06::largest_finite(StringInput::fromFile(args[0].clone())?)
    }
    "6b" => {
      day06::total_distance_under(StringInput::fromFile(args[0].clone())?, args[1].parse()?)
    }
    "7" | "7a" => {
      day07::get_execution_order(StringInput::fromFile(args[0].clone())?)
    }
    "7b" => {
      day06::total_distance_under(StringInput::fromFile(args[0].clone())?, args[1].parse()?)
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
    )).unwrap(), PuzzleSolution::usize(4));
  }

  #[test]
  fn day03b() {
    assert_eq!(day03::find_nonoverlapping_claim(StringInput::fromString(
      "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2".to_string()
    )).unwrap(), PuzzleSolution::usize(3));
  }

  #[test]
  fn day06a() {
    assert_eq!(day06::largest_finite(StringInput::fromString(
      "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9".to_string()
    )).unwrap(), PuzzleSolution::usize(17));
  }

  #[test]
  fn day06b() {
    assert_eq!(day06::total_distance_under(StringInput::fromString(
      "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9".to_string()
    ), 32).unwrap(), PuzzleSolution::usize(16));
  }

  #[test]
  fn day07() {
    assert_eq!(day07::get_execution_order(StringInput::fromString(
      "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.".to_string()
    )).unwrap(), PuzzleSolution::String("CABDFE".to_string()));
  }

  #[test]
  fn day07b() {
    assert_eq!(day07::get_execution_time(StringInput::fromString(
      "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.".to_string()
    ), 2, 0).unwrap(), PuzzleSolution::usize(15));
  }

  fn expected_results() -> Vec<(&'static str, Vec<String>, PuzzleSolution)> {
    vec![
      ("1a", vec!["data/day01.txt".to_string()], PuzzleSolution::i64(533)),
      ("1b", vec!["data/day01.txt".to_string()], PuzzleSolution::i64(73272)),
      ("2a", vec!["data/day02.txt".to_string()], PuzzleSolution::u64(8820)),
      ("2b", vec!["data/day02.txt".to_string()], PuzzleSolution::String("bpacnmglhizqygfsjixtkwudr".to_string())),
      ("3a", vec!["data/day03.txt".to_string()], PuzzleSolution::usize(113576)),
      ("3b", vec!["data/day03.txt".to_string()], PuzzleSolution::usize(825)),
      ("4a", vec!["data/day04.txt".to_string()], PuzzleSolution::usize(140932)),
      ("4b", vec!["data/day04.txt".to_string()], PuzzleSolution::usize(51232)),
      ("5a", vec!["data/day05.txt".to_string()], PuzzleSolution::usize(10250)),
      ("5b", vec!["data/day05.txt".to_string()], PuzzleSolution::usize(6188)),
      ("6a", vec!["data/day06.txt".to_string()], PuzzleSolution::usize(4186)),
      ("6b", vec!["data/day06.txt".to_string(), "10000".to_string()], PuzzleSolution::usize(45509)),
      ("7a", vec!["data/day07.txt".to_string()], PuzzleSolution::String("FMOXCDGJRAUIHKNYZTESWLPBQV".to_string()))
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
