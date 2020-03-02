use std::collections::HashMap;
use std::mem;

const ON: char = '#';
const OFF: char = '.';
const P1_ITERATIONS: u32 = 5;

pub fn solve(input: String) -> (u32, u32) {
    let pattern: Vec<Vec<char>> = starting_pattern();
    let rules: HashMap<String, String> = build_rules(input);
    let pattern_str = grid_to_str(&pattern);
    let next_pattern = str_to_grid(&rules[&pattern_str]);
    for _ in 0..P1_ITERATIONS {}
    (pixels_on(&next_pattern), next_pattern.len() as u32)
}

fn next_pattern(grid: Vec<Vec<char>>, rules: &HashMap<String, String>) -> Vec<Vec<char>> {
    let mut sub_length: usize = 3;
    if grid.len() < 4 {
        let grid_str = grid_to_str(&grid);
        return str_to_grid(&rules[&grid_str]);
    } else if grid.len() % 2 == 0 {
        sub_length = 2;
    }

    let mut rows_iter = grid.chunks_exact(sub_length);
    for rowset in rows_iter {}
    for tlc in 0..grid.len() / sub_length {}

    vec![vec![]]
}

fn build_rules(input: String) -> HashMap<String, String> {
    let mut rules: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        // insert orig rule
        let things: Vec<&str> = line.split_terminator(" => ").collect();
        let rule = things[0];
        let output = things[1];
        rules.insert(rule.to_owned(), output.to_owned());

        // insert rotations
        let mut temp_grid = str_to_grid(rule);
        rotate_cw(&mut temp_grid);
        let rule = grid_to_str(&temp_grid);
        rules.insert(rule.to_owned(), output.to_owned());
        rotate_cw(&mut temp_grid);
        let rule = grid_to_str(&temp_grid);
        rules.insert(rule.to_owned(), output.to_owned());
        rotate_cw(&mut temp_grid);
        let rule = grid_to_str(&temp_grid);
        rules.insert(rule.to_owned(), output.to_owned());

        // flip it and insert rotations
        flip_vert(&mut temp_grid);
        let rule = grid_to_str(&temp_grid);
        rules.insert(rule.to_owned(), output.to_owned());
        rotate_cw(&mut temp_grid);
        let rule = grid_to_str(&temp_grid);
        rules.insert(rule.to_owned(), output.to_owned());
        rotate_cw(&mut temp_grid);
        let rule = grid_to_str(&temp_grid);
        rules.insert(rule.to_owned(), output.to_owned());
        rotate_cw(&mut temp_grid);
        let rule = grid_to_str(&temp_grid);
        rules.insert(rule.to_owned(), output.to_owned());
    }
    rules
}

fn pixels_on(grid: &Vec<Vec<char>>) -> u32 {
    grid.iter()
        .map(|r| r.iter().filter(|c| **c == ON).count() as u32)
        .sum()
}

fn rotate_cw(grid: &mut Vec<Vec<char>>) {
    grid.reverse();
    for i in 1..grid.len() {
        let (left, right) = grid.split_at_mut(i);
        for (j, left_item) in left.iter_mut().enumerate().take(i) {
            mem::swap(&mut left_item[i], &mut right[0][j]);
        }
    }
}

fn rotate_ccw() {}

fn flip_horiz(grid: &mut Vec<Vec<char>>) {
    for row in grid.iter() {
        // row.reverse();
    }
}

fn flip_vert(grid: &mut Vec<Vec<char>>) {
    grid.reverse();
}

fn str_to_grid(pattern: &str) -> Vec<Vec<char>> {
    let mut ret: Vec<Vec<char>> = vec![vec![]];
    let mut line: usize = 0;
    for c in pattern.chars() {
        match c {
            ON | OFF => ret[line].push(c),
            '/' => {
                ret.push(vec![]);
                line += 1;
            }
            _ => panic!("Unexpected character!"),
        }
    }
    ret
}

fn grid_to_str(pattern: &Vec<Vec<char>>) -> String {
    let mut ret = String::new();
    let mut first_line = true;
    for line in pattern {
        if !first_line {
            ret.push('/');
        }
        for c in line {
            ret.push(*c);
        }
        first_line = false;
    }
    ret
}

fn starting_pattern() -> Vec<Vec<char>> {
    vec![vec![OFF, ON, OFF], vec![OFF, OFF, ON], vec![ON, ON, ON]]
}
