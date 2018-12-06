use util::input::get_input;
use std::collections::HashMap;

pub fn part_one(path : String) -> i32 {
    let input = get_input(&path).as_strings();
    let mut twos = 0;
    let mut threes = 0;
    for word in input {
        let letter_counts = get_histogram(&word);
        if count_twos(&letter_counts) > 0 {
            twos += 1;
        }
        if count_threes(&letter_counts) > 0 {
            threes += 1;
        }
    }
    twos * threes
}

pub fn part_two(path : String) -> String {
    let input = get_input(&path).as_strings();
    for left in &input {
        for right in &input {
            let idxs = edit_index(&left, &right);
            if idxs.len() == 1 {
                return remove_letter(&left, idxs[0]);
            }
        }
    }
    return "".to_string()
}

pub fn run() {
    println!("Part 1: {}", part_one("inputs/day02.txt".to_string()));
    println!("Part 2: {}", part_two("inputs/day02.txt".to_string()));
}

fn get_histogram(word : &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    for letter in word.chars() {
        counts.entry(letter)
            .and_modify(|count| { *count += 1})
            .or_insert(1);
    }
    return counts
}

fn count_twos(counts : &HashMap<char, i32>) -> usize {
    counts.values().filter(|&val| *val == 2).count()
}

fn count_threes(counts : &HashMap<char, i32>) -> usize {
    counts.values().filter(|&val| *val == 3).count()
}

fn remove_letter(word : &str, idx : usize) -> String {
    let mut result = String::new();
    for ent in word.char_indices() {
        if ent.0 == idx {
            continue;
        }
        result.push(ent.1);
    }
    result
}

fn edit_index(left : &str, right : &str) ->  Vec<usize> {
    let mut idices = Vec::new();
    let mut left_itr = left.chars();
    let mut right_itr = right.chars();
    for idx in  0..left.len() {
        if left_itr.next() != right_itr.next() {
            idices.push(idx)
        }
    }
    idices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_one("inputs/day02_test.txt".to_string()), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_two("inputs/day02_test_2.txt".to_string()), "fgij")
    }
}
