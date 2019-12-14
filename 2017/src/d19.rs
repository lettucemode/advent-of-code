pub fn solve(input: String) -> (String, u32) {
    let diagram: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let start_x: usize = diagram[0].iter().position(|&x| x != ' ').unwrap();
    let mut pos = Point { x: start_x, y: 0 };
    let mut dir = Direction::Down;
    let mut letters = String::from("");
    let mut step_count = 0;

    loop {
        match dir {
            Direction::Up => pos.y -= 1,
            Direction::Down => pos.y += 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
        };
        step_count += 1;

        match diagram[pos.y][pos.x] {
            'A'..='Z' => letters.push(diagram[pos.y][pos.x]),
            '+' => match dir {
                Direction::Up | Direction::Down => {
                    dir = if pos.x == 0 || diagram[pos.y][pos.x - 1] == '-' {
                        Direction::Left
                    } else {
                        Direction::Right
                    };
                }
                Direction::Left | Direction::Right => {
                    dir = if pos.y == 0 || diagram[pos.y + 1][pos.x] == '|' {
                        Direction::Down
                    } else {
                        Direction::Up
                    };
                }
            },
            '-' => (),
            '|' => (),
            ' ' => break,
            _ => (),
        }
    }

    (letters, step_count)
}

struct Point {
    x: usize,
    y: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
