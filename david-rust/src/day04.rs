use std::collections::HashMap;
use std::iter;
use std::iter::FromIterator;

use regex::Regex;

use utils::*;

fn get_guards(input: StringInput) -> Result<HashMap<usize, Vec<usize>>> {
  let lines = input.lines();
  let mut events = Vec::with_capacity(lines.len());
  let time_re = Regex::new(r"\[(\d\d\d\d)-(\d\d)-(\d\d) (\d\d):(\d\d)]")?;
  let guard_re = Regex::new(r"Guard #(\d+)")?;
  let asleep_re = Regex::new("asleep")?;
  for line in lines {
    let caps = time_re.captures(line).ok_or(InvalidLine { line: line.to_string() })?;
    let time = Time {
      year: caps[1].parse()?,
      month: caps[2].parse()?,
      day: caps[3].parse()?,
      hour: caps[4].parse()?,
      minute: caps[5].parse()?,
    };
    let evt = match guard_re.captures(line) {
      Some(caps) => {
        EventType::Guard(caps[1].parse()?)
      }
      None => {
        if asleep_re.is_match(line) {
          EventType::Sleep
        } else {
          EventType::Wake
        }
      }
    };
    events.push(Event { time, evt });
  }
  events.sort();

  let mut guards = HashMap::new();
  let mut current_guard = 0;
  let mut fell_asleep_at = 0;
  for event in events.iter() {
    match event.evt {
      EventType::Guard(id) => {
        current_guard = id
      }
      EventType::Sleep => {
        fell_asleep_at = event.time.minute
      }
      EventType::Wake => {
        let hour = guards.entry(current_guard).or_insert_with(|| Vec::from_iter(iter::repeat(0).take(60)));
        for x in fell_asleep_at..event.time.minute {
          let minute = hour.get_mut(x).unwrap();
          *minute += 1;
        }
      }
    }
  }
  Ok(guards)
}

pub fn most_asleep_guard(input: StringInput) -> PuzzleResult {
  let guards = get_guards(input)?;

  let mut guards: Vec<_> = guards.into_iter().map(|(id, hour)| {
    let minutes_asleep = hour.iter().sum();
    let max = hour.iter().enumerate().max_by_key(|(i, x)| *x).map(|(i, x)| i).unwrap_or(61);
    Guard { id, minutes_asleep, most_asleep_minute: max }
  }).collect();

  let guard = guards.iter().max_by_key(|g| g.minutes_asleep).unwrap();

  Ok(PuzzleSolution::Day04(guard.id * guard.most_asleep_minute))
}

pub fn most_asleep_minute(input: StringInput) -> PuzzleResult {
  let guards = get_guards(input)?;

  let mut guards: Vec<_> = guards.into_iter().map(|(id, hour)| {
    let (minute, times_asleep) = hour.iter().enumerate().max_by_key(|(i, x)| *x).unwrap();
    Guard { id, minutes_asleep: *times_asleep, most_asleep_minute: minute }
  }).collect();

  let guard = guards.iter().max_by_key(|g| g.minutes_asleep).unwrap();

  Ok(PuzzleSolution::Day04(guard.id * guard.most_asleep_minute))
}

#[derive(Debug)]
struct Guard {
  id: usize,
  minutes_asleep: usize,
  most_asleep_minute: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
  time: Time,
  evt: EventType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
  Guard(usize),
  Wake,
  Sleep,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
  year: usize,
  month: usize,
  day: usize,
  hour: usize,
  minute: usize,
}
