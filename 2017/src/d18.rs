use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solve(input: String) -> (i64, i64) {
    let registers: HashMap<char, i64> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| (c, 0i64))
        .collect::<HashMap<_, _>>();

    let mut prog0 = Program::new(parse_instructions(input.clone()), registers.clone(), 0);
    let mut prog1 = Program::new(parse_instructions(input.clone()), registers.clone(), 1);

    let mut part1_answer: i64 = 0;
    loop {        
        let p0_blocked = prog0.step();
        let p1_blocked = prog1.step();
        if p0_blocked && p1_blocked {
            break;
        }
        if prog0.last_rcv != 0 && part1_answer == 0 {
            part1_answer = prog0.last_rcv;
        }
        match prog0.output {
            Some(v) => prog1.input_q.push_back(v),
            None => (),
        };
        match prog1.output {
            Some(v) => prog0.input_q.push_back(v),
            None => (),
        };
    }

    (part1_answer, prog1.sends)
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
            "rcv" => list.push(Instruction::new(Command::Receive, reg, ' ', 0)),
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

struct Program {
    instructions: Vec<Instruction>,
    registers: HashMap<char, i64>,
    pc: i64,
    pub last_send: i64,
    pub output: Option<i64>,
    pub last_rcv: i64,
    pub input_q: VecDeque<i64>,
    pub sends: i64,
}

impl Program {
    fn new(insts: Vec<Instruction>, regs: HashMap<char, i64>, num: i64) -> Program {
        let mut prog = Program {
            instructions: insts,
            registers: regs,
            pc: 0,
            last_send: 0,
            output: None,
            last_rcv: 0,
            input_q: VecDeque::<i64>::new(),
            sends: 0,
        };
        *prog.registers.get_mut(&'p').unwrap() = num;
        prog
    }

    pub fn step(&mut self) -> bool {
        let inst = &self.instructions[self.pc as usize];
        let mut blocked = false;
        self.output = None;
        match inst.command {
            Command::Sound => {
                self.last_send = if inst.reg1.is_numeric() {
                    inst.reg1.to_digit(10).unwrap().into()
                } else {
                    self.registers[&inst.reg1]
                };
                self.output = Some(self.last_send);
                self.sends += 1;
                self.pc = self.pc + 1;
            }
            Command::Set => {
                *self.registers.get_mut(&inst.reg1).unwrap() = if inst.reg2 == ' ' {
                    inst.val2
                } else {
                    self.registers[&inst.reg2]
                };
                self.pc = self.pc + 1;
            }
            Command::Add => {
                let v2 = if inst.reg2 == ' ' {
                    inst.val2
                } else {
                    self.registers[&inst.reg2]
                };
                *self.registers.get_mut(&inst.reg1).unwrap() += v2;
                self.pc = self.pc + 1;
            }
            Command::Multiply => {
                let v2 = if inst.reg2 == ' ' {
                    inst.val2
                } else {
                    self.registers[&inst.reg2]
                };
                *self.registers.get_mut(&inst.reg1).unwrap() *= v2;
                self.pc = self.pc + 1;
            }
            Command::Modulus => {
                let v2 = if inst.reg2 == ' ' {
                    inst.val2
                } else {
                    self.registers[&inst.reg2]
                };
                *self.registers.get_mut(&inst.reg1).unwrap() %= v2;
                self.pc = self.pc + 1;
            }
            Command::Receive => {
                self.last_rcv = self.last_send;
                if self.input_q.is_empty() {
                    blocked = true;
                } else {
                    *self.registers.get_mut(&inst.reg1).unwrap() =
                        self.input_q.pop_front().unwrap();
                    self.pc = self.pc + 1;
                }
            }
            Command::JumpGtrZero => {
                let tval: i64 = if inst.reg1.is_numeric() {
                    inst.reg1.to_digit(10).unwrap().into()
                } else {
                    self.registers[&inst.reg1]
                };
                if tval > 0 {
                    self.pc += if inst.reg2 == ' ' {
                        inst.val2
                    } else {
                        self.registers[&inst.reg2]
                    };
                } else {
                    self.pc = self.pc + 1;
                }
            }
        }
        blocked
    }
}

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

enum Command {
    Sound,
    Set,
    Add,
    Multiply,
    Modulus,
    Receive,
    JumpGtrZero,
}
