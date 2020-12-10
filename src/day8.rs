use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.into()).collect()
}

pub fn compute_part1(mut instructions: Vec<Instruction>) -> i32 {
    let (_, acc) = run_program(&mut instructions);
    acc
}

pub fn compute_part2(instructions: Vec<Instruction>) -> i32 {
    let mut index_of_last_change = -1isize;

    loop {
        // reset to a blank slate
        let mut instructions = instructions.clone();

        // find the next instruction to try swapping
        let instruction = instructions
            .iter_mut()
            .enumerate()
            .find(|(index, instruction)| match instruction.op {
                Op::Jmp(_) => *index as isize > index_of_last_change,
                Op::Nop(_) => *index as isize > index_of_last_change,
                _ => false,
            });

        // swap it if we found one
        if let Some((index, instruction)) = instruction {
            let swapped = match instruction.op {
                Op::Jmp(arg) => Op::Nop(arg),
                Op::Nop(arg) => Op::Jmp(arg),
                _ => unreachable!(),
            };
            instruction.op = swapped;
            index_of_last_change = index as isize;
        } else {
            // couldn't find an operation to swap
            break;
        }

        let (terminated, acc) = run_program(&mut instructions);

        if terminated {
            return acc;
        }
    }

    0
}

fn run_program(instructions: &mut Vec<Instruction>) -> (bool, i32) {
    let mut acc = 0;
    let mut index: isize = 0;
    let mut terminated = false;

    loop {
        if index as usize == instructions.len() {
            terminated = true;
            break;
        }

        assert!(index >= 0 && (index as usize) < instructions.len());
        let next = &mut instructions[index as usize];

        if next.hits > 0 {
            break;
        }

        next.hits += 1;
        match next.op {
            Op::Acc(arg) => {
                acc += arg;
                index += 1;
            }
            Op::Jmp(arg) => {
                index += arg;
            }
            Op::Nop(_) => {
                index += 1;
            }
        }
    }

    (terminated, acc)
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Acc(i32),
    Jmp(isize),
    Nop(isize),
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    op: Op,
    hits: u32,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        lazy_static! {
            // create this regex only the first time we try to use it
            // after that, re-use the existing one.
            static ref OP_MATCHER: Regex = Regex::new(r"^(\D{3}) ([+-]?\d+?)$").unwrap();
        }

        let captures = OP_MATCHER.captures(input).expect("invalid instruction");
        let arg = (&captures[2])
            .parse()
            .expect("invalid instruction argument");

        let op = match &captures[1] {
            "acc" => Op::Acc(arg),
            "jmp" => Op::Jmp(arg as isize),
            "nop" => Op::Nop(arg as isize),
            _ => unreachable!(),
        };

        Instruction { op, hits: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn part1() {
        let instructions = parse_input(EXAMPLE_INPUT);
        let solution = compute_part1(instructions);
        assert_eq!(solution, 5);
    }

    #[test]
    fn part2() {
        let instructions = parse_input(EXAMPLE_INPUT);
        let solution = compute_part2(instructions);
        assert_eq!(solution, 8);
    }
}
