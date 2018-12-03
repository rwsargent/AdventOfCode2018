#![allow(dead_code)]
mod days;
mod util;

#[cfg(test)]
mod tests {
    use days::day01;
    use days::day02;
    
    #[test]
    fn day01_1() {
        let sum = day01::part_one();
        println!("{:?}", sum);
    }

    #[test]
    fn day01_2() {
        let freq = day01::part_two();
        println!("{:?}", freq);
    }

    #[test]
    fn day02_01() {
        println!("Par 1: {}", day02::part_one("inputs/day02.txt".to_string()));
        println!("Part 2: {}", day02::part_two("inputs/day02.txt".to_string()));
    }
}
