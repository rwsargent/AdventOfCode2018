use util::input::get_input;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
// 
// Part one - Index by guard, by minute
fn part_one(_path : &str) -> String {
    let mut input = get_input("inputs/day04.txt").as_strings();
    input.sort_unstable();
    best_minute(&input)
}

fn part_two(_path : &str) -> String {
    let mut input = get_input("inputs/day04.txt").as_strings();
    input.sort_unstable();
    most_minute(index_guards(&input))
}

fn most_minute(guards : HashMap<i32, Guard>) -> String {
    let mut most_guard_id = 0;
    let mut most_minute_amount = 0;
    let mut most_minute = 0i32;
    for (_id, guard) in &guards {
        let mut max = 0;
        for (idx, &minute) in guard.minutes.iter().enumerate() {
            if minute > guard.minutes[max]  {
                max = idx;
            }
        }
        if most_minute_amount < guard.minutes[max] {
            most_guard_id = guard.id;
            most_minute_amount = guard.minutes[max];
            most_minute = max as i32;
        }
    }
    return (most_guard_id * most_minute).to_string()
}

fn best_minute(input : &Vec<String>) -> String {
    let index = index_guards(input);
    let max_guard = index.values().fold(create_guard(0), |max_guard, guard| {
        if max_guard.total_minutes < guard.total_minutes {
            return Guard{id : guard.id, minutes : guard.minutes, total_minutes : guard.total_minutes};
        } else {
            return max_guard;
        }
    });
    let mut best = (0i32, 0i32);
    for (idx, min) in max_guard.minutes.iter().enumerate() {
        if min > &best.1 {
            best = (idx as i32, *min);
        }
    }
    (best.0 * max_guard.id).to_string()
}

fn index_guards(input : &Vec<String>) -> HashMap<i32, Guard> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:(\d{2}))\] (Guard|falls|wakes) (#(\d+))?").unwrap();
    }
    let mut guards = HashMap::new();
    let mut current_guard_id = 0;
    let mut sleep_min_start : i32 = 0;
    for line in input {
        let cap = RE.captures(&line).unwrap();
        match &cap[3] {
            "Guard" => {
                current_guard_id = cap[5].parse().unwrap();
            },
            "falls" => {
                sleep_min_start = cap[2].parse().unwrap();
            },
            "wakes" => {
                let end = cap[2].parse().unwrap_or(-1);
                let current_guard = guards.entry(current_guard_id)
                    .or_insert(create_guard(current_guard_id));
                for min in sleep_min_start..end {
                    current_guard.minutes[min as usize] += 1;
                }
                current_guard.total_minutes += end - sleep_min_start;
            },
            _ => continue,
        }
    }
    guards
}

fn create_guard(id : i32) -> Guard {
    return Guard {
        id : id,
        minutes : [0; 60],
        total_minutes : 0,
    }
}
pub fn run() {
    println!("Part one: {}", part_one(""));
    println!("Part two: {}", part_two(""));
}

struct Guard {
    id : i32,
    minutes : [i32; 60],
    total_minutes : i32,
}
impl fmt::Debug for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Guard{{ id: {}, total_minutes: {}}}", self.id, self.total_minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(best_minute(&get_input("inputs/test_day04.txt").as_strings()), String::from("240"));
        let guards = index_guards(&get_input("inputs/test_day04.txt").as_strings());
        assert_eq!(most_minute(guards), String::from("4455"));
    }

}
