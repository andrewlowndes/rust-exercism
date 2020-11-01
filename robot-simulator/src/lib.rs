#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            pos: (x, y),
            dir: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        let new_dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        self.dir = new_dir;
        self
    }

    pub fn turn_left(mut self) -> Self {
        let new_dir = match self.dir {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };

        self.dir = new_dir;
        self
    }

    pub fn advance(mut self) -> Self {
        match self.dir {
            Direction::North => self.pos.1 += 1,
            Direction::West => self.pos.0 -= 1,
            Direction::South => self.pos.1 -= 1,
            Direction::East => self.pos.0 += 1,
        };

        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut result = self;

        for instruction in instructions.chars() {
            match &instruction {
                'R' => {
                    result = result.turn_right();
                }
                'L' => {
                    result = result.turn_left();
                }
                'A' => {
                    result = result.advance();
                }
                _ => {
                    panic!(format!("Unknown char '{}'", instruction));
                }
            }
        }

        result
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
