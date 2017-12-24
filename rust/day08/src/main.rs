use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::num;
use std::ops::Index;
use std::slice::Split;

fn main() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let (highest_value, registers) = run_instructions(&contents);
    println!("max register {:?}", registers.iter().max_by(|x, y| (*x).1.cmp(&(*y).1)).unwrap());
    println!("highest_value {:?}", highest_value);
}

fn run_instructions(input: &String) -> (i64, HashMap<String, i64>) {
    let mut registers = HashMap::new();
    let mut highest_value = 0;
    input.split("\n").filter(|x| x.len() > 0).for_each(|line| {
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        let register = tokens[0].to_string();
        let instruction = tokens[1];
        let amount = tokens[2].parse::<i64>().unwrap();
        // tokens[3] should always be "if"
        let cond_register = tokens[4].to_string();
        let condition = tokens[5];
        let cond_amount = tokens[6].parse::<i64>().unwrap();
        let cond_register_value = registers.get(&cond_register).map(|x| *x).unwrap_or_default();
        let do_work = match condition {
            ">" => {
                cond_register_value > cond_amount
            }
            "<" => {
                cond_register_value < cond_amount
            }
            "<=" => {
                cond_register_value <= cond_amount
            }
            ">=" => {
                cond_register_value >= cond_amount
            }
            "==" => {
                cond_register_value == cond_amount
            }
            "!=" => {
                cond_register_value != cond_amount
            }
            _ => unimplemented!()
        };
        if do_work {
            let register_value = registers.get(&register).map(|x| *x).unwrap_or_default();
            match instruction {
                "inc" => {
                    registers.insert(register, register_value + amount);
                }
                "dec" => {
                    registers.insert(register, register_value - amount);
                }
                _ => unimplemented!()
            }
            let new_highest_value = registers.iter().map(|f| *f.1).max().unwrap();
            if new_highest_value > highest_value {
                highest_value = new_highest_value;
            }
        }
    });
    (highest_value, registers)
}

#[test]
fn instruction_test() {
    let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10".to_string();
    assert_eq!(run_instructions(&input).1.iter().map(|f| *f.1).max().unwrap(), 1);
}
