pub fn get_input(day: u32) -> Option<String> {
    let path = format!("inputs/day{}.txt", day);
    std::fs::read_to_string(path).ok()
}

#[macro_export]
macro_rules! build_runner {
    ( $name: ident, $( $day_module: ident),* ) => {
        fn $name () {
            $(
                println!("{}:", stringify!($day_module));

                let solution = $day_module::compute_part1($day_module::setup());
                println!("    part1: {}", solution);

                let solution = $day_module::compute_part2($day_module::setup());
                println!("    part2: {}", solution);
            )*
        }
    }
}
