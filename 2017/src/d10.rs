use hex;

pub fn solve(input: String) -> (u32, String) {
    let initial_circle: Vec<u8> = (0..=255).collect();

    // part 1
    let p1_lens: Vec<usize> = input
        .split_terminator(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut p1_circle = initial_circle.clone();
    hash_round(&mut p1_circle, &p1_lens, &mut 0, &mut 0);

    // part 2
    let mut p2_lens: Vec<usize> = input.chars().map(|x| x as usize).collect();
    p2_lens.append(&mut vec![17, 31, 73, 47, 23]);
    let mut pos = 0;
    let mut skip = 0;
    let mut p2_circle = initial_circle.clone();
    for _ in 0..64 {
        hash_round(&mut p2_circle, &p2_lens, &mut pos, &mut skip);
    }
    let dense_hash: Vec<u8> = p2_circle
        .chunks_exact(16)
        .map(|c| c.iter().fold(0, |acc, &x| x ^ acc))
        .collect();
    let p2_ans: String = dense_hash.iter().map(|&x| hex::encode([x])).collect();

    // aw ye
    (p1_circle[0] as u32 * p1_circle[1] as u32, p2_ans)
}

fn hash_round(circle: &mut [u8], lengths: &[usize], pos: &mut usize, skip: &mut usize) {
    for span in lengths {
        let end = (*pos + span - 1) % circle.len();
        let mut c: usize = 0;
        while c < span / 2 {
            let i = (*pos + c) % circle.len();
            let mut j = end as i32 - c as i32;
            if j < 0 {
                j += circle.len() as i32;
            }
            circle.swap(i, j as usize);
            c += 1;
        }
        *pos = (*pos + span + *skip) % circle.len();
        *skip += 1;
    }
}
