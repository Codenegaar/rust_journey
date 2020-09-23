mod shape;
mod point;

use shape::Shape;
use point::Point;

fn main() {
    let mut shape: Shape = Shape::new("Test".to_string());
    shape.point(Point{x: 0, y: 0});
    shape.point(Point{x: 0, y: 2});
    shape.point(Point{x: 2, y: 0});
    shape.point(Point{x: 2, y: 2});

    println!("{}", shape);
}
