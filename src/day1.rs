use itertools::Itertools;

pub fn part1() {
    let values = get_enumerated_values();

    let (a, b) = values
        .iter()
        .cartesian_product(values.iter())
        .filter(|((i, _), (j, _))| i != j)
        .map(|((_, a), (_, b))| (a, b))
        .filter(|(a, b)| *a + *b == 2020)
        .next()
        .expect("didn't find any entries that sum to 2020");

    println!("day1 part1: {}", a * b);
}

pub fn part2() {
    let values = get_enumerated_values();

    let (a, b, c) = values.iter()
        .cartesian_product(values.iter())
        .cartesian_product(values.iter())
        .filter(|(((i, _), (j, _)), (k, _))| i != j && i != k && j != k)
        .map(|(((_, a), (_, b)), (_, c))| (a, b, c))
        .filter(|(a, b, c)| *a + *b + *c == 2020)
        .next()
        .expect("didn't find any entries that sum to 2020");

    println!("day1 part2: {}", a * b * c);
}

fn get_enumerated_values() -> Vec<(usize, u32)> {
    get_input()
        .lines()
        .map(|line| line.parse::<u32>().expect("couldn't parse line into u32"))
        .enumerate()
        .collect()
}

fn get_input() -> String {
    std::fs::read_to_string("inputs/day1.txt")
        .expect("missing input file for day 1")
}
