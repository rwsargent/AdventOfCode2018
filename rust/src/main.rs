extern crate regex;
#[macro_use] extern crate lazy_static;

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
        x => println!("Unknown day {}.", x)
    }
}
