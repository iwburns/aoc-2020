pub fn get_input(day: &str) -> String {
    let path = format!("inputs/{}.txt", day);
    std::fs::read_to_string(path).unwrap_or_else(|_| {
        panic!("missing input for {}", day);
    })
}

#[macro_export]
macro_rules! build_runner {
    ( $name: ident, $( $day_module: ident),* ) => {
        fn $name () {
            $(
                let day = stringify!($day_module);
                println!("{}:", day);

                let input = util::get_input(&day);

                let parsed = $day_module::parse_input(&input);
                let solution = $day_module::compute_part1(parsed);
                println!("    part1: {}", solution);

                let parsed = $day_module::parse_input(&input);
                let solution = $day_module::compute_part2(parsed);
                println!("    part2: {}", solution);
            )*
        }
    }
}
