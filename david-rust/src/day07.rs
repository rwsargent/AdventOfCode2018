use std::collections::HashSet;
use std::iter;

use regex::Regex;

use utils::*;

fn get_actions_and_dependencies(input: StringInput) -> Result<(HashSet<String>, MultiMap<String, String>)> {
  let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin\.")?;
  let mut dependencies = MultiMap::new();
  let mut actions = HashSet::new();
  for cap in re.captures_iter(&input.content) {
    dependencies.insert(cap[2].to_string(), cap[1].to_string());
    actions.insert(cap[1].to_string());
    actions.insert(cap[2].to_string());
  }
  Ok((actions, dependencies))
}

fn get_next_task(ordered_actions: &Vec<String>, unfinished_actions: &HashSet<String>, dependencies: &MultiMap<String, String>) -> Option<usize> {
  let mut found_action = None;
  for (i, action) in ordered_actions.iter().enumerate() {
    match dependencies.get(action) {
      Some(required) => {
        if required.iter().all(|i| !unfinished_actions.contains(i)) {
          found_action = Some(i);
          break;
        }
      }
      None => {
        found_action = Some(i);
        break;
      }
    }
  }
  found_action
}

pub fn get_execution_order(input: StringInput) -> PuzzleResult {
  let mut result = "".to_string();

  let (mut actions, dependencies) = get_actions_and_dependencies(input)?;

  let mut ordered = actions.clone().into_iter().collect::<Vec<_>>();
  ordered.sort();

  let mut making_progress = true;
  while making_progress && actions.len() != 0 {
    making_progress = false;
    match get_next_task(&ordered, &actions, &dependencies) {
      Some(idx) => {
        let action = ordered.remove(idx);
        result.push_str(&action);
        actions.remove(&action);
        making_progress = true;
      }
      _ => {}
    }
  }

  Ok(PuzzleSolution::String(result))
}

#[derive(Debug)]
struct Worker {
  finishes_at: usize,
  current_task: Option<String>,
}

fn get_time_for_task(task: &String) -> Result<usize> {
  let r = task.chars().next().map(|c| {
    ((c as u8) - ('A' as u8) + 1) as usize
  });
  r.ok_or(Box::new(InvalidInput))
}

pub fn get_execution_time(input: StringInput, workers: usize, task_overhead: usize) -> PuzzleResult {
  if workers == 0 {
    return Err(Box::new(InvalidInput));
  }
  let mut workers = iter::repeat_with(|| Worker { finishes_at: 0, current_task: None }).take(workers).collect::<Vec<_>>();
  let (mut actions, dependencies) = get_actions_and_dependencies(input)?;
  let mut ordered = actions.clone().into_iter().collect::<Vec<_>>();
  let mut time = 0;
  while !actions.is_empty() {
    let mut next_time = usize::max_value();
    for worker in &mut workers {
      let mut next_task = None;
      let mut completed_task = false;
      match worker.current_task {
        Some(ref task) => {
          if worker.finishes_at == time {
            actions.remove(task);
            next_task = get_next_task(&ordered, &actions, &dependencies);
            completed_task = true;
          } else if worker.finishes_at < next_time {
            next_time = worker.finishes_at;
          }
        }
        None => {
          next_task = get_next_task(&ordered, &actions, &dependencies);
        }
      }
      match next_task {
        Some(task) => {
          let task = ordered.remove(task);
          worker.finishes_at = time + get_time_for_task(&task)? + task_overhead;
          worker.current_task = Some(task);
          if worker.finishes_at < next_time {
            next_time = worker.finishes_at;
          }
        }
        None => {
          if completed_task {
            worker.current_task = None;
          }
        }
      }
    }
    if next_time != usize::max_value() {
      time = next_time;
    }
  }
  Ok(PuzzleSolution::usize(time))
}