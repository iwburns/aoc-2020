use parse_display::{Display as PDisplay, FromStr as PFromStr};

pub fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .expect("couldn't convert line to password entry")
        })
        .collect()
}

pub fn compute_part1(entries: Vec<Entry>) -> usize {
    entries
        .iter()
        .filter(|entry| {
            let occurrences = entry.password.matches(entry.character).count();
            occurrences >= entry.min && occurrences <= entry.max
        })
        .count()
}

pub fn compute_part2(entries: Vec<Entry>) -> usize {
    entries
        .iter()
        .filter(|entry| {
            let letter = Some(entry.character);

            let left_match = entry.password.chars().nth(entry.min - 1) == letter;
            let right_match = entry.password.chars().nth(entry.max - 1) == letter;

            left_match ^ right_match // ^ is xor
        })
        .count()
}

#[derive(PDisplay, PFromStr, PartialEq, Debug)]
#[display("{min}-{max} {character}: {password}")]
pub struct Entry {
    min: usize,
    max: usize,
    character: char,
    password: String,
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
        let entries = parse_input(EXAMPLE_INPUT);
        let solution = compute_part1(entries);
        assert_eq!(solution, 2);
    }

    #[test]
    fn part2() {
        let entries = parse_input(EXAMPLE_INPUT);
        let solution = compute_part2(entries);
        assert_eq!(solution, 1);
    }
}
