use std::{collections::HashMap, hash::Hash, iter::Step, ops::Range};

use crate::{
    coordinate::{Coordinate, RectangularRange, RowCol, XY},
    position::{Direction, Position},
    GetOrDefault,
};

pub struct Grid<T, C = XY>
where
    C: Coordinate,
    C::Unit: Step,
{
    tiles: HashMap<C, T>,
    range: RectangularRange<C>,
}

impl<T, C> Grid<T, C>
where
    C: Coordinate + Hash + Eq + Clone,
    C::Unit: Step,
{
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            range: RectangularRange::empty(),
        }
    }

    pub fn new_from_range(range: crate::coordinate::RectangularRange<C>) -> Self {
        Self {
            tiles: HashMap::new(),
            range,
        }
    }

    pub fn range(&self) -> &RectangularRange<C> {
        &self.range
    }

    pub fn range_mut(&mut self) -> &mut RectangularRange<C> {
        &mut self.range
    }

    pub fn tiles(&self) -> impl Iterator<Item = &T> {
        self.tiles.values()
    }

    pub fn tiles_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.tiles.values_mut()
    }

    pub fn enumerate_tiles(&self) -> impl Iterator<Item = (C, Option<&T>)> {
        self.range.iter().map(|c| {
            let t = self.get(&c);
            (c, t)
        })
    }
    pub fn enumerate_tiles_mut(&mut self) -> impl Iterator<Item = (&C, &mut T)>
    where
        T: Default,
    {
        for c in self.range.iter() {
            if !self.tiles.contains_key(&c) {
                self.tiles.insert(c, T::default());
            }
        }
        self.tiles.iter_mut()
    }

    pub fn get(&self, position: &C) -> Option<&T> {
        self.tiles.get(position)
    }

    pub fn get_mut(&mut self, position: &C) -> Option<&mut T> {
        self.tiles.get_mut(position)
    }

    pub fn get_mut_or_default(&mut self, position: &C) -> &mut T
    where
        T: Default,
    {
        self.range.extend(position);
        self.tiles.get_mut_or_default(position)
    }

    pub fn insert(&mut self, position: C, tile: T) -> Option<T> {
        self.range.extend(&position);
        self.tiles.insert(position, tile)
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T, RowCol> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let mut grid = Self::new();
        let mut r = 0;
        for row in value {
            let mut c = 0;
            for value in row {
                let coordinate = RowCol::new(r as i32, c as i32);
                grid.insert(coordinate, value);
                c += 1;
            }
            r += 1;
        }
        grid
    }
}
