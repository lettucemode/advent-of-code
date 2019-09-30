pub fn solve(input: String) -> (u32, u32) {
    let jumps: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (part_one(jumps.clone()), part_two(jumps.clone()))
}

fn part_one(mut jumps: Vec<i32>) -> u32 {
    let mut moves: u32 = 0;
    let mut idx: i32 = 0;
    while (idx as usize) < jumps.len() {
        jumps[idx as usize] = jumps[idx as usize] + 1;
        idx = idx + jumps[idx as usize] - 1;
        moves = moves + 1;
    }
    moves
}

fn part_two(mut jumps: Vec<i32>) -> u32 {
    let mut moves: u32 = 0;
    let mut idx: i32 = 0;
    while (idx as usize) < jumps.len() {
        let val = jumps[idx as usize];
        if val < 3 {
            jumps[idx as usize] = val + 1;
        } else {
            jumps[idx as usize] = val - 1;
        }
        idx = idx + val;
        moves = moves + 1;
    }
    moves
}
