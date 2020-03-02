// Part 1: bijankplfgmeodhc
// Part 2: bpjahknliomefdgc
// so for this one, I eventually noticed that every factor of 10th answer
// (after 10 runs, 100 runs, 1000 runs, etc.) was the same answer
// so probably the correct solution here is to implement something that looks for patterns
// and how often it loops etc, then extrapolate that out to 1 billion
// but eh, I've been stuck on this for so long...moving on!

enum MoveType {
    Spin,
    Exchange,
    Partner,
}

struct DanceMove {
    step: MoveType,
    p1: u8,
    p2: u8,
}

pub fn solve(input: String) -> (String, String) {
    let moves: Vec<DanceMove> = build_moves_list(input);
    let mut front: i8 = 0;
    let mut dance_line: Vec<char> = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]
    .iter()
    .cloned()
    .collect();
    do_a_dance(&moves, &mut dance_line, &mut front);
    let first_dance = to_string(&dance_line, front);
    for _ in 0..(100 - 1) {
        do_a_dance(&moves, &mut dance_line, &mut front);
    }
    (first_dance, to_string(&dance_line, front))
}

fn build_moves_list(input: String) -> Vec<DanceMove> {
    let mut moves: Vec<DanceMove> = vec![];
    for line in input.split_terminator(',') {
        let stepc = &line[..1];
        let move_type: MoveType = if stepc == "s" {
            MoveType::Spin
        } else if stepc == "x" {
            MoveType::Exchange
        } else {
            MoveType::Partner
        };
        let p1: u8;
        let p2: u8;
        match move_type {
            MoveType::Spin => {
                p1 = line[1..].parse().unwrap();
                p2 = 0;
            }
            MoveType::Exchange => {
                let slash_index = line.find('/').unwrap();
                p1 = (&line[1..slash_index]).parse().unwrap();
                p2 = (&line[slash_index + 1..]).parse().unwrap();
            }
            MoveType::Partner => {
                p1 = line.chars().nth(1).unwrap() as u8;
                p2 = line.chars().nth(3).unwrap() as u8;
            }
        }
        moves.push(DanceMove {
            step: move_type,
            p1: p1,
            p2: p2,
        })
    }
    moves
}

fn do_a_dance(moves: &Vec<DanceMove>, dance_line: &mut Vec<char>, front: &mut i8) {
    let dance_len = dance_line.len();
    for dance_move in moves {
        match dance_move.step {
            MoveType::Spin => {
                *front = (*front + dance_len as i8 - dance_move.p1 as i8) % dance_len as i8;
            }
            MoveType::Exchange => {
                dance_line.swap(
                    (*front as usize + dance_move.p1 as usize) % dance_len,
                    (*front as usize + dance_move.p2 as usize) % dance_len,
                );
            }
            MoveType::Partner => {
                let mut i1: usize = dance_len;
                let mut i2: usize = dance_len;
                for i in 0..dance_len {
                    if dance_line[i] == dance_move.p1 as char {
                        i1 = i;
                    } else if dance_line[i] == dance_move.p2 as char {
                        i2 = i;
                    }
                    if i1 < dance_len && i2 < dance_len {
                        break;
                    }
                }
                dance_line.swap(i1, i2);
            }
        }
    }
}

fn to_string(dance_line: &Vec<char>, front: i8) -> String {
    let dance_len = dance_line.len();
    let mut temp: Vec<char> = vec![' '; dance_len];
    for i in 0..dance_len {
        temp[i] = dance_line[(front as usize + i) % dance_len];
    }
    temp.iter().collect()
}
