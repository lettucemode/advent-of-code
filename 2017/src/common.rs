pub fn get_input(path: String) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub fn knot_hash(input: &str) -> String {
    let mut lengths: Vec<usize> = input.chars().map(|x| x as usize).collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let mut circle: Vec<u8> = (0..=255).collect();
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        knot_hash_round(&mut circle, &lengths, &mut pos, &mut skip);
    }

    let dense_hash: Vec<u8> = circle
        .chunks_exact(16)
        .map(|c| c.iter().fold(0, |acc, &x| x ^ acc))
        .collect();
    dense_hash.iter().map(|&x| hex::encode([x])).collect()
}

pub fn knot_hash_round(circle: &mut [u8], lengths: &[usize], pos: &mut usize, skip: &mut usize) {
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
