use crate::util;

pub fn parse_input(input: &str) -> Grid {
    let rows = input.lines().map(|line| line.into()).collect();
    Grid { rows }
}

pub fn compute_part1(grid: Grid) -> usize {
    count_trees(&grid, (3, 1))
}

pub fn compute_part2(grid: Grid) -> usize {
    #[rustfmt::skip]
    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    slopes
        .iter()
        .map(|&slope| count_trees(&grid, slope))
        .product()
}

fn count_trees(grid: &Grid, step: (usize, usize)) -> usize {
    grid.iter((0, 0), step)
        .filter(|slot| matches!(slot, Slot::Tree))
        .count()
}

#[derive(Copy, Clone)]
enum Slot {
    Tree,
    Open,
}

#[derive(Clone)]
struct Row {
    slots: Vec<Slot>,
}

impl From<&str> for Row {
    fn from(input: &str) -> Self {
        let slots = input
            .chars()
            .map(|c| match c {
                '.' => Slot::Open,
                '#' => Slot::Tree,
                _ => panic!("invalid input data"),
            })
            .collect();

        Row { slots }
    }
}

pub struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn iter(&self, start: (usize, usize), step: (usize, usize)) -> impl Iterator<Item = Slot> {
        let row_width = self
            .rows
            .get(0)
            .expect("grid should have at least one row")
            .slots
            .len();

        GridIter {
            rows: self.rows.clone(),
            current_x: start.0,
            current_y: start.1,
            step_x: step.0,
            step_y: step.1,
            row_width,
        }
    }
}

struct GridIter {
    rows: Vec<Row>,
    current_x: usize,
    current_y: usize,
    step_x: usize,
    step_y: usize,
    row_width: usize,
}

impl Iterator for GridIter {
    type Item = Slot;

    fn next(&mut self) -> Option<Self::Item> {
        // wrap horizontal axis as the problem describes, but
        // don't wrap the vertical axis or we'll loop forever.
        let next_x = (self.current_x + self.step_x) % self.row_width;
        let next_y = self.current_y + self.step_y;

        self.rows
            .get(self.current_y)
            .and_then(|row| row.slots.get(self.current_x))
            .cloned()
            .map(|slot| {
                self.current_x = next_x;
                self.current_y = next_y;
                slot
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "\
        ..##.......\n\
        #...#...#..\n\
        .#....#..#.\n\
        ..#.#...#.#\n\
        .#...##..#.\n\
        ..#.##.....\n\
        .#.#.#....#\n\
        .#........#\n\
        #.##...#...\n\
        #...##....#\n\
        .#..#...#.#\n\
    ";

    #[test]
    fn part1() {
        let grid = parse_input(EXAMPLE_INPUT);
        let solution = compute_part1(grid);
        assert_eq!(solution, 7);
    }

    #[test]
    fn part2() {
        let grid = parse_input(EXAMPLE_INPUT);
        let solution = compute_part2(grid);
        assert_eq!(solution, 336);
    }
}
