use std::{collections::HashMap, hash::Hash};

use crate::coordinate::{Coordinate, RectangularRange, RectangularRangeBounds};

pub type TileMap<T, C> = HashMap<C, T>;

trait TileMapTrait {
    type Coordinate: Coordinate;

    fn range(&self) -> RectangularRange<Self::Coordinate>;
}

impl<T, C: Coordinate> TileMapTrait for TileMap<T, C> {
    type Coordinate = C;

    fn range(&self) -> RectangularRange<Self::Coordinate> {
        self.keys().range()
    }
}
