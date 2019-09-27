use super::common;

pub fn solve() -> (u32, i32, i32) {
    let day_number = 4;
    let input = common::get_input(format!("./input/d{}.txt", day_number));
    let input: i32 = input.parse().unwrap();
    (day_number, part_one(input), part_two(input))
}

fn part_one(input: i32) -> i32 {
    input
}

fn part_two(input: i32) -> i32 {
    input
}

