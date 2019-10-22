use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub fn solve(input: String) -> (u32, u32) {
    let banks: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    (part_one(banks.clone()), part_two(banks.clone()))
}

fn part_one(mut banks: Vec<u32>) -> u32 {
    let mut cycles: u32 = 0;
    let mut past: Vec<u64> = Vec::new();
    loop {
        let mut dist = banks.iter().rev().max().unwrap().clone();
        let mut idx = banks.iter().position(|&x| x == dist).unwrap();
        banks[idx] = 0;
        while dist > 0 {
            idx = (idx + 1) % banks.len();
            banks[idx] = banks[idx] + 1;
            dist = dist - 1;
        }
        cycles = cycles + 1;
        let mut hasher = DefaultHasher::new();
        for i in 0..banks.len() {
            hasher.write_u32(banks[i]);
        }
        let hash = hasher.finish();
        if past.contains(&hash) {
            break;
        }
        past.push(hash);
    }
    cycles
}

fn part_two(mut banks: Vec<u32>) -> u32 {
    let mut past: Vec<u64> = Vec::new();
    loop {
        let mut dist = banks.iter().rev().max().unwrap().clone();
        let mut idx = banks.iter().position(|&x| x == dist).unwrap();
        banks[idx] = 0;
        while dist > 0 {
            idx = (idx + 1) % banks.len();
            banks[idx] = banks[idx] + 1;
            dist = dist - 1;
        }
        let mut hasher = DefaultHasher::new();
        for i in 0..banks.len() {
            hasher.write_u32(banks[i]);
        }
        let hash = hasher.finish();
        if past.contains(&hash) {
            return past.len() as u32 - past.iter().position(|&x| x == hash).unwrap() as u32;
        }
        past.push(hash);
    }
}
