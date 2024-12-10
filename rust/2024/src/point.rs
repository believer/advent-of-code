//! Helper functions for working with points since
//! 2D operations are a common theme in Advent of Code
//!
//! ```
//! use advent_of_code_2023::point::Point;
//!
//! let a = Point::new(1, 2);
//! let b = Point::new(3, 4);
//!
//! assert_eq!(a + b, Point::new(4, 6));
//!
//! ```

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }

    pub fn counter_clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    /// The sum of the absolute values of two points
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

/// Make it possible to sum two points
impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

/// Make it possible to += two points
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Point { x, y } = self;
        write!(f, "({}, {})", x, y)
    }
}

// Common points and directions. Useful for looking
// around a point or moving in a direction
pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const TOP_LEFT: Point = Point::new(-1, -1);
pub const TOP_RIGHT: Point = Point::new(1, -1);
pub const BOTTOM_LEFT: Point = Point::new(-1, 1);
pub const BOTTOM_RIGHT: Point = Point::new(1, 1);

pub const CARDINALS: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
pub const ALL_DIRECTIONS: [Point; 8] = [
    TOP_LEFT,
    UP,
    TOP_RIGHT,
    LEFT,
    RIGHT,
    BOTTOM_LEFT,
    DOWN,
    BOTTOM_RIGHT,
];
