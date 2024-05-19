use super::hull::Hull;

#[derive(Debug)]
pub struct PolyHull {
    points: Vec<(u32, u32)>,
}
impl PolyHull {
    pub fn new(points: &[(u32, u32)], epsilon: f32) -> Self {
        let hull = approx_hull(points, epsilon);
        PolyHull { points: hull }
    }
}

impl Hull for PolyHull {
    // works better for poly hulls than convex hulls
    fn get_center(&self) -> (u32, u32) {
        let mut sum_x = 0;
        let mut sum_y = 0;
        for (x, y) in &self.points {
            sum_x += x;
            sum_y += y;
        }

        (
            sum_x / self.points.len() as u32,
            sum_y / self.points.len() as u32,
        )
    }

    fn get_points(&self) -> &Vec<(u32, u32)> {
        &self.points
    }
}

fn approx_hull(points: &[(u32, u32)], epsilon: f32) -> Vec<(u32, u32)> {
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
fn distance(point: &(u32, u32), line: &((u32, u32), (u32, u32))) -> f32 {
    let (x1, y1) = (line.0 .0 as f32, line.0 .1 as f32);
    let (x2, y2) = (line.1 .0 as f32, line.1 .1 as f32);
    let (x0, y0) = (point.0 as f32, point.1 as f32);

    let num = ((x2 - x1) * (y0 - y1) - (x0 - x1) * (y2 - y1)).abs();
    let den = (((y2 as i32 - y1 as i32).pow(2) + (x2 as i32 - x1 as i32).pow(2)) as f32).sqrt();

    num / den
}
