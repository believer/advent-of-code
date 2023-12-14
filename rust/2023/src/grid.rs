use std::{
    fmt::{Display, Formatter},
    ops::{Index, IndexMut},
};

use crate::point::Point;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
    /// Find _one_ point that has the given value in the grid.
    ///
    /// # Example
    ///
    /// ```
    /// use advent_of_code_2023::grid::Grid;
    /// use advent_of_code_2023::point::Point;
    ///
    /// let data = "...#
    /// .#S.
    /// ..#.";
    ///
    /// let grid: Grid<u8> = Grid::from(data);
    /// let point = grid.find(b'S');
    ///
    /// assert_eq!(point, Some(Point { x: 2, y: 1 }))
    /// ```
    pub fn find(&self, value: T) -> Option<Point> {
        self.data
            .iter()
            .position(|&x| x == value)
            .map(|i| Point::new((i as i32) % self.width, (i as i32) / self.width))
    }

    /// Find all points that contain the given value in the grid.
    ///
    /// # Example
    ///
    /// ```
    /// use advent_of_code_2023::grid::Grid;
    ///
    /// let data = "...#
    /// .#..
    /// ..#.";
    ///
    /// let grid: Grid<u8> = Grid::from(data);
    /// let points = grid.find_all(b'#');
    ///
    /// assert_eq!(points.len(), 3);
    /// ```
    pub fn find_all(&self, value: T) -> Vec<Point> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x == value { Some(i) } else { None })
            .map(|i| Point::new((i as i32) % self.width, (i as i32) / self.width))
            .collect()
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    pub fn swap(&mut self, a: Point, b: Point) {
        let a = (self.width * a.y + a.x) as usize;
        let b = (self.width * b.y + b.x) as usize;

        self.data.swap(a, b);
    }
}

/// Used to get a value using a Point as index from the grid
///
/// ```
/// use advent_of_code_2023::grid::Grid;
/// use advent_of_code_2023::point::Point;
///
/// let grid: Grid<u8> = Grid::from(".#..");
/// assert_eq!(grid[Point::new(1, 0)], b'#');
/// ```
impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        &self.data[(self.width * point.y + point.x) as usize]
    }
}

/// Used to set a value using a Point as index on the grid
///
/// ```
/// use advent_of_code_2023::grid::Grid;
/// use advent_of_code_2023::point::Point;
///
/// let mut grid: Grid<u8> = Grid::from("....");
/// let point = Point::new(1,0);
///
/// grid[point] = b'#';
///
/// assert_eq!(grid[point], b'#');
/// ```
impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.data[(self.width * point.y + point.x) as usize]
    }
}

/// Used for debugging a grid visually.
impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[Point::new(x, y)] as char)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
