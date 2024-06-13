use super::point::Point;
use rand::seq::SliceRandom;
use smallest_enclosing_circle::{smallest_enclosing_circle, Circle};

pub fn ransac_circle_2d(data: &[Point], k: u32) -> Circle {
    let mut rand = rand::thread_rng();
    let mut best_circle_sum = 0;
    let mut best_circle = Circle::None;
    for _ in 0..k {
        // select three random points
        let p1 = *data.choose(&mut rand).unwrap();
        let p2 = *data.choose(&mut rand).unwrap();
        let p3 = *data.choose(&mut rand).unwrap();

        let points: Vec<[f64; 2]> = Vec::from([p1.into(), p2.into(), p3.into()]);

        // create circle
        let circle = smallest_enclosing_circle(points.into_iter());

        // count points near circle
        let mut sum = 0;
        data.iter().for_each(|point| {
            let center = circle.center().unwrap();
            let distance = (((point.x as f64 - center[0]).powi(2)
                + (point.y as f64 - center[1]).powi(2))
            .sqrt()
                - circle.radius())
            .abs();

            // ähm 10 gut??
            if distance < 10. {
                sum += 1;
            }
        });

        if sum > best_circle_sum {
            best_circle = circle;
            best_circle_sum = sum;
        }
    }

    best_circle
}
