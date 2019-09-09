use super::common;

pub fn solve() -> (u32, u32, u32) {
    let input = common::get_input("./input/d1.txt");

    // Part 1
    let mut total_p1: u32 = 0;
    let mut last_ch: char = 'k';
    let mut modified_input = input.clone();
    modified_input.push(input.chars().nth(0).unwrap());
    for ch in modified_input.chars() {
        if ch == last_ch {
            total_p1 += last_ch.to_digit(10).unwrap();
        }
        last_ch = ch;
    }

    // Part 2
    let mut total_p2: u32 = 0;
    for ch in input.char_indices() {
        let next_idx = (ch.0 + input.len() / 2) % input.len();
        let next_ch = input.chars().nth(next_idx).unwrap();
        if ch.1 == next_ch {
            total_p2 += ch.1.to_digit(10).unwrap();
        }
    }

    (1, total_p1, total_p2)
}
