use crate::util;
use itertools::Itertools;

pub fn run() {
    println!("day1");
    part1();
    part2();
}

fn part1() {
    let values = parse_input();
    let solution = compute_part1(values);
    println!("    part1: {}", solution);
}

fn part2() {
    let values = parse_input();
    let solution = compute_part2(values);
    println!("    part2: {}", solution);
}

fn compute_part1(values: Vec<u32>) -> u32 {
    let (a, b) = values
        .iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == 2020)
        .expect("didn't find any entries that sum to 2020");

    a * b
}

fn compute_part2(values: Vec<u32>) -> u32 {
    let (a, b, c) = values
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == 2020)
        .expect("didn't find any entries that sum to 2020");

    a * b * c
}

fn parse_input() -> Vec<u32> {
    util::get_input(1)
        .expect("missing input file for day 1")
        .lines()
        .map(|line| line.parse::<u32>().expect("couldn't parse line into u32"))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let values = vec![1721, 979, 366, 299, 675, 1456];
        let solution = compute_part1(values);
        assert_eq!(solution, 514579);
    }

    #[test]
    fn part2() {
        let values = vec![1721, 979, 366, 299, 675, 1456];
        let solution = compute_part2(values);
        assert_eq!(solution, 241861950);
    }
}
