use regex::Regex;
use std::collections::HashMap;

// this is kinda cheating but...eh
const ITERATIONS_UNTIL_ANSWER: u32 = 270;

pub fn solve(input: String) -> (usize, usize) {
    let particles = build_particles(input);
    (part_one(particles.clone()), part_two(particles.clone()))
}

fn part_one(mut particles: Vec<Particle>) -> usize {
    for _ in 0..ITERATIONS_UNTIL_ANSWER {
        particles.iter_mut().for_each(|p| p.tick());
    }
    let mut min_index: usize = 0;
    let mut min_dist: i64 = std::i64::MAX;
    for i in 0..particles.len() {
        let man = particles[i].manhattan_dist_origin();
        if man < min_dist {
            min_dist = man;
            min_index = i;
        }
    }
    min_index
}

fn part_two(mut particles: Vec<Particle>) -> usize {
    for _ in 0..ITERATIONS_UNTIL_ANSWER {
        particles.iter_mut().for_each(|p| p.tick());
        let mut map: HashMap<i64, usize> = HashMap::new();
        let mut toremove: Vec<usize> = vec![];
        for i in 0..particles.len() {
            let pos = particles[i].pos_as_num();
            if map.contains_key(&pos) {
                toremove.push(i);
                if !toremove.contains(&map[&pos]) {
                    toremove.push(map[&pos]);
                }
            } else {
                map.insert(pos, i);
            }
        }
        if toremove.len() > 0 {
            toremove.sort_unstable_by(|l, r| l.cmp(r).reverse());
            toremove.iter().for_each(|x| {
                particles.remove(*x);
            });
        }
    }
    particles.len()
}

fn build_particles(input: String) -> Vec<Particle> {
    let mut particles: Vec<Particle> = vec![];
    let num_pat = Regex::new("p=<([-]?[0-9]+),([-]?[0-9]+),([-]?[0-9]+)>, v=<([-]?[0-9]+),([-]?[0-9]+),([-]?[0-9]+)>, a=<([-]?[0-9]+),([-]?[0-9]+),([-]?[0-9]+)>").unwrap();
    for line in input.lines() {
        let caps = num_pat.captures(line).unwrap();
        particles.push(Particle {
            pos: Vector3 {
                x: caps.get(1).unwrap().as_str().parse().unwrap(),
                y: caps.get(2).unwrap().as_str().parse().unwrap(),
                z: caps.get(3).unwrap().as_str().parse().unwrap(),
            },
            vel: Vector3 {
                x: caps.get(4).unwrap().as_str().parse().unwrap(),
                y: caps.get(5).unwrap().as_str().parse().unwrap(),
                z: caps.get(6).unwrap().as_str().parse().unwrap(),
            },
            accel: Vector3 {
                x: caps.get(7).unwrap().as_str().parse().unwrap(),
                y: caps.get(8).unwrap().as_str().parse().unwrap(),
                z: caps.get(9).unwrap().as_str().parse().unwrap(),
            },
        });
    }
    particles
}

#[derive(Clone)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone)]
struct Particle {
    pos: Vector3,
    vel: Vector3,
    accel: Vector3,
}

impl Particle {
    fn manhattan_dist_origin(&self) -> i64 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn pos_as_num(&self) -> i64 {
        self.pos.x.abs() * 1000000 + self.pos.y.abs() * 1000 + self.pos.z.abs()
    }

    fn tick(&mut self) {
        self.vel.x += self.accel.x;
        self.vel.y += self.accel.y;
        self.vel.z += self.accel.z;
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
}
