use super::common;

pub fn solve() -> (u32, u32, u32) {
    let input = common::get_input("./input/d2.txt");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    // Part 1
    let mut diff_sum: u32 = 0;
    for line in &lines {
        let nums: Vec<u32> = line
            .split_ascii_whitespace()
            .map(|item: &str| item.parse().unwrap())
            .collect();
        let min: u32 = nums
            .iter()
            .fold(std::u32::MAX, |acc, &x| if acc < x { acc } else { x });
        let max: u32 = nums
            .iter()
            .fold(std::u32::MIN, |acc, &x| if acc > x { acc } else { x });
        diff_sum += max - min;
    }

    // Part 2
    let mut divisible_sum: u32 = 0;
    for line in &lines {
        let mut found = false;
        let nums: Vec<u32> = line
            .split_ascii_whitespace()
            .map(|item: &str| item.parse().unwrap())
            .collect();
        for i in 0..nums.len() {
            for k in (i + 1)..nums.len() {
                let result_1: f32 = (nums[i] as f32) / (nums[k] as f32);
                if result_1 == result_1.trunc() {
                    divisible_sum += result_1 as u32;
                    found = true;
                    break;
                }
                let result_2: f32 = (nums[k] as f32) / (nums[i] as f32);
                if result_2 == result_2.trunc() {
                    divisible_sum += result_2 as u32;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }

    (2, diff_sum, divisible_sum)
}
