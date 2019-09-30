pub fn solve(input: String) -> (u32, u32) {
    (part_one(&input), part_two(&input))
}

fn part_one(input: &str) -> u32 {
    let mut list: Vec<&str> = Vec::new();
    let mut total = 0;
    for line in input.split_terminator("\n") {
        list.clear();
        let mut valid = true;
        for passphrase in line.split_whitespace() {
            if !list.contains(&passphrase) {
                list.push(&passphrase);
            } else {
                valid = false;
                break;
            }
        }
        if valid {
            total = total + 1;
        }
    }
    total
}

fn part_two(input: &str) -> u32 {
    let mut total = 0;
    let mut list: Vec<String> = Vec::new();
    for line in input.split_terminator("\n") {
        list.clear();
        let mut valid = true;
        for passphrase in line.split_whitespace() {
            let mut sorted: Vec<char> = passphrase.chars().collect();
            sorted.sort();
            let sorted: String = sorted.into_iter().collect();
            if !list.contains(&sorted) {
                list.push(sorted);
            } else {
                valid = false;
                break;
            }
        }
        if valid {
            total = total + 1;
        }
    }
    total
}
