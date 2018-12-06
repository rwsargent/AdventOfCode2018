use util::input::get_input;
use util::cursor::Cursor;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn part_one(path : &str) -> String {
    let instructions = parse(&get_input(path).as_strings());
    let board = fill_board(&instructions);
    count_overlap(&board).to_string()
}

fn part_two(path : &str) -> String {
    let instructions = parse(&get_input(path).as_strings());
    match find_solo(&instructions) {
        Some(id) => id.to_string(),
        None => String::from(""),
    }
}

fn parse(lines : &Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
        instructions.push(create_instruction(&line))
    }
    instructions
}

fn fill_board(instructions : &Vec<Instruction>) -> HashMap<(i32, i32), Vec<u32>> {
    let mut board = HashMap::new();
    for inst in instructions {
        for x in 0..inst.range_x {
            for y in 0..inst.range_y {
                board.entry((inst.coord.x + x, inst.coord.y + y))
                    .and_modify(|ids: &mut Vec<u32>| {
                        ids.push(inst.id);
                    })
                    .or_insert(vec![inst.id]);
            }
        }
    }
    board
}
fn find_solo(instructions : &Vec<Instruction>) -> Option<u32> {
    let board = fill_board(&instructions);
    let dupes = find_dupes(&board);
    for insts in instructions {
        if !dupes.contains(&insts.id) {
            return Some(insts.id)
        }
    }
    None
}
fn count_overlap(board : &HashMap<(i32, i32), Vec<u32>>) -> usize {
    board.values()
        .filter(|&ids| ids.len() >= 2)
        .count()
}

fn find_dupes(board : &HashMap<(i32, i32), Vec<u32>>) -> HashSet<u32> {
    let mut dupes = HashSet::new();
    for ids in board.values().filter(|&ids| ids.len() != 1) {
        for id in ids {
            dupes.insert(*id);
        }
    }
    dupes
}

fn create_instruction(line : &String) -> Instruction {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let cap =  RE.captures(&line).unwrap();
    Instruction {
        id : cap[1].parse().unwrap(),
        coord : Cursor {
            x : cap[2].parse().unwrap(),
            y : cap[3].parse().unwrap(),
        },
        range_x : cap[4].parse().unwrap(),
        range_y : cap[5].parse().unwrap(),
    }
}

pub fn run() {
    println!("Part one: {}", part_one("inputs/day03.txt"));
    println!("Part one: {}", part_two("inputs/day03.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![String::from("#1 @ 1,3: 4x4"),
                         String::from("#2 @ 3,1: 4x4"),
                         String::from("#3 @ 5,5: 2x2")];
        let insts = parse(&input);
        assert_eq!(4, count_overlap(&fill_board(&insts)));
    }

    #[test]
    fn test_part2() {
        let input = vec![String::from("#1 @ 1,3: 4x4"),
                         String::from("#2 @ 3,1: 4x4"),
                         String::from("#3 @ 5,5: 2x2")];
        let insts = parse(&input);
        assert_eq!(Some(3), find_solo(&insts));
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Instruction {
    id : u32,
    coord : Cursor,
    range_x : i32,
    range_y : i32,
}
