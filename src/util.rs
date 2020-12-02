pub fn get_input(day: u32) -> Option<String> {
    let path = format!("inputs/day{}.txt", day);
    std::fs::read_to_string(path).ok()
}
