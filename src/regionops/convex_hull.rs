use super::hull::Hull;

#[derive(Debug)]
pub struct ConvexHull {
    points: Vec<(u32, u32)>,
}
impl ConvexHull {
    pub fn new(points: &[(u32, u32)]) -> Self {
        let hull = graham_scan(points);
        ConvexHull { points: hull }
    }

    pub fn contains(&self, x: u32, y: u32) -> bool {
        false
    }
}

impl Hull for ConvexHull {
    fn get_center(&self) -> (u32, u32) {
        todo!()
    }

    fn get_points(&self) -> &Vec<(u32, u32)> {
        &self.points
    }
}

fn graham_scan(points: &[(u32, u32)]) -> Vec<(u32, u32)> {
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

fn orientation(p: (u32, u32), q: (u32, u32), r: (u32, u32)) -> u8 {
    let val = (q.1 as i32 - p.1 as i32) * (r.0 as i32 - q.0 as i32)
        - (q.0 as i32 - p.0 as i32) * (r.1 as i32 - q.1 as i32);

    if val == 0 {
        return 0;
    }

    if val > 0 {
        return 1;
    }

    2
}
