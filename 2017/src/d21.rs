use std::collections::HashMap;

const ON: char = '#';
const OFF: char = '.';
const P1_ITERATIONS: u32 = 5;
const P2_ITERATIONS: u32 = 18;

pub fn solve(input: String) -> (u32, u32) {
    let mut grid = String::from(".#./..#/###");
    let rules: HashMap<String, String> = build_rules(input);
    for _ in 0..P1_ITERATIONS {
        grid = iterate(&grid, &rules);
    }
    let p1_ans = pixels_on(&grid);
    for _ in 0..P2_ITERATIONS - P1_ITERATIONS {
        grid = iterate(&grid, &rules);
    }
    (p1_ans, pixels_on(&grid))
}

fn iterate(grid: &str, rules: &HashMap<String, String>) -> String {
    let num_slashes = grid.chars().filter(|&c| c == '/').count();
    let row_size = (((grid.len() - num_slashes) % 2 != 0) as u32 + 2) as usize;

    // split grid into array of strings - one for each section
    let mut patterns: Vec<String> = vec![];
    let mut builds: Vec<Vec<char>> = vec![vec![]];
    let mut str_count = 0;
    let mut ch_count = 0;
    for ch in grid.chars() {
        if ch == '/' {
            ch_count = 0;
            str_count = 0;
        } else {
            builds[str_count].push(ch);
            ch_count += 1;
            if ch_count == row_size {
                if (row_size == 2 && builds[str_count].len() == 5)
                    || (row_size == 3 && builds[str_count].len() == 11)
                {
                    patterns.push(builds[str_count].clone().into_iter().collect());
                    builds[str_count].clear();
                } else {
                    builds[str_count].push('/');
                }
                str_count += 1;
                if str_count == builds.len() {
                    builds.push(vec![]);
                }
                ch_count = 0;
            }
        }
    }

    // transform each section
    for i in 0..patterns.len() {
        patterns[i] = rules[&patterns[i]].clone();
    }

    let row_len = patterns[0].find('/').unwrap();

    // put them back together
    if patterns.len() == 1 {
        return patterns[0].clone();
    } else {
        let combo_limit = (patterns.len() as f64).sqrt() as usize;
        let mut result = String::from("");

        let mut row = 0;
        let mut p = 0;
        let mut c = 0;
        loop {
            let chunk = patterns[p].split_terminator('/').collect::<Vec<&str>>()[c];
            result.push_str(chunk);
            p += 1;
            if p % combo_limit == 0 {
                p = row;
                c += 1;
                if c == row_len {
                    row += combo_limit;
                    if row == patterns.len() {
                        break;
                    }
                    p = row;
                    c = 0;
                }
                result.push('/');
            }
        }
        result
    }
}

fn build_rules(input: String) -> HashMap<String, String> {
    let mut rules: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        // insert orig rule
        let things: Vec<&str> = line.split_terminator(" => ").collect();
        let mut input = things[0].to_owned();
        let output = things[1];
        rules.insert(input.to_owned(), output.to_owned());

        // insert rotations
        input = rotate(&input);
        rules.insert(input.to_owned(), output.to_owned());
        input = rotate(&input);
        rules.insert(input.to_owned(), output.to_owned());
        input = rotate(&input);
        rules.insert(input.to_owned(), output.to_owned());

        // flip it and insert rotations
        input = flip(&input);
        rules.insert(input.to_owned(), output.to_owned());
        input = rotate(&input);
        rules.insert(input.to_owned(), output.to_owned());
        input = rotate(&input);
        rules.insert(input.to_owned(), output.to_owned());
        input = rotate(&input);
        rules.insert(input.to_owned(), output.to_owned());
    }
    rules
}

fn pixels_on(pattern: &str) -> u32 {
    pattern.chars().filter(|&c| c == ON).count() as u32
}

fn rotate(pattern: &str) -> String {
    let mut rbytes: Vec<u8> = vec![];
    let pattern_bytes = pattern.as_bytes();
    if pattern.len() == 5 {
        rbytes.push(pattern_bytes[3]);
        rbytes.push(pattern_bytes[0]);
        rbytes.push(pattern_bytes[2]);
        rbytes.push(pattern_bytes[4]);
        rbytes.push(pattern_bytes[1]);
    } else if pattern.len() == 11 {
        rbytes.push(pattern_bytes[8]);
        rbytes.push(pattern_bytes[4]);
        rbytes.push(pattern_bytes[0]);
        rbytes.push(pattern_bytes[3]);
        rbytes.push(pattern_bytes[9]);
        rbytes.push(pattern_bytes[5]);
        rbytes.push(pattern_bytes[1]);
        rbytes.push(pattern_bytes[7]);
        rbytes.push(pattern_bytes[10]);
        rbytes.push(pattern_bytes[6]);
        rbytes.push(pattern_bytes[2]);
    } else {
        panic!("called rotate() with a bad-length'd string.");
    }
    String::from_utf8(rbytes).unwrap()
}

fn flip(pattern: &str) -> String {
    let mut rbytes: Vec<u8> = vec![];
    let pattern_bytes = pattern.as_bytes();
    if pattern.len() == 5 {
        rbytes.push(pattern_bytes[3]);
        rbytes.push(pattern_bytes[4]);
        rbytes.push(pattern_bytes[2]);
        rbytes.push(pattern_bytes[0]);
        rbytes.push(pattern_bytes[1]);
    } else if pattern.len() == 11 {
        rbytes.push(pattern_bytes[8]);
        rbytes.push(pattern_bytes[9]);
        rbytes.push(pattern_bytes[10]);
        rbytes.push(pattern_bytes[3]);
        rbytes.push(pattern_bytes[4]);
        rbytes.push(pattern_bytes[5]);
        rbytes.push(pattern_bytes[6]);
        rbytes.push(pattern_bytes[7]);
        rbytes.push(pattern_bytes[0]);
        rbytes.push(pattern_bytes[1]);
        rbytes.push(pattern_bytes[2]);
    } else {
        panic!("called flip() with a bad-length'd string.");
    }
    String::from_utf8(rbytes).unwrap()
}
