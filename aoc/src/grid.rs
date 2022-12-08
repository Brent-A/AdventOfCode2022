use std::ops::Range;

use crate::position::{Direction, Position};

pub struct Grid<T> {
    tiles: Vec<Vec<T>>,
    origin: Position,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            origin: Position::new(0, 0),
        }
    }

    pub fn new_from_range(rows: Range<usize>, cols: Range<usize>) -> Self
    where
        T: Default,
    {
        let origin = Position::new(
            rows.start.try_into().unwrap(),
            cols.start.try_into().unwrap(),
        );
        let mut rows_vec = Vec::new();
        for _ in rows {
            let mut row_vec = Vec::new();
            row_vec.resize_with(cols.end - cols.start, || T::default());
            rows_vec.push(row_vec);
        }
        Self {
            tiles: rows_vec,
            origin,
        }
    }

    pub fn expand(&mut self, direction: Direction)
    where
        T: Default,
    {
        match direction {
            Direction::Up => {
                let mut new_row = Vec::new();
                new_row.resize_with(self.cols(), || T::default());
                self.tiles.insert(0, new_row);
                self.origin = self.origin.move_absolute(Direction::Up, 1);
            }
            Direction::Down => {
                let mut new_row = Vec::new();
                new_row.resize_with(self.cols(), || T::default());
                self.tiles.push(new_row);
            }
            Direction::Left => {
                for row in &mut self.tiles {
                    row.insert(0, T::default());
                }
                self.origin = self.origin.move_absolute(Direction::Left, 1);
            }
            Direction::Right => {
                for row in &mut self.tiles {
                    row.push(T::default());
                }
            }
        }
    }

    pub fn origin(&self) -> Position {
        self.origin.clone()
    }

    pub fn rows(&self) -> usize {
        self.tiles.len()
    }

    pub fn row_range(&self) -> Range<i32> {
        self.origin.row()..(self.origin.row() + self.rows() as i32)
    }
    pub fn col_range(&self) -> Range<i32> {
        self.origin.col()..(self.origin.col() + self.cols() as i32)
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.row_range()
            .flat_map(move |r| self.col_range().map(move |c| Position::new(r, c)))
    }

    pub fn tiles(&self) -> impl Iterator<Item = &T> {
        self.tiles.iter().flat_map(|r| r.iter())
    }

    pub fn tiles_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.tiles.iter_mut().flat_map(|r| r.iter_mut())
    }

    pub fn cols(&self) -> usize {
        if let Some(row) = self.tiles.first() {
            row.len()
        } else {
            0
        }
    }

    pub fn get(&self, position: &Position) -> &T {
        self.tiles
            .get(position.row() as usize)
            .unwrap()
            .get(position.col() as usize)
            .unwrap()
    }

    pub fn get_mut(&mut self, position: &Position) -> &mut T {
        self.tiles
            .get_mut(position.row() as usize)
            .unwrap()
            .get_mut(position.col() as usize)
            .unwrap()
    }

    pub fn get_mut_or_default(&mut self, position: &Position) -> &mut T where T : Default {
        while position.row() < self.row_range().start {
            self.expand(Direction::Up);
        }
        while position.row() >= self.row_range().end {
            self.expand(Direction::Down);
        }
        while position.col() < self.col_range().start {
            self.expand(Direction::Left);
        }
        while position.col() >= self.col_range().end {
            self.expand(Direction::Right);
        }
        self.get_mut(position)
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Self { tiles: value, origin: Position::new(0,0) }
    }
}
