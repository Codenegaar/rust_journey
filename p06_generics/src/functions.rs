use std::ops::Add;

/// A non-generic struct which represents a point in the 2-d space
///
/// # Examples
///
/// ```
/// let point: Point = Point(12, 3);
/// ```
pub struct Point(f64, f64);

/// A generic struct to represent a point in the 2-d space
///
/// # Examples
///
/// ```
/// let point: GenPoint = GenPoint(-6, 3);
/// ```
pub struct GenPoint<T>(T, T);
impl<T: Add<Output = T>> Add for GenPoint<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self (self.0 + other.0, self.1 + other.1)
    }
}

pub struct Rectangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point
}
impl Rectangle {
    pub fn new(start_x: f64, start_y: f64, width: f64, height: f64) -> Self {
        Rectangle {
            a: Point(start_x,           start_y),
            b: Point(start_x + width,   start_y),
            c: Point(start_x + width,   start_y + height),
            d: Point(start_x,           start_y + height)
        }
    }
}

pub struct GenRectangle<T> {
    pub a: GenPoint<T>,
    pub b: GenPoint<T>,
    pub c: GenPoint<T>,
    pub d: GenPoint<T>
}
impl<T: Add<Output = T>> GenRectangle<T> {
    pub fn new(start_x: T, start_y: T, width: T, height: T) -> Self {
        GenRectangle {
            a: GenPoint(start_x,           start_y),
            b: GenPoint(start_x + width,  start_y),
            c: GenPoint(start_x + width,    start_y + height),
            d: GenPoint(start_x,            start_y + height)
        }
    }
}
