use crate::util;
use parse_display::{Display as PDisplay, FromStr as PFromStr};

pub fn run() {
    println!("day2");
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

fn compute_part1(entries: Vec<Entry>) -> usize {
    entries
        .iter()
        .filter(|entry| {
            let occurrences = entry.password.matches(entry.character).count();
            occurrences >= entry.min && occurrences <= entry.max
        })
        .count()
}

fn compute_part2(entries: Vec<Entry>) -> usize {
    entries
        .iter()
        .filter(|entry| {
            let letter = Some(entry.character);
            let left_index = entry.min - 1;
            let right_index = entry.max - 1;

            let left_match = entry.password.chars().nth(left_index) == letter;
            let right_match = entry.password.chars().nth(right_index) == letter;

            left_match ^ right_match // ^ is xor
        })
        .count()
}

#[derive(PDisplay, PFromStr, PartialEq, Debug)]
#[display("{min}-{max} {character}: {password}")]
struct Entry {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

fn parse_input() -> Vec<Entry> {
    util::get_input(2)
        .expect("missing input file for day 2")
        .lines()
        .map(|line| {
            line.parse()
                .expect("couldn't convert line to password entry")
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "\
        1-3 a: abcde\n\
        1-3 b: cdefg\n\
        2-9 c: ccccccccc\
    ";

    #[test]
    fn part1() {
        let entries = EXAMPLE_INPUT
            .lines()
            .map(|line| {
                line.parse()
                    .expect("couldn't convert line to password entry")
            })
            .collect();

        let solution = compute_part1(entries);
        assert_eq!(solution, 2);
    }

    #[test]
    fn part2() {
        let entries = EXAMPLE_INPUT
            .lines()
            .map(|line| {
                line.parse()
                    .expect("couldn't convert line to password entry")
            })
            .collect();

        let solution = compute_part2(entries);
        assert_eq!(solution, 1);
    }
}
