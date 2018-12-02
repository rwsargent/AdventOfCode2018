use std::env;

mod day01;
mod utils;

fn main() {
  let day = env::args().skip(1).take(1).next().unwrap();
  let args: Vec<String> = env::args().skip(2).collect();
  match day.as_ref() {
    "1" | "1a" => {
      day01::adjust_frequency(args[0].clone());
    }
    "1b" => {
      day01::find_duplicate_frequency(args[0].clone());
    }
    x => {
      println!("Invalid day specified: {}", x);
    }
  }
}
