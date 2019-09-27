pub fn get_input(path: String) -> String {
    std::fs::read_to_string(path).unwrap()
}