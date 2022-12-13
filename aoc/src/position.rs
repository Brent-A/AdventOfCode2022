#[derive(Debug, Clone)]
pub struct Position {
    row: i32,
    col: i32,
    orientation: Option<Direction>,
}

pub const EACH_DIRECTION: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }
    pub fn as_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::None => '*',
        }
    }
    pub fn rotate(&self, rotation: Rotation) -> Direction {
        match self {
            Direction::Up => match rotation {
                Rotation::Left => Direction::Left,
                Rotation::Right => Direction::Right,
            },
            Direction::Right => match rotation {
                Rotation::Left => Direction::Up,
                Rotation::Right => Direction::Down,
            },
            Direction::Down => match rotation {
                Rotation::Left => Direction::Right,
                Rotation::Right => Direction::Left,
            },
            Direction::Left => match rotation {
                Rotation::Left => Direction::Down,
                Rotation::Right => Direction::Up,
            },
            Direction::None => Direction::None,
        }
    }
}

pub const EACH_MOVEMENT: [Movement; 4] = [
    Movement::Forward,
    Movement::Back,
    Movement::Left,
    Movement::Right,
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Movement {
    Forward,
    Left,
    Right,
    Back,
}

pub const EACH_ROTATION: [Rotation; 2] = [Rotation::Left, Rotation::Right];

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
            Direction::None => self.clone(),
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
                    Direction::None => self.clone(),
                },
                Movement::Left => match orientation {
                    Direction::Up => self.move_absolute(Direction::Left, distance),
                    Direction::Down => self.move_absolute(Direction::Right, distance),
                    Direction::Left => self.move_absolute(Direction::Down, distance),
                    Direction::Right => self.move_absolute(Direction::Up, distance),
                    Direction::None => self.clone(),
                },
                Movement::Right => match orientation {
                    Direction::Up => self.move_absolute(Direction::Right, distance),
                    Direction::Down => self.move_absolute(Direction::Left, distance),
                    Direction::Left => self.move_absolute(Direction::Up, distance),
                    Direction::Right => self.move_absolute(Direction::Down, distance),
                    Direction::None => self.clone(),
                },
                Movement::Back => match orientation {
                    Direction::Up => self.move_absolute(Direction::Down, distance),
                    Direction::Down => self.move_absolute(Direction::Up, distance),
                    Direction::Left => self.move_absolute(Direction::Right, distance),
                    Direction::Right => self.move_absolute(Direction::Left, distance),
                    Direction::None => self.clone(),
                },
            }
        } else {
            panic!("Position has no orientation")
        }
    }

    pub fn rotate(&self, rotation: Rotation) -> Self {
        let new_orientation = self.orientation.unwrap().rotate(rotation);
        Self {
            row: self.row,
            col: self.col,
            orientation: Some(new_orientation),
        }
    }
}
