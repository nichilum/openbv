pub fn graham_scan(points: &[(u32, u32)]) -> Vec<(u32, u32)> {
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

fn orientation(p: (u32, u32), q: (u32, u32), r: (u32, u32)) -> i32 {
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

pub fn approx_hull(points: &[(u32, u32)], epsilon: f32) -> Vec<(u32, u32)> {
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
    let (x1, y1) = line.0;
    let (x2, y2) = line.1;
    let (x0, y0) = *point;

    let num = ((y2 as i32 - y1 as i32) * x0 as i32 - (x2 as i32 - x1 as i32) * y0 as i32
        + x2 as i32 * y1 as i32
        - y2 as i32 * x1 as i32)
        .abs() as f32;
    let den = (((y2 as i32 - y1 as i32).pow(2) + (x2 as i32 - x1 as i32).pow(2)) as f32).sqrt();

    num / den
}
