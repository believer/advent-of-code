use std::ops::Index;

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
