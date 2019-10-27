mod common;
mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
use std::fmt::Display;
use std::time::Instant;

type PuzzleFunc<T, U> = fn(String) -> (T, U);

fn run_puzzle<T: Display, U: Display>(day: u8, func: PuzzleFunc<T, U>) {
    let input = common::get_input(format!("./input/d{}.txt", day));
    let start = Instant::now();
    let (rp1, rp2) = func(input);
    let millis = start.elapsed().as_millis();
    let secs = millis / 1000;
    let msecs = ((millis as f32 / 1000.0) - (millis as f32 / 1000.0).trunc()) * 1000.0;
    println!(
        "Solution for Day {}:\n\t Part 1: {}\n\t Part 2: {}\n\t Time: {}s {}ms",
        &day, &rp1, &rp2, secs, msecs as u32
    );
}

fn main() {
    run_puzzle(1, d1::solve);
    run_puzzle(2, d2::solve);
    run_puzzle(3, d3::solve);
    run_puzzle(4, d4::solve);
    run_puzzle(5, d5::solve);
    run_puzzle(6, d6::solve);
    run_puzzle(7, d7::solve);
}
