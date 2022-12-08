#[derive(Debug, Clone)]
pub struct Position {
    row: i32,
    col: i32,
    orientation: Option<Direction>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn as_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Movement {
    Forward,
    Left,
    Right,
    Back,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Rotation {
    Left,
    Right,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        Self {
            row,
            col,
            orientation: None,
        }
    }
    pub fn new_oriented(row: i32, col: i32, orientation: Direction) -> Self {
        Self {
            row,
            col,
            orientation: Some(orientation),
        }
    }

    pub fn row(&self) -> i32 {
        self.row
    }
    pub fn col(&self) -> i32 {
        self.col
    }
    pub fn orientation(&self) -> Option<Direction> {
        self.orientation
    }

    pub fn move_absolute(&self, direction: Direction, distance: i32) -> Position {
        match direction {
            Direction::Up => Position {
                row: self.row - distance,
                col: self.col,
                orientation: self.orientation,
            },
            Direction::Down => Position {
                row: self.row + distance,
                col: self.col,
                orientation: self.orientation,
            },
            Direction::Left => Position {
                row: self.row,
                col: self.col - distance,
                orientation: self.orientation,
            },
            Direction::Right => Position {
                row: self.row,
                col: self.col + distance,
                orientation: self.orientation,
            },
        }
    }
    pub fn move_relative(&self, movement: Movement, distance: i32) -> Position {
        if let Some(orientation) = self.orientation {
            match movement {
                Movement::Forward => match orientation {
                    Direction::Up => self.move_absolute(Direction::Up, distance),
                    Direction::Down => self.move_absolute(Direction::Down, distance),
                    Direction::Left => self.move_absolute(Direction::Left, distance),
                    Direction::Right => self.move_absolute(Direction::Right, distance),
                },
                Movement::Left => match orientation {
                    Direction::Up => self.move_absolute(Direction::Left, distance),
                    Direction::Down => self.move_absolute(Direction::Right, distance),
                    Direction::Left => self.move_absolute(Direction::Down, distance),
                    Direction::Right => self.move_absolute(Direction::Up, distance),
                },
                Movement::Right => match orientation {
                    Direction::Up => self.move_absolute(Direction::Right, distance),
                    Direction::Down => self.move_absolute(Direction::Left, distance),
                    Direction::Left => self.move_absolute(Direction::Up, distance),
                    Direction::Right => self.move_absolute(Direction::Down, distance),
                },
                Movement::Back => match orientation {
                    Direction::Up => self.move_absolute(Direction::Down, distance),
                    Direction::Down => self.move_absolute(Direction::Up, distance),
                    Direction::Left => self.move_absolute(Direction::Right, distance),
                    Direction::Right => self.move_absolute(Direction::Left, distance),
                },
            }
        } else {
            panic!("Position has no orientation")
        }
    }

    pub fn rotate(&self, rotation: Rotation) -> Self {
        let new_orientation = match self.orientation {
            Some(Direction::Up) => match rotation {
                Rotation::Left => Direction::Left,
                Rotation::Right => Direction::Right,
            },
            Some(Direction::Right) => match rotation {
                Rotation::Left => Direction::Up,
                Rotation::Right => Direction::Down,
            },
            Some(Direction::Down) => match rotation {
                Rotation::Left => Direction::Right,
                Rotation::Right => Direction::Left,
            },
            Some(Direction::Left) => match rotation {
                Rotation::Left => Direction::Down,
                Rotation::Right => Direction::Up,
            },
            None => panic!("Position has no orientation"),
        };
        Self {
            row: self.row,
            col: self.col,
            orientation: Some(new_orientation),
        }
    }
}
