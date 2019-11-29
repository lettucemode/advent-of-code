use crate::common;
const SIZE: i32 = 128;

pub fn solve(input: String) -> (u32, u32) {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    let mut num_groups = 0;
    for i in 0..SIZE {
        let hash_input = format!("{}-{}", input, i);
        let hash = common::knot_hash(&hash_input);
        let bytes = hex::decode(hash).unwrap();
        grid.push(
            bytes
                .iter()
                .map(|x| format!("{:08b}", x))
                .collect::<Vec<String>>()
                .concat()
                .chars()
                .map(|x| Cell { bit: x, group: 0 })
                .collect(),
        );
    }
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j].bit == '0' || grid[i][j].group > 0 {
                continue;
            }

            num_groups += 1;
            recurse(&mut grid, num_groups, i, j);
        }
    }
    (
        grid.iter()
            .map(|x| x.iter().filter(|&y| y.bit == '1').count() as u32)
            .sum(),
        num_groups,
    )
}

fn recurse(grid: &mut Vec<Vec<Cell>>, gnum: u32, x: usize, y: usize) {
    grid[x][y].group = gnum;
    // this seems suboptimal but, eh
    if 0 <= x as i32 - 1 && grid[x - 1][y].group == 0 && grid[x - 1][y].bit == '1' {
        recurse(grid, gnum, x - 1, y);
    }
    if x as i32 + 1 < SIZE && grid[x + 1][y].group == 0 && grid[x + 1][y].bit == '1' {
        recurse(grid, gnum, x + 1, y);
    }
    if 0 <= y as i32 - 1 && grid[x][y - 1].group == 0 && grid[x][y - 1].bit == '1' {
        recurse(grid, gnum, x, y - 1);
    }
    if y as i32 + 1 < SIZE && grid[x][y + 1].group == 0 && grid[x][y + 1].bit == '1' {
        recurse(grid, gnum, x, y + 1);
    }
}

struct Cell {
    bit: char,
    group: u32,
}
