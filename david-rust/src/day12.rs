use bit_set::BitSet;
use regex::Regex;

use std::collections::HashMap;
use utils::*;

fn parse_input(input: StringInput) -> Result<(Tape, BitSet)> {
  let initial_re = Regex::new(r"initial state: ([#.]*)")?;
  let rule_re = Regex::new(r"([#.])([#.])([#.])([#.])([#.]) => ([#.])")?;

  let mut lines = input.lines().into_iter();
  let line = lines.next().ok_or(Box::new(InvalidInput))?;
  let init_caps = initial_re.captures(line).ok_or(Box::new(InvalidLine { line: line.to_string() }))?;

  let mut initial_tape = Tape::new();
  for (i, char) in init_caps[1].chars().enumerate() {
    match char {
      '#' => {
        initial_tape.insert(i as i32);
      }
      _ => {}
    }
  }

  let mut rules = BitSet::new();
  // blank line
  lines.next();
  for line in lines {
    let cap = rule_re.captures(line).ok_or(Box::new(InvalidLine { line: line.to_string() }))?;
    if is_plant(&cap[6]) {
      let mut value = 0;
      if is_plant(&cap[1]) {
        value += 1;
      }
      if is_plant(&cap[2]) {
        value += 2;
      }
      if is_plant(&cap[3]) {
        value += 4;
      }
      if is_plant(&cap[4]) {
        value += 8;
      }
      if is_plant(&cap[5]) {
        value += 16;
      }
      rules.insert(value);
    }
  }

  Ok((initial_tape, rules))
}

fn is_plant(s: &str) -> bool {
  match s {
    "#" => true,
    _ => false
  }
}

struct Rule {
  pattern: [bool; 5],
  result: bool,
}

fn get_value(tape: &Tape, idx: i32) -> usize {
  let mut value = 0;
  let mut m: usize = 1;
  for i in (idx - 2)..=(idx + 2) {
    if tape.contains(i) {
      value += m;
    }
    m = m << 1;
  }
  value
}

fn step(tape: &Tape, rules: &BitSet) -> Tape {
  let mut result = Tape::new();
  for i in (tape.min - 5)..=(tape.max + 5) {
    let val = get_value(tape, i);
    if rules.contains(val) {
      result.insert(i);
    }
  }
  result
}

fn print_tape(tape: &Tape) {
  let mut s = String::new();
  let mut got_one = false;
  for i in tape.min..=tape.max {
//    if i == 0 {
//      s.push('|');
//    }
    if tape.contains(i) {
      s.push('#');
      got_one = true;
    } else if got_one {
      s.push('.')
    }
  }
  print!("{} {} {}", tape.min, tape.max, s);
}

pub fn sum_pots_with_plants(input: StringInput, generations: usize) -> PuzzleResult {
  let (mut state, rules) = parse_input(input)?;

  let mut iterations_until_loop = 0;
  let mut first_sighting = 0;
  for i in 0..generations {
    state = step(&state, &rules);
    print!("\r{}  ", i);
    print_tape(&state);
  }

  Ok(PuzzleSolution::i32(state.iter().sum()))
}