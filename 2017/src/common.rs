pub fn get_input(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}