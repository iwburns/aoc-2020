mod day1;
mod day2;
mod day3;
mod util;

fn main() {
    build_runner!(run_days, day1, day2, day3);
    run_days();
}
