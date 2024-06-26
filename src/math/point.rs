#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Self {
        Point { x, y }
    }
}

impl Point {
    pub const ZERO: Point = Point { x: 0, y: 0 };

    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl From<Point> for [u32; 2] {
    fn from(value: Point) -> Self {
        [value.x, value.y]
    }
}
impl From<Point> for [f64; 2] {
    fn from(value: Point) -> Self {
        [value.x as f64, value.y as f64]
    }
}
