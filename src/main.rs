mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

fn main() {
    build_runner!(run_days, day1, day2, day3, day4, day5, day6);
    run_days();
}
