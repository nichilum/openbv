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

    pub fn contains(&self, point: Point) -> bool {
        false
    }
}

impl Hull for ConvexHull {
    fn get_center(&self) -> Point {
        todo!()
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
