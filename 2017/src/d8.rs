use std::collections::HashMap;

pub fn solve(input: String) -> (i32, i32) {
    let mut rs: HashMap<String, i32> = HashMap::new();
    let mut highest_ever: i32 = std::i32::MIN;
    for line in input.split_terminator("\n") {
        let mut iter = line.split_whitespace();
        let reg1 = iter.next().unwrap();
        let op = iter.next().unwrap();
        let val1 = iter.next().unwrap();
        iter.next();
        let reg2 = iter.next().unwrap();
        let cmp = iter.next().unwrap();
        let val2 = iter.next().unwrap();

        let test_val = val2.parse::<i32>().unwrap();
        let test_reg = v(&mut rs, reg2);
        let test_pass = match cmp {
            "<" => *test_reg < test_val,
            "<=" => *test_reg <= test_val,
            ">" => *test_reg > test_val,
            ">=" => *test_reg >= test_val,
            "==" => *test_reg == test_val,
            "!=" => *test_reg != test_val,
            _ => false,
        };

        if test_pass {
            if op == "dec" {
                *v(&mut rs, reg1) -= val1.parse::<i32>().unwrap();
            } else if op == "inc" {
                *v(&mut rs, reg1) += val1.parse::<i32>().unwrap();
            }
        }

        highest_ever = rs
            .iter()
            .fold(highest_ever, |acc, x| if acc > *x.1 { acc } else { *x.1 });
    }
    (
        rs.iter()
            .fold(std::i32::MIN, |acc, x| if acc > *x.1 { acc } else { *x.1 }),
        highest_ever,
    )
}

fn v<'a>(rs: &'a mut HashMap<String, i32>, s: &str) -> &'a mut i32 {
    rs.entry(s.to_owned()).or_insert_with(|| 0)
}
