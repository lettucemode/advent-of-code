// todo for part 2:
// - convert to Programs with runUntilBlock()
// - track num of things sent
// - track part 1 solution too (part about < 0 isn't in part 2 reqs)

use std::collections::HashMap;

pub fn solve(input: String) -> (i64, i64) {
    let mut registers: HashMap<char, i64> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| (c, 0i64))
        .collect::<HashMap<_, _>>();
    let mut last_played: i64 = 0;
    let mut last_recover: i64 = 0;
    let instructions = parse_instructions(input);
    let mut pc: i64 = 0;
    while 0 <= pc && pc < instructions.len() as i64 {
        let inst = &instructions[pc as usize];
        // println!("PC: {}, {:?}", pc, inst);
        match inst.command {
            Command::Sound => {
                last_played = registers[&inst.reg1];
                // println!(
                //     " BEEP BEEP {} BEEP BEEP ",
                //     registers[&inst.reg1]
                // );
                pc = pc + 1;
            }
            Command::Set => {
                *registers.get_mut(&inst.reg1).unwrap() = if inst.reg2 == ' ' {
                    inst.val2
                } else {
                    registers[&inst.reg2]
                };
                pc = pc + 1;
            }
            Command::Add => {
                let v2 = if inst.reg2 == ' ' {
                    inst.val2
                } else {
                    registers[&inst.reg2]
                };
                *registers.get_mut(&inst.reg1).unwrap() += v2;
                pc = pc + 1;
            }
            Command::Multiply => {
                let v2 = if inst.reg2 == ' ' {
                    inst.val2
                } else {
                    registers[&inst.reg2]
                };
                *registers.get_mut(&inst.reg1).unwrap() *= v2;
                pc = pc + 1;
            }
            Command::Modulus => {
                let v2 = if inst.reg2 == ' ' {
                    inst.val2
                } else {
                    registers[&inst.reg2]
                };
                *registers.get_mut(&inst.reg1).unwrap() %= v2;
                pc = pc + 1;
            }
            Command::Recover => {
                if registers[&inst.reg1] > 0 {
                    last_recover = last_played;
                    // break now for part 1
                    break;
                }
                pc = pc + 1;
            }
            Command::JumpGtrZero => {
                if registers[&inst.reg1] > 0 {
                    pc += if inst.reg2 == ' ' {
                        inst.val2
                    } else {
                        registers[&inst.reg2]
                    };
                } else {
                    pc = pc + 1;
                }
            }
        }
    }
    (last_recover, 0)
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    let mut list: Vec<Instruction> = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap();
        let reg = parts.next().unwrap().chars().nth(0).unwrap();
        match cmd {
            "snd" => list.push(Instruction::new(Command::Sound, reg, ' ', 0)),
            "set" => {
                let p2 = parts.next().unwrap();
                match p2.parse::<i64>() {
                    Ok(v) => list.push(Instruction::new(Command::Set, reg, ' ', v)),
                    Err(_e) => list.push(Instruction::new(
                        Command::Set,
                        reg,
                        p2.chars().nth(0).unwrap(),
                        0,
                    )),
                }
            }
            "add" => {
                let p2 = parts.next().unwrap();
                match p2.parse::<i64>() {
                    Ok(v) => list.push(Instruction::new(Command::Add, reg, ' ', v)),
                    Err(_e) => list.push(Instruction::new(
                        Command::Add,
                        reg,
                        p2.chars().nth(0).unwrap(),
                        0,
                    )),
                }
            }
            "mul" => {
                let p2 = parts.next().unwrap();
                match p2.parse::<i64>() {
                    Ok(v) => list.push(Instruction::new(Command::Multiply, reg, ' ', v)),
                    Err(_e) => list.push(Instruction::new(
                        Command::Multiply,
                        reg,
                        p2.chars().nth(0).unwrap(),
                        0,
                    )),
                }
            }
            "mod" => {
                let p2 = parts.next().unwrap();
                match p2.parse::<i64>() {
                    Ok(v) => list.push(Instruction::new(Command::Modulus, reg, ' ', v)),
                    Err(_e) => list.push(Instruction::new(
                        Command::Modulus,
                        reg,
                        p2.chars().nth(0).unwrap(),
                        0,
                    )),
                }
            }
            "rcv" => list.push(Instruction::new(Command::Recover, reg, ' ', 0)),
            "jgz" => {
                let p2 = parts.next().unwrap();
                match p2.parse::<i64>() {
                    Ok(v) => list.push(Instruction::new(Command::JumpGtrZero, reg, ' ', v)),
                    Err(_e) => list.push(Instruction::new(
                        Command::JumpGtrZero,
                        reg,
                        p2.chars().nth(0).unwrap(),
                        0,
                    )),
                }
            }
            _ => (),
        }
    }
    list
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    reg1: char,
    reg2: char,
    val2: i64,
}

impl Instruction {
    fn new(c: Command, r1: char, r2: char, v2: i64) -> Instruction {
        Instruction {
            command: c,
            reg1: r1,
            reg2: r2,
            val2: v2,
        }
    }
}

#[derive(Debug)]
enum Command {
    Sound,
    Set,
    Add,
    Multiply,
    Modulus,
    Recover,
    JumpGtrZero,
}
