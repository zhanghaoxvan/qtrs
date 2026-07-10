//! 2D point representation for widget positioning.
//!
//! This module provides a Rust wrapper around Qt's `QPoint` class,
//! used for specifying positions in 2D space.

use crate::ffi;

/// A 2D point with integer coordinates.
///
/// `Point` is used throughout the library for widget positioning,
/// mouse events, and other geometric operations.
///
/// # Examples
///
/// ```
/// use qtrs::Point;
///
/// let pos = Point::new(100, 200);
/// assert_eq!(pos.x, 100);
/// assert_eq!(pos.y, 200);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    /// X coordinate (horizontal position).
    pub x: i32,
    /// Y coordinate (vertical position).
    pub y: i32,
}

impl Point {
    /// Creates a new point at `(x, y)`.
    ///
    /// # Parameters
    ///
    /// * `x` — Horizontal position in pixels
    /// * `y` — Vertical position in pixels
    ///
    /// # Examples
    ///
    /// ```
    /// use qtrs::Point;
    ///
    /// let origin = Point::new(0, 0);
    /// let top_left = Point::new(10, 20);
    /// ```
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Creates a point at the origin `(0, 0)`.
    ///
    /// This is equivalent to `Point::new(0, 0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use qtrs::Point;
    ///
    /// let origin = Point::origin();
    /// assert_eq!(origin.x, 0);
    /// assert_eq!(origin.y, 0);
    /// ```
    #[inline]
    pub const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Returns the X coordinate.
    #[inline]
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Returns the Y coordinate.
    #[inline]
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Returns a tuple `(x, y)` for convenient destructuring.
    ///
    /// # Examples
    ///
    /// ```
    /// use qtrs::Point;
    ///
    /// let pos = Point::new(10, 20);
    /// let (x, y) = pos.into_tuple();
    /// assert_eq!(x, 10);
    /// assert_eq!(y, 20);
    /// ```
    #[inline]
    pub fn into_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Creates a Point from a tuple `(x, y)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use qtrs::Point;
    ///
    /// let pos = Point::from_tuple((30, 40));
    /// assert_eq!(pos.x, 30);
    /// assert_eq!(pos.y, 40);
    /// ```
    #[inline]
    pub fn from_tuple((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }

    /// Converts this point to a raw C++ `QPoint` pointer.
    ///
    /// This is used internally by the library and should not be called directly.
    #[doc(hidden)]
    pub(crate) unsafe fn to_raw(&self) -> *mut ffi::QPoint {
        ffi::QPoint_new(self.x, self.y)
    }

    /// Checks if this point is at the origin `(0, 0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use qtrs::Point;
    ///
    /// let pos = Point::new(0, 0);
    /// assert!(pos.is_origin());
    ///
    /// let pos = Point::new(5, 0);
    /// assert!(!pos.is_origin());
    /// ```
    #[inline]
    pub fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

// ─── From conversions ─────────────────────────────────────────

impl From<(i32, i32)> for Point {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (i32, i32) {
    #[inline]
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

// ─── Arithmetic operators ────────────────────────────────────

impl std::ops::Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// ─── String formatting ────────────────────────────────────────

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

// ─── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Point::new(10, 20);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
    }

    #[test]
    fn test_origin() {
        let p = Point::origin();
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
        assert!(p.is_origin());
    }

    #[test]
    fn test_from_tuple() {
        let p = Point::from((30, 40));
        assert_eq!(p.x, 30);
        assert_eq!(p.y, 40);
    }

    #[test]
    fn test_into_tuple() {
        let p = Point::new(50, 60);
        let (x, y) = p.into_tuple();
        assert_eq!(x, 50);
        assert_eq!(y, 60);
    }

    #[test]
    fn test_add() {
        let a = Point::new(1, 2);
        let b = Point::new(3, 4);
        let c = a + b;
        assert_eq!(c.x, 4);
        assert_eq!(c.y, 6);
    }

    #[test]
    fn test_sub() {
        let a = Point::new(5, 7);
        let b = Point::new(2, 3);
        let c = a - b;
        assert_eq!(c.x, 3);
        assert_eq!(c.y, 4);
    }

    #[test]
    fn test_display() {
        let p = Point::new(10, 20);
        assert_eq!(format!("{}", p), "Point(10, 20)");
    }
}