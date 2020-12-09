use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub type Rules = HashMap<String, Vec<(usize, String)>>;

pub fn parse_input(input: &str) -> Rules {
    input.lines().map(get_next_rule).collect()
}

fn get_next_rule(line: &str) -> (String, Vec<(usize, String)>) {
    lazy_static! {
        // create this regex only the first time we try to use it
        // after that, re-use the existing one.
        static ref CONTENT_MATCHER: Regex = Regex::new(r"(\d+?) (\D+? \D+?) bags?[,.]").unwrap();
    }

    let (color, contents) =
        split_first(line, " bags contain ").expect("each line should contain a bag rule");

    let contents: Vec<(usize, String)> = CONTENT_MATCHER
        .captures_iter(contents)
        .map(|captures| {
            (
                (&captures[1])
                    .parse()
                    .expect("bag count should be usize-able"),
                (&captures[2]).to_string(),
            )
        })
        .collect();

    (color.to_string(), contents)
}

fn split_first<'a, 'b>(src: &'a str, pattern: &'b str) -> Option<(&'a str, &'a str)> {
    src.find(pattern)
        .map(|idx| (&src[0..idx], &src[(idx + pattern.len())..]))
}

pub fn compute_part1(rules: Rules) -> usize {
    rules
        .keys()
        .filter(|&key| contains_shiny_gold_bag(&rules, key))
        .count()
}

pub fn compute_part2(rules: Rules) -> usize {
    count_contents(&rules, "shiny gold")
}

fn contains_shiny_gold_bag(rules: &Rules, bag: &str) -> bool {
    rules
        .get(bag)
        .map(|contents| {
            contents.iter().any(|(_, bag_name)| {
                bag_name == "shiny gold" || contains_shiny_gold_bag(rules, bag_name)
            })
        })
        .unwrap_or(false)
}

fn count_contents(rules: &Rules, bag: &str) -> usize {
    rules
        .get(bag)
        .map(|contents| {
            contents
                .iter()
                .map(|(count, bag_name)| count * (count_contents(rules, bag_name) + 1))
                .sum()
        })
        .unwrap_or(1)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &'static str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EXAMPLE_INPUT_2: &'static str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn part1() {
        let rules = parse_input(EXAMPLE_INPUT_1);
        let solution = compute_part1(rules);
        assert_eq!(solution, 4);
    }

    #[test]
    fn part2() {
        let rules = parse_input(EXAMPLE_INPUT_2);
        let solution = compute_part2(rules);
        assert_eq!(solution, 126);
    }
}
