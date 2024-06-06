use std::f32::consts::PI;

use image::Rgb;

use rand::{self, Rng};

use super::point::Point;

pub fn k_means_hsv(
    image: &image::ImageBuffer<Rgb<f32>, Vec<f32>>,
    k: usize,
    iterations: usize,
) -> (Vec<Rgb<f32>>, Vec<Vec<Point>>) {
    let mut centers = Vec::<Rgb<f32>>::with_capacity(k);
    let mut clusters: Vec<Vec<Point>> = vec![Vec::new(); centers.len()];

    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let h = rng.gen_range(0f32..360f32);
        let s = rng.gen_range(0f32..1f32);
        let v = rng.gen_range(0f32..1f32);
        centers.push(Rgb([h, s, v]));
    }

    let mut converged = false;
    let mut iter_count = 0;
    while !converged {
        // ability to break early
        if iter_count == iterations && iterations > 0 {
            break;
        }
        iter_count += 1;

        clusters = vec![Vec::new(); centers.len()];

        image.enumerate_pixels().for_each(|(x, y, pixel)| {
            let nearest_center_index = centers
                .iter()
                .map(|&center| {
                    (pixel[0] - center[0]).powi(2)
                        + (pixel[1] - center[1]).powi(2)
                        + (pixel[2] - center[2]).powi(2)
                })
                .enumerate()
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(index, _)| index)
                .expect("one center should be the nearest");

            clusters[nearest_center_index].push(Point { x, y });
        });

        let new_centers = clusters
            .iter()
            .enumerate()
            .map(|(index, cluster)| {
                // if cluster is empty, return the same center
                if cluster.is_empty() {
                    return centers[index];
                }

                let mut x = 0.;
                let mut y = 0.;
                for point in cluster {
                    let hue = image.get_pixel(point.x, point.y)[0];
                    x += (hue / 180. * PI).cos();
                    y += (hue / 180. * PI).sin();
                }
                x /= cluster.len() as f32;
                y /= cluster.len() as f32;
                let mut h = y.atan2(x) * 180. / PI;

                if h < 0. {
                    h += 360.;
                }

                let s = cluster
                    .iter()
                    .map(|p| image.get_pixel(p.x, p.y)[1])
                    .sum::<f32>()
                    / cluster.len() as f32;
                let v = cluster
                    .iter()
                    .map(|p| image.get_pixel(p.x, p.y)[2])
                    .sum::<f32>()
                    / cluster.len() as f32;
                Rgb([h, s, v])
            })
            .collect::<Vec<_>>();

        converged = centers
            .iter()
            .zip(new_centers.iter())
            .all(|(c1, c2)| c1 == c2);
        centers = new_centers;
    }

    (centers, clusters)
}
