use crate::util;
use itertools::Itertools;
use std::str::Lines;
use regex::Regex;

pub fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .batching(|iter| get_next_group(iter))
        .collect()
}

fn get_next_group(lines: &mut Lines) -> Option<String> {
    let mut lines = lines.peekable();

    lines.peek()?;

    let group = lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_whitespace())
        .flatten()
        .join(" ");

    Some(group)
}

pub fn compute_part1(passports: Vec<String>) -> usize {
    passports
        .iter()
        .map(|s| Passport::from(s.as_str()))
        .filter(|p| {
            p.byr.is_some()
                && p.iyr.is_some()
                && p.eyr.is_some()
                && p.hgt.is_some()
                && p.hcl.is_some()
                && p.ecl.is_some()
                && p.pid.is_some()
        })
        .count()
}

pub fn compute_part2(passports: Vec<String>) -> usize {
    let hair = Regex::new("^#([0-9a-f]){6}$").unwrap();
    let eye = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let passport_id = Regex::new("^\\d{9}$").unwrap();

    passports
        .iter()
        .map(|s| Passport::from(s.as_str()))
        .filter(|p| {
            p.byr
                .and_then(|byr| match byr >= 1920 && byr <= 2002 {
                    true => Some(()),
                    false => None,
                })
                .and(p.iyr.and_then(|iyr| match iyr >= 2010 && iyr <= 2020 {
                    true => Some(()),
                    false => None,
                }))
                .and(p.eyr.and_then(|eyr| match eyr >= 2020 && eyr <= 2030 {
                    true => Some(()),
                    false => None,
                }))
                .and(p.hgt.and_then(|hgt| match hgt {
                    Height::In(i) if i >= 59 && i <= 76 => Some(()),
                    Height::Cm(c) if c >= 150 && c <= 193 => Some(()),
                    _ => None,
                }))
                .and(p.hcl.and_then(|hcl| match hair.is_match(hcl) {
                    true => Some(()),
                    false => None,
                }))
                .and(p.ecl.and_then(|ecl| match eye.is_match(ecl) {
                    true => Some(()),
                    false => None,
                }))
                .and(p.pid.and_then(|pid| match passport_id.is_match(pid) {
                    true => Some(()),
                    false => None,
                }))
                .is_some()
        })
        .count()
}

#[derive(Debug)]
struct Passport<'a> {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<Height>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(input: &'a str) -> Self {
        let parts = input.split_whitespace();

        let find = |target: &'a str| -> Option<&'a str> {
            parts.clone().find_map(|s| match s.starts_with(target) {
                true => s.split(':').nth(1),
                false => None,
            })
        };

        Passport {
            byr: find("byr").and_then(|value| value.parse().ok()),
            iyr: find("iyr").and_then(|value| value.parse().ok()),
            eyr: find("eyr").and_then(|value| value.parse().ok()),
            hgt: find("hgt").map(|value| value.into()),
            hcl: find("hcl"),
            ecl: find("ecl"),
            pid: find("pid"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Height {
    In(u32),
    Cm(u32),
}

impl From<&str> for Height {
    fn from(input: &str) -> Self {
        match input.ends_with("in") {
            true => Height::In(
                input
                    .trim_end_matches("in")
                    .parse()
                    .expect("invalid height inches"),
            ),
            false => Height::Cm(
                input
                    .trim_end_matches("cm")
                    .parse()
                    .expect("invalid height centimeters"),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &'static str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
    ";

    const EXAMPLE_INPUT_2: &'static str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    ";

    #[test]
    fn part1() {
        let passports = parse_input(EXAMPLE_INPUT_1);
        let solution = compute_part1(passports);
        assert_eq!(solution, 2);
    }

    #[test]
    fn part2() {
        let passports = parse_input(EXAMPLE_INPUT_2);
        let solution = compute_part2(passports);
        assert_eq!(solution, 4);
    }
}
