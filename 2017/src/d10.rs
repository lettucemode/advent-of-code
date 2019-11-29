use crate::common;

pub fn solve(input: String) -> (u32, String) {
    let mut circle: Vec<u8> = (0..=255).collect();
    let p1_lens: Vec<usize> = input
        .split_terminator(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    common::knot_hash_round(&mut circle, &p1_lens, &mut 0, &mut 0);

    (
        circle[0] as u32 * circle[1] as u32,
        common::knot_hash(&input),
    )
}
