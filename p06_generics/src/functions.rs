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

