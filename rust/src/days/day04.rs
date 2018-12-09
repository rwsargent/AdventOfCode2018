use std::collections::HashMap;
// 
// Part one - Index by guard, by minute
fn part_one(_path : &str) -> String {
    String::from("")
}

fn part_two(_path : &str) -> String {
    String::from("")
}

fn parse(input : &Vec<String>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (Guard|falls|wakes) (#\d+)?").unwrap();
    }
    let index = HashMap::new();
    let mut current_guard = -1;
    for line in input {
        let cap = RE.captures(&line).unwrap();
        match cap[2] {
            Some("Guard") => {
                current_guard = cap[3].parse().unwrap_or(current_guard);
                index.entry(current_guard)
                    .or_insert(create_guard(current_guard))
            },
            Some("falls") => {
            },
            Some("wakes") => {
            },
            None => continue;
        }
        guard.minutes[]
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
