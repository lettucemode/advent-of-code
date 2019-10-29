pub fn solve(input: String) -> (u32, u32) {
    let mut points: u32 = 0;
    let mut inc: u32 = 0;
    let mut garbage: bool = false;
    let mut gcount: u32 = 0;
    let mut iter = input.chars();
    loop {
        let ch: char = match iter.next() {
            None => break,
            Some(thing) => thing,
        };
        match ch {
            '!' => {
                iter.next();
            }
            '{' => {
                if !garbage {
                    inc += 1;
                } else {
                    gcount += 1;
                }
            }
            '}' => {
                if !garbage {
                    points += inc;
                    inc -= 1;
                } else {
                    gcount += 1;
                }
            }
            '<' => {
                if garbage {
                    gcount += 1;
                }
                garbage = true;
            }
            '>' => {
                garbage = false;
            }
            _ => {
                if garbage {
                    gcount += 1;
                }
            }
        }
    }
    (points, gcount)
}
