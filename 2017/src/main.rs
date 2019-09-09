#![allow(dead_code)]
mod common;
mod d1;
mod d2;
use std::time::Instant;

type PuzzleFuncU32 = fn() -> (u32, u32);

fn solve(func: PuzzleFuncU32, day: u32) {
    let start = Instant::now();
    let (rp1, rp2) = func();
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
    solve(d1::solve, 1);
    // Day 2
    solve(d2::solve, 2);
}
