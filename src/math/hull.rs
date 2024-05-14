pub fn graham_scan(points: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut stack = Vec::new();
    stack.push(points[0]);
    stack.push(points[1]);

    for i in 2..points.len() {
        let mut top = stack.pop().unwrap();
        let mut next_to_top = stack.pop().unwrap();

        while orientation(next_to_top, top, points[i]) != 2 {
            top = next_to_top;
            next_to_top = stack.pop().unwrap();
        }

        stack.push(next_to_top);
        stack.push(top);
        stack.push(points[i]);
    }

    stack
}

fn orientation(p: (u32, u32), q: (u32, u32), r: (u32, u32)) -> i32 {
    let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1);

    if val == 0 {
        return 0;
    }

    if val > 0 {
        return 1;
    }

    2
}
