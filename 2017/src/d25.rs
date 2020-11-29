use std::collections::HashMap;

pub fn solve(_: String) -> (u32, u32) {
    let num_steps: u32 = 12794428;
    let mut state = STATE::A;
    let mut the_tape: HashMap<i32, bool> = HashMap::new();
    let mut cursor: i32 = 0;

    for _ in 0..num_steps {
        let v = the_tape.entry(cursor).or_insert(false);
        match state {
            STATE::A => {
                if !*v {
                    *v = true;
                    cursor += 1;
                    state = STATE::B;
                } else {
                    *v = false;
                    cursor -= 1;
                    state = STATE::F;
                }
            }
            STATE::B => {
                if !*v {
                    *v = false;
                    cursor += 1;
                    state = STATE::C;
                } else {
                    *v = false;
                    cursor += 1;
                    state = STATE::D;
                }
            }
            STATE::C => {
                if !*v {
                    *v = true;
                    cursor -= 1;
                    state = STATE::D;
                } else {
                    *v = true;
                    cursor += 1;
                    state = STATE::E;
                }
            }
            STATE::D => {
                if !*v {
                    *v = false;
                    cursor -= 1;
                    state = STATE::E;
                } else {
                    *v = false;
                    cursor -= 1;
                    state = STATE::D;
                }
            }
            STATE::E => {
                if !*v {
                    *v = false;
                    cursor += 1;
                    state = STATE::A;
                } else {
                    *v = true;
                    cursor += 1;
                    state = STATE::C;
                }
            }
            STATE::F => {
                if !*v {
                    *v = true;
                    cursor -= 1;
                    state = STATE::A;
                } else {
                    *v = true;
                    cursor += 1;
                    state = STATE::A;
                }
            }
        }
    }

    let p1_answer: u32 = the_tape.iter().filter(|&(&_, &v)| v).count() as u32;

    (p1_answer, 0)
}

enum STATE {
    A,
    B,
    C,
    D,
    E,
    F,
}
