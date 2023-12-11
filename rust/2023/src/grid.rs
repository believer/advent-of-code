use std::{
    fmt::{Display, Formatter},
    ops::{Index, IndexMut},
};

use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub data: Vec<T>,
}

impl<T: From<u8> + Copy> From<&str> for Grid<T> {
    fn from(value: &str) -> Self {
        let raw_data = value
            .lines()
            .flat_map(|line| line.bytes().filter_map(|b| T::from(b).into()))
            .collect::<Vec<_>>();
        let width = value.lines().next().unwrap_or_default().len() as i32;
        let height = value.lines().count() as i32;

        Grid {
            width,
            height,
            data: raw_data,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn find(&self, value: T) -> Option<Point> {
        self.data
            .iter()
            .position(|&x| x == value)
            .map(|i| Point::new((i as i32) % self.width, (i as i32) / self.width))
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        &self.data[(self.width * point.y + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.data[(self.width * point.y + point.x) as usize]
    }
}

/// Used for debugging a grid visually.
impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[Point::new(x, y)])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
