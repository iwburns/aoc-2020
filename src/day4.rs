use itertools::Itertools;
use regex::Regex;
use std::str::Lines;

pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().batching(get_next_group).collect()
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
        .filter(|p| {
            p.contains("byr")
                && p.contains("iyr")
                && p.contains("eyr")
                && p.contains("hgt")
                && p.contains("hcl")
                && p.contains("ecl")
                && p.contains("pid")
        })
        .count()
}

pub fn compute_part2(passports: Vec<String>) -> usize {
    let match_byr = Regex::new(r"byr:(\d{4})\b").unwrap();
    let match_iyr = Regex::new(r"iyr:(\d{4})\b").unwrap();
    let match_eyr = Regex::new(r"eyr:(\d{4})\b").unwrap();

    let match_hgt_in = Regex::new(r"hgt:(\d+?)in\b").unwrap();
    let match_hgt_cm = Regex::new(r"hgt:(\d+?)cm\b").unwrap();

    let match_hcl = Regex::new(r"hcl:#[0-9a-f]{6}\b").unwrap();
    let match_ecl = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let match_pid = Regex::new(r"pid:\d{9}\b").unwrap();

    passports
        .iter()
        .filter_map(|p| {
            let byr = match_byr
                .captures(p)
                .and_then(|caps| caps[1].parse().ok())?;
            let iyr = match_iyr
                .captures(p)
                .and_then(|caps| caps[1].parse().ok())?;
            let eyr = match_eyr
                .captures(p)
                .and_then(|caps| caps[1].parse().ok())?;

            let hgt_in = match_hgt_in
                .captures(p)
                .and_then(|caps| caps[1].parse().ok());
            let hgt_cm = match_hgt_cm
                .captures(p)
                .and_then(|caps| caps[1].parse().ok());

            let hcl_found = match_hcl.is_match(p);
            let ecl_found = match_ecl.is_match(p);
            let pid_found = match_pid.is_match(p);

            match is_valid(byr, iyr, eyr, hgt_in, hgt_cm) && hcl_found && ecl_found && pid_found {
                true => Some(()),
                false => None,
            }
        })
        .count()
}

fn is_valid(byr: u32, iyr: u32, eyr: u32, hgt_in: Option<u32>, hgt_cm: Option<u32>) -> bool {
    let in_valid = hgt_in.map(|i| i >= 59 && i <= 76);
    let cm_valid = hgt_cm.map(|c| c >= 150 && c <= 193);
    let height_valid = in_valid.or(cm_valid).unwrap_or(false);

    (byr >= 1920 && byr <= 2002)
        && (iyr >= 2010 && iyr <= 2020)
        && (eyr >= 2020 && eyr <= 2030)
        && height_valid
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
