enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut direction = Direction::Right;
    let numbers = 1..=size * size;
    let mut spiral: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    let mut pos: (usize, usize) = (0, 0);

    for n in numbers {
        spiral[pos.0][pos.1] = n;
        match direction {
            Direction::Up =>
                if (pos.0 as u32) > 0 && spiral[pos.0 - 1][pos.1] == 0 {
                    pos.0 -= 1;
                } else {
                    direction = Direction::Right;
                    pos.1 += 1;
                },
            Direction::Down =>
                if (pos.0 as u32) < size - 1 && spiral[pos.0 + 1][pos.1] == 0 {
                    pos.0 += 1;
                } else {
                    direction = Direction::Left;
                    pos.1 -= 1;
                },
            Direction::Left =>
                if (pos.1 as usize) > 0 && spiral[pos.0][pos.1 - 1] == 0 {
                    pos.1 -= 1;
                } else {
                    direction = Direction::Up;
                    pos.0 -= 1;
                },
            Direction::Right =>
                if (pos.1 as u32) < size - 1 && spiral[pos.0][pos.1 + 1] == 0 {
                    pos.1 += 1;
                } else {
                    direction = Direction::Down;
                    pos.0 += 1;
                },
        }
    }
    spiral
}
