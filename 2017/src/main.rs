mod common;
mod d1;
mod d2;
use std::fmt::Display;
use std::time::Instant;

type PuzzleFunc<T> = fn() -> (u32, T, T);

fn run_puzzle<T: Display>(func: PuzzleFunc<T>) {
    let start = Instant::now();
    let (day, rp1, rp2) = func();
    let millis = start.elapsed().as_millis();
    let secs = millis / 1000;
    let msecs = ((millis as f32 / 1000.0) - (millis as f32 / 1000.0).trunc()) * 1000.0;
    println!(
        "Solution for Day {}:\n\t Part 1: {}\n\t Part 2: {}\n\t Time: {}s {}ms",
        &day, &rp1, &rp2, secs, msecs as u32
    );
}

fn main() {
    // Day 1
    run_puzzle(d1::solve);
    // Day 2
    run_puzzle(d2::solve);
}
