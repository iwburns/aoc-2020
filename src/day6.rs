use itertools::Itertools;
use std::collections::HashSet;
use std::str::Lines;

pub fn parse_input(input: &str) -> Vec<Group> {
    input.lines().batching(get_next_group).collect()
}

fn get_next_group(lines: &mut Lines) -> Option<Group> {
    let mut lines = lines.peekable();

    lines.peek()?;

    let answers = lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    Some(Group { answers })
}

pub fn compute_part1(groups: Vec<Group>) -> usize {
    groups
        .iter()
        .map(|group| {
            get_unique_answers_for_group(group).len()
        })
        .sum()
}

pub fn compute_part2(groups: Vec<Group>) -> usize {
    groups
        .iter()
        .map(|group| {
            let unique_answers = get_unique_answers_for_group(group);

            let all_answered_by_whole_group = unique_answers
                .iter()
                .filter(|&c| {
                    all_in_group_answered_yes(group, *c)
                })
                .count();

            all_answered_by_whole_group
        })
        .sum()
}

fn get_unique_answers_for_group(group: &Group) -> HashSet<char> {
    group
        .answers
        .iter()
        .fold(HashSet::new(), |mut acc, curr_answers_list| {
            curr_answers_list.chars().for_each(|c| {
                acc.insert(c);
            });
            acc
        })
}

fn all_in_group_answered_yes(group: &Group, question: char) -> bool {
    group
        .answers
        .iter()
        .all(|curr_answers_list| curr_answers_list.contains(question))
}

pub struct Group {
    answers: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part1() {
        let groups = parse_input(EXAMPLE_INPUT);
        let solution = compute_part1(groups);
        assert_eq!(solution, 11);
    }

    #[test]
    fn part2() {
        let groups = parse_input(EXAMPLE_INPUT);
        let solution = compute_part2(groups);
        assert_eq!(solution, 6);
    }
}
