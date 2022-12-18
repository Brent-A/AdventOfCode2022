use std::{
    iter::Step,
    ops::{RangeInclusive, Sub},
};

#[derive(Debug, Clone, Eq, PartialEq)]
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

    pub fn union(&self, other: &Self) -> Vec<Self>
    where
        T: Step,
    {
        match (&self.range, &other.range) {
            (None, None) => vec![],
            (None, Some(_o)) => vec![other.clone()],
            (Some(_s), None) => vec![self.clone()],
            (Some(_s), Some(_o)) => {
                let self_start = self.start().unwrap().clone();
                let self_end = self.end().unwrap().clone();
                let other_start = other.start().unwrap().clone();
                let other_end = other.end().unwrap().clone();

                if self_start < other_start {
                    if self_end < other_start {
                        if Step::forward(self_end, 1) == other_start {
                            vec![Self {
                                range: Some(self_start..=other_end),
                            }]
                        } else {
                            vec![self.clone(), other.clone()]
                        }
                    } else if self_end < other_end {
                        vec![Self {
                            range: Some(self_start..=other_end),
                        }]
                    } else {
                        vec![Self {
                            range: Some(self_start..=self_end),
                        }]
                    }
                } else if self_start < other_end {
                    if self_end < other_end {
                        vec![Self {
                            range: Some(other_start..=other_end),
                        }]
                    } else {
                        vec![Self {
                            range: Some(other_start..=self_end),
                        }]
                    }
                } else {
                    vec![self.clone(), other.clone()]
                }
            }
        }
    }

    pub fn intersect(&self, other: &Self) -> Self {
        if self.is_empty() || other.is_empty() {
            Self::empty()
        } else {
            let self_start = self.start().unwrap().clone();
            let self_end = self.end().unwrap().clone();
            let other_start = other.start().unwrap().clone();
            let other_end = other.end().unwrap().clone();

            if self_start < other_start {
                if self_end < other_start {
                    Self { range: None }
                } else if self_end < other_end {
                    Self {
                        range: Some(other_start..=self_end),
                    }
                } else {
                    Self {
                        range: Some(other_start..=other_end),
                    }
                }
            } else if self_start < other_end {
                if self_end < other_end {
                    Self {
                        range: Some(self_start..=self_end),
                    }
                } else {
                    Self {
                        range: Some(self_start..=other_end),
                    }
                }
            } else {
                Self { range: None }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.range.is_none()
    }
}

#[cfg(test)]
mod test {
    use super::Range;

    #[test]
    fn intersect() {
        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(0..=2)),
            Range::new(0..=2)
        );

        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(-1..=2)),
            Range::new(0..=2)
        );
        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(0..=3)),
            Range::new(0..=2)
        );
        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(1..=2)),
            Range::new(1..=2)
        );
        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(0..=1)),
            Range::new(0..=1)
        );
        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(-1..=3)),
            Range::new(0..=2)
        );
        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(1..=1)),
            Range::new(1..=1)
        );

        assert_eq!(
            Range::new(-1..=2).intersect(&Range::new(0..=2)),
            Range::new(0..=2)
        );
        assert_eq!(
            Range::new(0..=3).intersect(&Range::new(0..=2)),
            Range::new(0..=2)
        );
        assert_eq!(
            Range::new(1..=2).intersect(&Range::new(0..=2)),
            Range::new(1..=2)
        );
        assert_eq!(
            Range::new(0..=1).intersect(&Range::new(0..=2)),
            Range::new(0..=1)
        );

        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(2..=3)),
            Range::new(2..=2)
        );
        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(3..=4)),
            Range::empty()
        );
        assert_eq!(
            Range::new(0..=2).intersect(&Range::new(-2..=-1)),
            Range::empty()
        );
    }

    #[test]
    fn union() {
        assert_eq!(
            Range::new(0..=2).union(&Range::new(0..=2)),
            [Range::new(0..=2)]
        );

        assert_eq!(
            Range::new(0..=2).union(&Range::new(-1..=2)),
            [Range::new(-1..=2)]
        );
        assert_eq!(
            Range::new(0..=2).union(&Range::new(0..=3)),
            [Range::new(0..=3)]
        );
        assert_eq!(
            Range::new(0..=2).union(&Range::new(1..=2)),
            [Range::new(0..=2)]
        );
        assert_eq!(
            Range::new(0..=2).union(&Range::new(0..=1)),
            [Range::new(0..=2)]
        );
        assert_eq!(
            Range::new(0..=2).union(&Range::new(-1..=3)),
            [Range::new(-1..=3)]
        );
        assert_eq!(
            Range::new(0..=2).union(&Range::new(1..=1)),
            [Range::new(0..=2)]
        );

        assert_eq!(
            Range::new(0..=2).union(&Range::new(2..=3)),
            [Range::new(0..=3)]
        );
        assert_eq!(
            Range::new(0..=2).union(&Range::new(3..=4)),
            [Range::new(0..=4)]
        );
        assert_eq!(
            Range::new(0..=2).union(&Range::new(-2..=-1)),
            [Range::new(-2..=2)]
        );
    }
}
