use crate::math::point::Point;

use super::hull::Hull;

#[derive(Debug)]
pub struct PolyHull {
    points: Vec<Point>,
}
impl PolyHull {
    pub fn new(points: &[Point], epsilon: f32) -> Self {
        let hull = approx_hull(points, epsilon);
        PolyHull { points: hull }
    }
}

impl Hull for PolyHull {
    // works better for poly hulls than convex hulls
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

fn approx_hull(points: &[Point], epsilon: f32) -> Vec<Point> {
    let mut d_max = 0.;
    let mut index_max = -1;
    let line = (points[0], points[points.len() - 1]);

    for (i, point) in points.iter().enumerate().skip(2) {
        let d = distance(point, &line);
        if d > d_max {
            index_max = i as i32;
            d_max = d;
        }
    }

    if d_max > epsilon {
        let left = approx_hull(&points[..index_max as usize], epsilon);
        let right = approx_hull(&points[index_max as usize..], epsilon);

        let mut hull = Vec::new();
        hull.extend(left);
        hull.extend(right);

        hull
    } else {
        vec![points[0], points[points.len() - 1]]
    }
}

/// perpendicular distance from a point to a line
fn distance(point: &Point, line: &(Point, Point)) -> f32 {
    let (x1, y1) = (line.0.x as f32, line.0.y as f32);
    let (x2, y2) = (line.1.x as f32, line.1.y as f32);
    let (x0, y0) = (point.x as f32, point.y as f32);

    let num = ((x2 - x1) * (y0 - y1) - (x0 - x1) * (y2 - y1)).abs();
    let den = (((y2 as i32 - y1 as i32).pow(2) + (x2 as i32 - x1 as i32).pow(2)) as f32).sqrt();

    num / den
}
