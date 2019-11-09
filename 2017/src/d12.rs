use std::collections::HashMap;

pub fn solve(input: String) -> (usize, u32) {
    let mut pipes: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut currg: Vec<u32> = Vec::new();
    let mut stack: Vec<u32> = vec![0];
    for pipestr in input.split_terminator('\n') {
        let mut pipeiter = pipestr.split_terminator("<->");
        let key: u32 = pipeiter.next().unwrap().trim().parse().unwrap();
        pipes.insert(
            key,
            pipeiter
                .next()
                .unwrap()
                .split_terminator(',')
                .map(|x| x.trim().parse().unwrap())
                .collect(),
        );
    }

    // part 1
    while !stack.is_empty() {
        let cur: u32 = stack.pop().unwrap();
        if !currg.contains(&cur) {
            currg.push(cur);
            for link in &pipes[&cur] {
                stack.push(*link);
            }
        }
    }
    let p1_ans = currg.len();

    // part 2
    let mut total_groups: u32 = 1;
    for thing in &currg {
        pipes.remove(thing);
    }
    currg.clear();
    while !pipes.is_empty() {
        stack.push(*pipes.iter().nth(0).unwrap().0);
        while !stack.is_empty() {
            let cur: u32 = stack.pop().unwrap();
            if !currg.contains(&cur) {
                currg.push(cur);
                for link in &pipes[&cur] {
                    stack.push(*link);
                }
            }
        }
        for thing in &currg {
            pipes.remove(thing);
        }
        currg.clear();
        total_groups += 1;
    }

    (p1_ans, total_groups)
}
