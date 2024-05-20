use crate::math::point::Point;

use super::hull::Hull;

#[derive(Debug)]
pub struct ConvexHull {
    points: Vec<Point>,
}

impl ConvexHull {
    pub fn new(points: &[Point]) -> Self {
        let hull = graham_scan(points);
        ConvexHull { points: hull }
    }

    // https://rosettacode.org/wiki/Ray-casting_algorithm
    // https://en.wikipedia.org/wiki/Point_in_polygon
    pub fn contains(&self, point: Point) -> bool {
        let mut count = 0;
        let length = self.points.len();
        for i in 0..length {
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % length];

            if p1.y == p2.y {
                continue;
            }

            if point.y < p1.y.min(p2.y) || point.y >= p1.y.max(p2.y) {
                continue;
            }

            let x = (point.y as i32 - p1.y as i32) as f32 * (p2.x as i32 - p1.x as i32) as f32
                / (p2.y as i32 - p1.y as i32) as f32
                + p1.x as f32;

            if x > point.x as f32 {
                count += 1;
            }
        }

        count % 2 == 1
    }
}

impl Hull for ConvexHull {
    // doesn't works as well here
    fn get_center(&self) -> Point {
        let mut sum_x = 0;
        let mut sum_y = 0;
        for Point { x, y } in &self.points {
            sum_x += x;
            sum_y += y;
        }

        Point::new(
            sum_x / self.points.len() as u32,
            sum_y / self.points.len() as u32,
        )
    }

    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}

fn graham_scan(points: &[Point]) -> Vec<Point> {
    let mut stack = Vec::new();
    stack.push(points[0]);
    stack.push(points[1]);

    let mut ind = 2u32;
    let mut top = 1u32;
    let length = points.len() as u32;

    while ind < length {
        while top > 0
            && orientation(
                stack[top as usize - 1],
                stack[top as usize],
                points[ind as usize],
            ) != 2
        {
            stack.pop();
            top -= 1;
        }

        top += 1;
        stack.push(points[ind as usize]);
        ind += 1;
    }

    stack
}

fn orientation(p: Point, q: Point, r: Point) -> u8 {
    let val = (q.y as i32 - p.y as i32) * (r.x as i32 - q.x as i32)
        - (q.x as i32 - p.x as i32) * (r.y as i32 - q.y as i32);

    if val == 0 {
        return 0;
    }

    if val > 0 {
        return 1;
    }

    2
}
