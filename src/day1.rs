use crate::util;
use itertools::Itertools;

pub fn setup() -> Vec<u32> {
    let input = util::get_input(1).expect("missing input file for day 1");
    parse_input(&input)
}

pub fn compute_part1(values: Vec<u32>) -> u32 {
    let (a, b) = values
        .iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == 2020)
        .expect("didn't find any entries that sum to 2020");

    a * b
}

pub fn compute_part2(values: Vec<u32>) -> u32 {
    let (a, b, c) = values
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == 2020)
        .expect("didn't find any entries that sum to 2020");

    a * b * c
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().expect("couldn't parse line into u32"))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "\
        1721\n\
        979\n\
        366\n\
        299\n\
        675\n\
        1456\n\
    ";

    #[test]
    fn part1() {
        let values = parse_input(EXAMPLE_INPUT);
        let solution = compute_part1(values);
        assert_eq!(solution, 514579);
    }

    #[test]
    fn part2() {
        let values = parse_input(EXAMPLE_INPUT);
        let solution = compute_part2(values);
        assert_eq!(solution, 241861950);
    }
}
