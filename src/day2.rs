use crate::util;
use std::convert::{TryFrom, TryInto};

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
            let left_index = entry.min - 1;
            let right_index = entry.max - 1;

            let (_, left) = entry
                .password
                .char_indices()
                .find(|&(i, _)| i == left_index)
                .expect("left index out of bounds");

            let (_, right) = entry
                .password
                .char_indices()
                .find(|&(i, _)| i == right_index)
                .expect("right index out of bounds");

            (right == entry.character) ^ (left == entry.character) // ^ is xor
        })
        .count()
}

struct Entry {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

#[derive(Copy, Clone, Debug)]
enum ParseError {
    MissingMin,
    MissingMax,
    MissingMinMax,
    MissingCharacter,
    MissingPassword,
    InvalidMin,
    InvalidMax,
}

impl TryFrom<&str> for Entry {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // example input lines:
        //
        //     1-3 a: abcde
        //     1-3 b: cdefg
        //     2-9 c: ccccccccc
        //
        // the first example will be used below as a reference

        // ["1-3", "a:", "abcde"]
        let mut parts = value.split_ascii_whitespace().take(3);

        let mut min_max = parts
            .next() // "1-3"
            .ok_or(ParseError::MissingMinMax)?
            .split('-') // ["1", "3"]
            .take(2);
        let min = min_max
            .next() // "1"
            .ok_or(ParseError::MissingMin)?
            .parse()
            .map_err(|_| ParseError::InvalidMin)?;
        let max = min_max
            .next() // "3"
            .ok_or(ParseError::MissingMax)?
            .parse()
            .map_err(|_| ParseError::InvalidMax)?;

        let character = parts
            .next() // "a:"
            .ok_or(ParseError::MissingCharacter)?
            .chars() // ["a", ":"]
            .take(1)
            .next()
            .ok_or(ParseError::MissingCharacter)?;

        let password = parts
            .next() // "abcde"
            .ok_or(ParseError::MissingPassword)?
            .to_string();

        let entry = Entry {
            min,
            max,
            character,
            password,
        };

        Ok(entry)
    }
}

fn parse_input() -> Vec<Entry> {
    util::get_input(2)
        .expect("missing input file for day 2")
        .lines()
        .map(|line| {
            line.try_into()
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
                line.try_into()
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
                line.try_into()
                    .expect("couldn't convert line to password entry")
            })
            .collect();

        let solution = compute_part2(entries);
        assert_eq!(solution, 1);
    }
}
