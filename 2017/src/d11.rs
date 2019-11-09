const N: &str = "n";
const NE: &str = "ne";
const SE: &str = "se";
const S: &str = "s";
const SW: &str = "sw";
const NW: &str = "nw";

pub fn solve(input: String) -> (i32, i32) {
    let mut q: i32 = 0;
    let mut r: i32 = 0;
    let mut max_dist = 0;
    for step in input.split_terminator(',') {
        match step {
            N => {
                r -= 1;
            }
            NE => {
                r -= 1;
                q += 1;
            }
            SE => {
                q += 1;
            }
            S => {
                r += 1;
            }
            SW => {
                r += 1;
                q -= 1;
            }
            NW => {
                q -= 1;
            }
            _ => (),
        }
        max_dist = std::cmp::max(max_dist, axial_dist(q, r));
    }
    (axial_dist(q, r), max_dist)
}

fn axial_dist(q: i32, r: i32) -> i32 {
    std::cmp::max(std::cmp::max(q.abs(), r.abs()), (-q - r).abs())
}
