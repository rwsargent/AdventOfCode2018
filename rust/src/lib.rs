mod days;
mod util;

#[cfg(test)]
mod tests {
    use days::day01;
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
}