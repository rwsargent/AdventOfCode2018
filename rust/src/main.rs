extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate chrono;

mod days;
mod util;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("You need to include the day")
    }
    let day = &args[1];
    match day.as_ref() {
        "day01" => days::day01::run(),
        "day02" => days::day02::run(),
        "day03" => days::day03::run(),
        "day04" => days::day04::run(),
        "day05" => days::day05::run(),
        "day06" => days::day06::run(),
        "day07" => days::day07::run(),
        x => println!("Unknown day {}.", x),
    }
}
