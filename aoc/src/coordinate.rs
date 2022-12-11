use std::{
    borrow::Borrow,
    fmt::Display,
    iter::Step,
    ops::{Add, Deref, DerefMut, RangeBounds, RangeInclusive, Sub},
};

use crate::position::Direction;
use crate::range::Range;

pub enum HorizontalAxisOrientation {
    PositiveRight,
    PositiveLeft,
}
pub enum VerticalAxisOrientation {
    PositiveUp,
    PositiveDown,
}

pub trait Coordinate
where
    Self: Sized + Default,
{
    type Unit: Copy + PartialOrd + 'static;

    const VERTICAL_AXIS_ORIENTATION: VerticalAxisOrientation;
    const HORIZONTAL_AXIS_ORIENTATION: HorizontalAxisOrientation;

    fn horizontal(&self) -> &Self::Unit;
    fn vertical(&self) -> &Self::Unit;

    fn from_horz_vert(horizontal: Self::Unit, vertical: Self::Unit) -> Self;

    fn project(&self, direction: Direction, distance: Self::Unit) -> Self;

    fn left(&self, distance: Self::Unit) -> Self {
        self.project(Direction::Left, distance)
    }
    fn right(&self, distance: Self::Unit) -> Self {
        self.project(Direction::Right, distance)
    }
    fn up(&self, distance: Self::Unit) -> Self {
        self.project(Direction::Up, distance)
    }
    fn down(&self, distance: Self::Unit) -> Self {
        self.project(Direction::Down, distance)
    }
}

pub trait RectangularRangeBounds<C: Coordinate> {
    fn range(self) -> RectangularRange<C>;
}

impl<'a, T, C: Coordinate + 'a> RectangularRangeBounds<C> for T
where
    T: Iterator<Item = &'a C> + 'a,
{
    fn range(self) -> RectangularRange<C> {
        self.fold(RectangularRange::empty(), |a, i| a.extended(i))
    }
}

pub struct RectangularRange<C>
where
    C: Coordinate,
{
    horizontal: HorizontalRange<C>,
    vertical: VerticalRange<C>,
}

