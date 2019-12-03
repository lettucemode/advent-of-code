use std::collections::HashMap;
use std::str;

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
    let mut dance_line: HashMap<char, u8> = [
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
        ('k', 10),
        ('l', 11),
        ('m', 12),
        ('n', 13),
        ('o', 14),
        ('p', 15),
    ]
    .iter()
    .cloned()
    .collect();
    do_a_dance(&moves, &mut dance_line);
    let first_dance = to_string(&dance_line);
    for i in 0..(1000000000 - 1) {
        do_a_dance(&moves, &mut dance_line);
        if i % 1000 == 0 {
            println!("hey");
        }
    }
    (first_dance, to_string(&dance_line))
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

fn do_a_dance(moves: &Vec<DanceMove>, dance_line: &mut HashMap<char, u8>) {
    for dance_move in moves {
        match dance_move.step {
            MoveType::Spin => {
                // for pair in dance_line.iter_mut() {
                //     *pair.1 = (*pair.1 + dance_move.p1) % 16u8;
                // }
            }
            MoveType::Exchange => {
                // let c1 = *dance_line.iter().find(|x| *x.1 == dance_move.p1).unwrap().0;
                // let c2 = *dance_line.iter().find(|x| *x.1 == dance_move.p2).unwrap().0;
                // let t = *dance_line.get(&c1).unwrap();
                // *dance_line.get_mut(&c1).unwrap() = *dance_line.get(&c2).unwrap();
                // *dance_line.get_mut(&c2).unwrap() = t;
            }
            MoveType::Partner => {
                let c1 = dance_move.p1 as char;
                let c2 = dance_move.p2 as char;
                let t = *dance_line.get(&c1).unwrap();
                *dance_line.get_mut(&c1).unwrap() = *dance_line.get(&c2).unwrap();
                *dance_line.get_mut(&c2).unwrap() = t;
            }
        }
    }
}

fn to_string(dance_line: &HashMap<char, u8>) -> String {
    let mut bytes: Vec<u8> = vec![0; 16];
    for pair in dance_line.iter() {
        bytes[*pair.1 as usize] = *pair.0 as u8;
    }
    str::from_utf8(&bytes).unwrap().to_owned()
}
