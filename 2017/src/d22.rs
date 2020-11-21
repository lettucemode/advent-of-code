use std::collections::HashMap;

const CLEAN: char = '.';
const WEAKENED: char = 'W';
const INFECTED: char = '#';
const FLAGGED: char = 'F';

pub fn solve(input: String) -> (u32, u32) {
    let grid_size = input.find('\n').unwrap() - 1;
    let midpoint = grid_size as i32 / 2;
    let the_grid: HashMap<(i32, i32), NodeState> = create_grid(input);
    let agent = Agent {
        x: midpoint,
        y: midpoint,
        direction: Direction::UP,
    };
    (
        part_one(the_grid.clone(), agent.clone()),
        part_two(the_grid.clone(), agent.clone()),
    )
}

fn part_one(mut the_grid: HashMap<(i32, i32), NodeState>, mut agent: Agent) -> u32 {
    let mut infections_caused = 0;
    for _ in 0..10000 {
        let node = the_grid
            .entry((agent.y, agent.x))
            .or_insert(NodeState::CLEAN);

        if *node == NodeState::CLEAN {
            agent.direction = match agent.direction {
                Direction::UP => Direction::LEFT,
                Direction::RIGHT => Direction::UP,
                Direction::DOWN => Direction::RIGHT,
                Direction::LEFT => Direction::DOWN,
            };
            infections_caused += 1;
        } else {
            agent.direction = match agent.direction {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
            };
        }

        *node = match *node {
            NodeState::CLEAN => NodeState::INFECTED,
            NodeState::INFECTED => NodeState::CLEAN,
            _ => node.clone(),
        };
        match agent.direction {
            Direction::UP => agent.y -= 1,
            Direction::RIGHT => agent.x += 1,
            Direction::DOWN => agent.y += 1,
            Direction::LEFT => agent.x -= 1,
        }
    }
    infections_caused
}

fn part_two(mut the_grid: HashMap<(i32, i32), NodeState>, mut agent: Agent) -> u32 {
    let mut infections_caused = 0;
    for _ in 0..10000000 {
        let node = the_grid
            .entry((agent.y, agent.x))
            .or_insert(NodeState::CLEAN);

        match *node {
            NodeState::CLEAN => {
                agent.direction = match agent.direction {
                    Direction::UP => Direction::LEFT,
                    Direction::RIGHT => Direction::UP,
                    Direction::DOWN => Direction::RIGHT,
                    Direction::LEFT => Direction::DOWN,
                };
            }
            NodeState::WEAKENED => infections_caused += 1,
            NodeState::INFECTED => {
                agent.direction = match agent.direction {
                    Direction::UP => Direction::RIGHT,
                    Direction::RIGHT => Direction::DOWN,
                    Direction::DOWN => Direction::LEFT,
                    Direction::LEFT => Direction::UP,
                };
            }
            NodeState::FLAGGED => {
                agent.direction = match agent.direction {
                    Direction::UP => Direction::DOWN,
                    Direction::RIGHT => Direction::LEFT,
                    Direction::DOWN => Direction::UP,
                    Direction::LEFT => Direction::RIGHT,
                }
            }
        }

        *node = match *node {
            NodeState::CLEAN => NodeState::WEAKENED,
            NodeState::WEAKENED => NodeState::INFECTED,
            NodeState::INFECTED => NodeState::FLAGGED,
            NodeState::FLAGGED => NodeState::CLEAN,
        };
        match agent.direction {
            Direction::UP => agent.y -= 1,
            Direction::RIGHT => agent.x += 1,
            Direction::DOWN => agent.y += 1,
            Direction::LEFT => agent.x -= 1,
        }
    }
    infections_caused
}

fn create_grid(input: String) -> HashMap<(i32, i32), NodeState> {
    let mut the_grid: HashMap<(i32, i32), NodeState> = HashMap::new();
    let lines: Vec<String> = input.lines().map(|line| line.to_owned()).collect();
    for i in 0..lines.len() {
        let mut k = 0;
        for ch in lines[i].chars() {
            the_grid.insert(
                (i as i32, k),
                if ch == CLEAN {
                    NodeState::CLEAN
                } else {
                    NodeState::INFECTED
                },
            );
            k += 1;
        }
    }
    the_grid
}

#[derive(Clone, PartialEq)]
enum NodeState {
    CLEAN,
    WEAKENED,
    INFECTED,
    FLAGGED,
}

#[derive(Clone, Debug)]
struct Agent {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