impl<C: Coordinate> RectangularRange<C> {
    pub fn new(horizontal: HorizontalRange<C>, vertical: VerticalRange<C>) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    pub fn empty() -> Self {
        Self {
            horizontal: HorizontalRange::empty(),
            vertical: VerticalRange::empty(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.horizontal.is_empty()
    }

    pub fn horizontal(&self) -> &HorizontalRange<C> {
        &self.horizontal
    }

    pub fn vertical(&self) -> &VerticalRange<C> {
        &self.vertical
    }

    pub fn iter(&self) -> impl Iterator<Item = C> + '_
    where
        C::Unit: Step + 'static,
    {
        self.horizontal
            .iter()
            .flat_map(move |h| self.vertical.iter().map(move |v| C::from_horz_vert(h, v)))
    }

    pub fn extended(&self, point: &C) -> Self {
        if !self.is_empty() {
            Self {
                horizontal: HorizontalRange::new(self.horizontal.extended(point.horizontal())),
                vertical: VerticalRange::new(self.vertical.extended(point.vertical())),
            }
        } else {
            Self {
                horizontal: HorizontalRange::new(Range::new(
                    *point.horizontal()..=*point.horizontal(),
                )),
                vertical: VerticalRange::new(Range::new(*point.vertical()..=*point.vertical())),
            }
        }
    }

    pub fn extend(&mut self, point: &C) {
        *self = self.extended(point);
    }

    pub fn expand(&mut self, direction: Direction, distance: C::Unit) {
        match direction {
            Direction::Up => self.extend(&self.top_left().unwrap().up(distance)),
            Direction::Down => self.extend(&self.bottom_left().unwrap().down(distance)),
            Direction::Left => self.extend(&self.top_left().unwrap().left(distance)),
            Direction::Right => self.extend(&self.top_right().unwrap().right(distance)),
        }
    }

    pub fn contains(&self, point: &C) -> bool {
        self.horizontal.contains(point.horizontal()) && self.vertical.contains(point.vertical())
    }

    pub fn left(&self) -> Option<&C::Unit> {
        self.horizontal.left()
    }

    pub fn right(&self) -> Option<&C::Unit> {
        self.horizontal.right()
    }

    pub fn top(&self) -> Option<&C::Unit> {
        self.vertical.top()
    }

    pub fn bottom(&self) -> Option<&C::Unit> {
        self.vertical.bottom()
    }

    pub fn top_left(&self) -> Option<C> {
        if self.is_empty() {
            None
        } else {
            Some(C::from_horz_vert(
                *self.left().unwrap(),
                *self.top().unwrap(),
            ))
        }
    }

    pub fn top_right(&self) -> Option<C> {
        if self.is_empty() {
            None
        } else {
            Some(C::from_horz_vert(
                *self.right().unwrap(),
                *self.top().unwrap(),
            ))
        }
    }

    pub fn bottom_left(&self) -> Option<C> {
        if self.is_empty() {
            None
        } else {
            Some(C::from_horz_vert(
                *self.left().unwrap(),
                *self.bottom().unwrap(),
            ))
        }
    }

    pub fn bottom_right(&self) -> Option<C> {
        if self.is_empty() {
            None
        } else {
            Some(C::from_horz_vert(
                *self.right().unwrap(),
                *self.bottom().unwrap(),
            ))
        }
    }

    pub fn edge_positions(&self, edge: Direction) -> Box<dyn Iterator<Item = C>>
    where
        C::Unit: Step,
    {
        match edge {
            Direction::Up => {
                let v = *self.vertical.top().unwrap();
                Box::new(
                    self.horizontal
                        .iter()
                        .map(move |h| Coordinate::from_horz_vert(h, v)),
                )
            }
            Direction::Down => {
                let v = *self.vertical.bottom().unwrap();
                Box::new(
                    self.horizontal
                        .iter()
                        .map(move |h| Coordinate::from_horz_vert(h, v)),
                )
            }
            Direction::Left => {
                let h = *self.horizontal.left().unwrap();
                Box::new(
                    self.vertical
                        .iter()
                        .map(move |v| Coordinate::from_horz_vert(h, v)),
                )
            }
            Direction::Right => {
                let h = *self.horizontal.right().unwrap();
                Box::new(
                    self.vertical
                        .iter()
                        .map(move |v| Coordinate::from_horz_vert(h, v)),
                )
            }
        }
    }
}

pub struct HorizontalRange<C>
where
    C: Coordinate,
{
    range: Range<C::Unit>,
}

impl<C> Deref for HorizontalRange<C>
where
    C: Coordinate,
{
    type Target = Range<C::Unit>;

    fn deref(&self) -> &Self::Target {
        &self.range
    }
}

impl<C> DerefMut for HorizontalRange<C>
where
    C: Coordinate,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.range
    }
}

impl<C> HorizontalRange<C>
where
    C: Coordinate,
{
    pub fn new(range: Range<C::Unit>) -> Self {
        Self { range }
    }
    pub fn empty() -> Self {
        Self {
            range: Range::empty(),
        }
    }
    pub fn left(&self) -> Option<&C::Unit> {
        match C::HORIZONTAL_AXIS_ORIENTATION {
            HorizontalAxisOrientation::PositiveRight => self.range.start(),
            HorizontalAxisOrientation::PositiveLeft => self.range.end(),
        }
    }
    pub fn right(&self) -> Option<&C::Unit> {
        match C::HORIZONTAL_AXIS_ORIENTATION {
            HorizontalAxisOrientation::PositiveRight => self.range.end(),
            HorizontalAxisOrientation::PositiveLeft => self.range.start(),
        }
    }
}

pub struct VerticalRange<C>
where
    C: Coordinate,
{
    range: Range<C::Unit>,
}

impl<C> Deref for VerticalRange<C>
where
    C: Coordinate,
{
    type Target = Range<C::Unit>;

    fn deref(&self) -> &Self::Target {
        &self.range
    }
}

impl<C> DerefMut for VerticalRange<C>
where
    C: Coordinate,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.range
    }
}

