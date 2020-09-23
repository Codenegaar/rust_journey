use crate::point::Point;
use core::fmt;

pub struct Shape {
    name: String,
    points: Vec<Point>
}

impl Shape {
    pub fn new(name: String) -> Self {
        Shape {
            name,
            points: Vec::new()
        }
    }

    pub fn point(&mut self, point: Point) -> &mut Self {
        self.points.push(point);
        self
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for point in self.points.iter() {
            write!(f, "{}\n", point);
        }

        Ok(())
    }
}