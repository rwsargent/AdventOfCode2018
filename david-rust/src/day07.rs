use std::collections::HashMap;
use std::collections::HashSet;

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

pub fn get_execution_order(input: StringInput) -> PuzzleResult {
  let mut result = "".to_string();

  let (mut actions, dependencies) = get_actions_and_dependencies(input)?;

  let mut completed = HashSet::new();
  let mut ordered = actions.into_iter().collect::<Vec<_>>();
  ordered.sort();
  let size = ordered.len();

  let mut making_progress = true;
  while making_progress && completed.len() != size {
    let mut found_action = None;
    for (i, action) in ordered.iter().enumerate() {
      match dependencies.get(action) {
        Some(required) => {
          if required.iter().all(|i| completed.contains(i)) {
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
    match found_action {
      Some(idx) => {
        let action = ordered.remove(idx);
        result.push_str(&action);
        completed.insert(action);
      }
      _ => {}
    }
  }

  Ok(PuzzleSolution::String(result))
}

pub fn get_execution_time(input: StringInput, workers: usize, task_overhead: usize) -> PuzzleResult {
  if workers == 0 {
    return Err(Box::new(InvalidInput));
  }
  Ok(PuzzleSolution::usize(0))
}