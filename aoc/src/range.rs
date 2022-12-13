use std::{
    iter::Step,
    ops::{RangeBounds, RangeInclusive, Sub},
};

#[derive(Clone, Eq, PartialEq)]
pub struct Range<T> {
    range: Option<RangeInclusive<T>>,
}

impl<T: Clone + PartialOrd> Range<T> {
    pub fn new(range: RangeInclusive<T>) -> Self {
        Self { range: Some(range) }
    }
    pub fn empty() -> Self {
        Self { range: None }
    }
    pub fn extended(&self, point: &T) -> Self
    where
        T: Copy,
    {
        if let Some(range) = &self.range {
            if *point < *range.start() {
                Self::new(*point..=*range.end())
            } else if point > range.end() {
                Self::new(*range.start()..=*point)
            } else {
                self.clone()
            }
        } else {
            Self::new(*point..=*point)
        }
    }
    pub fn extend(&mut self, point: &T)
    where
        T: Copy,
    {
        *self = self.extended(point);
    }

    pub fn iter(&self) -> Box<dyn DoubleEndedIterator<Item = T>>
    where
        T: Step + 'static,
    {
        let iterator: Box<dyn DoubleEndedIterator<Item = T>> = if let Some(range) = &self.range {
            Box::new(range.clone())
        } else {
            Box::new(std::iter::empty())
        };
        iterator
    }

    pub fn contains(&self, point: &T) -> bool {
        if let Some(r) = &self.range {
            r.contains(point)
        } else {
            false
        }
    }

    pub fn start(&self) -> Option<&T> {
        self.range.as_ref().map(|r| r.start())
    }
    pub fn end(&self) -> Option<&T> {
        self.range.as_ref().map(|r| r.end())
    }
    pub fn count(&self) -> usize
    where
        T: Sub + Copy,
        usize: From<<T as Sub>::Output>,
    {
        if let Some(r) = &self.range {
            <<T as std::ops::Sub>::Output as Into<usize>>::into(*r.end() - *r.start()) + 1
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.range.is_none()
    }
}
