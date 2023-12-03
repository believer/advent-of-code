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

use std::ops::Add;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// The sum of the absolute values of two points
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

// Common points and directions. Useful for looking
// around a point or moving in a direction
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const TOP_LEFT: Point = Point::new(-1, -1);
pub const TOP_RIGHT: Point = Point::new(1, -1);
pub const BOTTOM_LEFT: Point = Point::new(-1, 1);
pub const BOTTOM_RIGHT: Point = Point::new(1, 1);

pub const DIAGONALS: [Point; 8] =
    [
        TOP_LEFT,
        UP,
        TOP_RIGHT,
        LEFT,
        RIGHT,
        BOTTOM_LEFT,
        DOWN,
        BOTTOM_RIGHT,
    ];
