const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const DIVISOR: u64 = 2147483647;
const PART_1_LOOPS: u64 = 40000000;
const PART_2_LOOPS: u64 = 5000000;
const MULT_A: u64 = 4;
const MULT_B: u64 = 8;

pub fn solve(input: String) -> (u32, u32) {
    let mut judge_points_p1: u32 = 0;
    let mut judge_points_p2: u32 = 0;
    let mut fiter = input.split_terminator('\n');
    let a_start: u64 = fiter
        .next()
        .unwrap()
        .split_whitespace()
        .nth(4)
        .unwrap()
        .parse()
        .unwrap();
    let b_start: u64 = fiter
        .next()
        .unwrap()
        .split_whitespace()
        .nth(4)
        .unwrap()
        .parse()
        .unwrap();
    // part 1
    let mut gen_a = GenP1::new(FACTOR_A, a_start);
    let mut gen_b = GenP1::new(FACTOR_B, b_start);
    for _ in 0..PART_1_LOOPS {
        let a_bits = format!("{:016b}", gen_a.next().unwrap());
        let b_bits = format!("{:016b}", gen_b.next().unwrap());
        let a_lower = &a_bits[a_bits.len() - 16..];
        let b_lower = &b_bits[b_bits.len() - 16..];
        if a_lower == b_lower {
            judge_points_p1 += 1;
        }
    }
    // part 2
    let mut gen_a = GenP2::new(FACTOR_A, a_start, MULT_A);
    let mut gen_b = GenP2::new(FACTOR_B, b_start, MULT_B);
    for _ in 0..PART_2_LOOPS {
        let a_bits = format!("{:016b}", gen_a.next().unwrap());
        let b_bits = format!("{:016b}", gen_b.next().unwrap());
        let a_lower = &a_bits[a_bits.len() - 16..];
        let b_lower = &b_bits[b_bits.len() - 16..];
        if a_lower == b_lower {
            judge_points_p2 += 1;
        }
    }
    (judge_points_p1, judge_points_p2)
}

struct GenP1 {
    lastval: u64,
    factor: u64,
}

impl GenP1 {
    fn new(factor: u64, startval: u64) -> GenP1 {
        GenP1 {
            factor: factor,
            lastval: startval,
        }
    }
}

impl Iterator for GenP1 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.lastval = (self.lastval * self.factor) % DIVISOR;
        Some(self.lastval)
    }
}

struct GenP2 {
    lastval: u64,
    factor: u64,
    multiple_restriction: u64,
}

impl GenP2 {
    fn new(factor: u64, startval: u64, mult: u64) -> GenP2 {
        GenP2 {
            factor: factor,
            lastval: startval,
            multiple_restriction: mult,
        }
    }
}

impl Iterator for GenP2 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.lastval = (self.lastval * self.factor) % DIVISOR;
            if self.lastval % self.multiple_restriction == 0 {
                break;
            }
        }
        Some(self.lastval)
    }
}
