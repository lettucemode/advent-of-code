use std::collections::VecDeque;
const ITERATIONS_P1: u32 = 2017;
const ITERATIONS_P2: u32 = 50000000;

pub fn solve(input: String) -> (u32, u32) {
    let step_size: usize = input.parse().unwrap();
    let mut spinlock: VecDeque<u32> = VecDeque::new();
    spinlock.push_back(0);

    // part 1
    for i in 1..=ITERATIONS_P1 {
        spinlock.rotate_left((step_size + 1) % spinlock.len());
        spinlock.push_front(i);
    }
    let p1_answer = spinlock[1];

    // part 2
    for i in ITERATIONS_P1 + 1..ITERATIONS_P2 {
        spinlock.rotate_left((step_size + 1) % spinlock.len());
        spinlock.push_front(i);
    }
    let p2_answer = spinlock[spinlock.iter().position(|&x| x == 0).unwrap() + 1];
    (p1_answer, p2_answer)
}
