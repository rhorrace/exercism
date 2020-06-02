// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            pos: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let new_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            direction: new_direction,
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        let new_direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            direction: new_direction,
            ..self
        }
    }

    pub fn advance(self) -> Self {
        let new_pos = match self.direction {
            Direction::North => (self.pos.0, self.pos.1 + 1),
            Direction::East => (self.pos.0 + 1, self.pos.1),
            Direction::South => (self.pos.0, self.pos.1 - 1),
            Direction::West => (self.pos.0 - 1, self.pos.1),
        };
        Self {
            pos: new_pos,
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot_state = self;
        for state in instructions.chars() {
            robot_state = match state {
                'L' => robot_state.turn_left(),
                'R' => robot_state.turn_right(),
                'A' => robot_state.advance(),
                _ => robot_state
            };
        }
        robot_state
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
