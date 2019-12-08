const ITERATIONS: u32 = 2017;

pub fn solve(input: String) -> (u32, u32) {
    let step_size: usize = input.parse().unwrap();
    let mut list: Vec<u32> = vec![0];
    let mut current_position: usize = 0;
    for i in 1..=ITERATIONS {
        current_position = ((current_position + step_size) % list.len()) + 1;
        list.insert(current_position, i);
    }
    (list[current_position + 1], 0)
}
