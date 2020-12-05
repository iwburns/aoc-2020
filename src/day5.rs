use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<Seat> {
    input.lines().map(|line| line.into()).collect()
}

pub fn compute_part1(seats: Vec<Seat>) -> u32 {
    seats
        .iter()
        .map(|seat| seat.id())
        .max()
        .expect("we have at least one seat")
}

pub fn compute_part2(seats: Vec<Seat>) -> u32 {
    let sorted_ids: Vec<u32> = seats.iter().map(|seat| seat.id()).sorted().collect();

    sorted_ids
        .windows(2)
        .find(|pair| pair[0] + 2 == pair[1])
        .map(|pair| pair[0] + 1)
        .expect("couldn't find gap in seats")
}

pub struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row as u32 * 8 + self.col as u32
    }
}

impl From<&str> for Seat {
    fn from(seat: &str) -> Self {
        let binary: String = seat
            .chars()
            .map(|c| match c {
                'F' | 'L' => '0',
                'B' | 'R' => '1',
                _ => panic!("invalid seat"),
            })
            .collect();

        // unwraps are safe, b/c we know this is a binary string if we didn't panic above
        let row = u8::from_str_radix(&binary[0..7], 2).unwrap();
        let col = u8::from_str_radix(&binary[7..10], 2).unwrap();

        Seat { row, col }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "\
        BFFFBBFRRR\n\
        FFFBBBFRRR\n\
        BBFFBBFRLL\n\
    ";

    #[test]
    fn part1() {
        let seats = parse_input(EXAMPLE_INPUT);
        let solution = compute_part1(seats);
        assert_eq!(solution, 820);
    }
}