impl<C> VerticalRange<C>
where
    C: Coordinate,
{
    pub fn new(range: Range<C::Unit>) -> Self {
        Self { range }
    }
    pub fn empty() -> Self {
        Self {
            range: Range::empty(),
        }
    }
    pub fn top(&self) -> Option<&C::Unit> {
        match C::VERTICAL_AXIS_ORIENTATION {
            VerticalAxisOrientation::PositiveUp => self.range.end(),
            VerticalAxisOrientation::PositiveDown => self.range.start(),
        }
    }
    pub fn bottom(&self) -> Option<&C::Unit> {
        match C::VERTICAL_AXIS_ORIENTATION {
            VerticalAxisOrientation::PositiveUp => self.range.start(),
            VerticalAxisOrientation::PositiveDown => self.range.end(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct XY<U = i32> {
    x: U,
    y: U,
}

impl<U> XY<U> {
    pub fn new(x: U, y: U) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> &U {
        &self.x
    }
    pub fn y(&self) -> &U {
        &self.y
    }
}

impl<U: Display> Display for XY<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<x={},y={}>", self.x, self.y))
    }
}

impl<U: Add<U, Output = U> + Sub<U, Output = U> + Default + Copy + PartialOrd + 'static> Coordinate
    for XY<U>
{
    type Unit = U;

    fn project(&self, direction: Direction, distance: Self::Unit) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y + distance,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y - distance,
            },
            Direction::Left => Self {
                x: self.x - distance,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + distance,
                y: self.y + distance,
            },
        }
    }

    fn from_horz_vert(horizontal: Self::Unit, vertical: Self::Unit) -> Self {
        Self {
            x: horizontal,
            y: vertical,
        }
    }

    const VERTICAL_AXIS_ORIENTATION: VerticalAxisOrientation = VerticalAxisOrientation::PositiveUp;

    const HORIZONTAL_AXIS_ORIENTATION: HorizontalAxisOrientation =
        HorizontalAxisOrientation::PositiveRight;

    fn horizontal(&self) -> &Self::Unit {
        &self.x
    }

    fn vertical(&self) -> &Self::Unit {
        &self.y
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RowCol<U = i32, const POSITIVE_RIGHT: bool = true, const POSITIVE_UP: bool = false> {
    row: U,
    col: U,
}

impl<U> RowCol<U> {
    pub fn new(row: U, col: U) -> Self {
        Self { row, col }
    }
}

impl<U: Display> Display for RowCol<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<r={},c={}>", self.row, self.col))
    }
}

impl<
        U: Add<U, Output = U> + Sub<U, Output = U> + Default + Copy + PartialOrd + 'static,
        const PR: bool,
        const PU: bool,
    > Coordinate for RowCol<U, PR, PU>
{
    type Unit = U;

    fn project(&self, direction: Direction, distance: Self::Unit) -> Self {
        match direction {
            Direction::Up => Self {
                row: if PU {
                    self.row + distance
                } else {
                    self.row - distance
                },
                col: self.col,
            },
            Direction::Down => Self {
                row: if PU {
                    self.row - distance
                } else {
                    self.row + distance
                },
                col: self.col,
            },
            Direction::Left => Self {
                row: self.row,
                col: if PR {
                    self.col - distance
                } else {
                    self.col + distance
                },
            },
            Direction::Right => Self {
                row: self.row,
                col: if PR {
                    self.col + distance
                } else {
                    self.col - distance
                },
            },
        }
    }

    fn from_horz_vert(horizontal: Self::Unit, vertical: Self::Unit) -> Self {
        Self {
            row: vertical,
            col: horizontal,
        }
    }

    const VERTICAL_AXIS_ORIENTATION: VerticalAxisOrientation = if PU {
        VerticalAxisOrientation::PositiveUp
    } else {
        VerticalAxisOrientation::PositiveDown
    };

    const HORIZONTAL_AXIS_ORIENTATION: HorizontalAxisOrientation = if PR {
        HorizontalAxisOrientation::PositiveRight
    } else {
        HorizontalAxisOrientation::PositiveLeft
    };

    fn horizontal(&self) -> &Self::Unit {
        &self.col
    }

    fn vertical(&self) -> &Self::Unit {
        &self.row
    }
}
