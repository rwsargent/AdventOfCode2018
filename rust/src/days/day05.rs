use std::cmp::min;
use std::usize::MAX;
use util::input::get_input;

fn polymerize(input: Vec<char>) -> Vec<char> {
    let mut tail: Vec<char> = Vec::with_capacity(input.len());
    for letter in input {
        let left = tail.pop();
        match left {
            Some(c) => {
                if (c as u8) ^ (letter as u8) != 32 {
                    tail.push(c);
                    tail.push(letter);
                }
            }
            None => tail.push(letter),
        }
    }
    return tail;
}

fn polymerize_with_skip(input: &Vec<char>, skip: char) -> Vec<char> {
    let mut tail: Vec<char> = Vec::with_capacity(input.len());
    for letter in input {
        if letter.eq_ignore_ascii_case(&skip) {
            continue;
        }
        let left = tail.pop();
        match left {
            Some(c) => {
                if (c as u8) ^ (*letter as u8) != 32 {
                    tail.push(c);
                    tail.push(*letter);
                }
            }
            None => tail.push(*letter),
        }
    }
    return tail;
}

fn part_one() -> usize {
    let input = get_input("inputs/day05.txt").as_chars();
    let output = polymerize(input);
    output.len()
}
fn part_two() -> usize {
    let mut min_len = MAX;
    let input = get_input("inputs/day05.txt").as_chars();
    for skip in ('a' as u8)..('z' as u8) {
        min_len = min(polymerize_with_skip(&input, skip as char).len(), min_len);
    }
    min_len
}

pub fn run() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_on_examples() {
        let empty: Vec<char> = Vec::new();
        assert_eq!(polymerize(vec!['a', 'A']), empty);
        assert_eq!(polymerize(vec!['a', 'b', 'B', 'A']), empty);
        assert_eq!(
            polymerize(vec!['a', 'b', 'A', 'B']),
            vec!['a', 'b', 'A', 'B']
        );
        assert_eq!(
            polymerize("dabAcCaCBAcCcaDA".chars().collect()),
            "dabCBAcaDA".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            polymerize("aabAAB".chars().collect()),
            ("aabAAB".chars().collect::<Vec<char>>())
        );
    }

    #[test]
    fn skip_tests() {
        let polymer = "dabAcCaCBAcCcaDA".chars().collect();
        assert_eq!(
            polymerize_with_skip(&polymer, 'a'),
            "dbCBcD".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            polymerize_with_skip(&polymer, 'b'),
            "daCAcaDA".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            polymerize_with_skip(&polymer, 'c'),
            "daDA".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            polymerize_with_skip(&polymer, 'd'),
            "abCBAc".chars().collect::<Vec<char>>()
        );
    }
}
