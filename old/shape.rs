
use std::f64;

struct Point {
    x: f64,
    y: f64
}

enum Shape {
    Circle(Point, f64),
    Rectangle(Point, Point)
}

fn area(sh: Shape) -> f64 {
    match sh {
        Circle(_, size) => f64::consts::PI * size * size,
        Rectangle(Point { x, y }, Point { x: x2, y: y2 }) => (x2 - x) * (y2 - y)
    }
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Circle(_, r) => f64::consts::PI * r * r,
            Rectangle(Point { x, y }, Point { x: x2, y: y2 }) => (x2 - x) * (y2 - y)
        }
    }
}

fn main() {
  let s = Circle(Point { x: 1.0, y: 2.0 }, 3.0);
  let p = Point{x:1.0, y:2.0};
  let x = Circle(p, 3.0);
  let a = area(x);
  println!("{}, {}", a, s.area());
  println!("end");
}
