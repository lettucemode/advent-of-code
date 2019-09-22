use super::common;
use std::cmp;

pub fn solve() -> (u32, i32, i32) {
    let input = common::get_input("./input/d3.txt");
    let input: i32 = input.parse().unwrap();
    (3, part_one(&input), part_two(&input))
}

fn part_one(input: &i32) -> i32 {
    let mut ring_no: i32 = 0;
    let mut side_length: i32 = 1;
    let mut progress: i32 = 0;
    let mut ring_size: i32;

    loop {
        ring_size = cmp::max(1, side_length * 4 - 4);
        progress += ring_size;
        if input < &progress {
            break;
        };
        ring_no += 1;
        side_length += 2;
    }

    let place: i32 = (input - (progress - ring_size)) % ring_size;
    let sweet_spots = vec![
        side_length / 2,
        side_length / 2 + side_length - 1,
        side_length / 2 + (side_length - 1) * 2,
        side_length / 2 + (side_length - 1) * 3,
    ];
    let dist_from_sweet_spot = sweet_spots.iter().fold(std::i32::MAX, |acc, &x| {
        if acc < (place - x).abs() {
            acc
        } else {
            (place - x).abs()
        }
    });

    ring_no + dist_from_sweet_spot
}

fn part_two(input: &i32) -> i32 {
    let mut grid = [[0u32; 500]; 500];
    grid[249][249] = 1;
    let mut location = Point { x: 250, y: 249 };
    let (nsum, nnum) = sum_and_num_neighbors(&grid, location);
    *input
}

struct Point {
    x: usize,
    y: usize,
}

fn sum_and_num_neighbors(grid: &[[u32; 500]; 500], point: Point) -> (u32, u32) {
    let mut total: u32 = 0;
    let mut num: u32 = 0;
    for i in (point.x - 1)..=(point.x + 1) {
        for j in (point.y - 1)..=(point.y + 1) {
            if grid[i][j] > 0 {
                num += 1;
                total += grid[i][j];
            }
        }
    }
    (total, num)
}
