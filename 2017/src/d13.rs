use std::collections::HashMap;

const MAX_LAYERS: u32 = 97;

pub fn solve(input: String) -> (u32, u32) {
    let mut firewall: HashMap<u32, u32> = HashMap::new();
    for line in input.split_terminator('\n') {
        let mut liter = line.split_terminator(':');
        firewall.insert(
            liter.next().unwrap().parse().unwrap(),
            liter.next().unwrap().trim().parse().unwrap(),
        );
    }
    let mut p2_ans: u32 = 1;
    loop {
        if !gets_caught(p2_ans, &firewall) {
            break;
        }
        p2_ans += 1;
    }
    (get_severity(0, &firewall), p2_ans)
}

fn get_severity(time: u32, firewall: &HashMap<u32, u32>) -> u32 {
    let mut severity: u32 = 0;
    for layer in 0..MAX_LAYERS {
        if firewall.contains_key(&layer) && is_caught(time + layer, firewall[&layer]) {
            severity += layer * firewall[&layer];
        }
    }
    severity
}

fn gets_caught(time: u32, firewall: &HashMap<u32, u32>) -> bool {
    for layer in 0..MAX_LAYERS {
        if firewall.contains_key(&layer) && is_caught(time + layer, firewall[&layer]) {
            return true;
        }
    }
    false
}

fn is_caught(time: u32, range: u32) -> bool {
    range <= 1 || time as i32 % (range as i32 + range as i32 - 2) == 0
}
