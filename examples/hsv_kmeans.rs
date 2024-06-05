use std::f32::consts::PI;

use image::{GenericImage, Rgb};
use openbv::{colorops::hsv::HsvExt, math::point::Point, open_rgb};

use rand::{self, Rng};

fn main() {
    let mut image = open_rgb("./images/Set03_single.jpg").unwrap();
    let hsv_img = image.to_hsv();

    // let (centers, clusters) = k_means(&hsv_img, 2, |image, p1, p2| {
    //     let hsv1 = image.get_pixel(p1.x, p1.y);
    //     let hsv2 = image.get_pixel(p2.x, p2.y);

    //     // calculate distance between hues in a circular way
    //     let hue_distance = (hsv1[0] - hsv2[0])
    //         .abs()
    //         .min(360.0 - (hsv1[0] - hsv2[0]).abs());
    //     hue_distance
    // });

    let (centers, clusters) = k_means_hsv(&hsv_img, 20);

    for (i, cluster) in clusters.iter().enumerate() {
        let center_color = centers[i];

        // center color to rgb
        let (h, s, v) = (center_color[0], center_color[1], center_color[2]);
        let h_i = (h / 60.).floor();
        let f = h / 60. - h_i;

        let p = v * (1. - s);
        let q = v * (1. - s * f);
        let t = v * (1. - s * (1. - f));

        let (r, g, b) = match h_i {
            0. | 6. => (v, t, p),
            1. => (q, v, p),
            2. => (p, v, t),
            3. => (p, q, v),
            4. => (t, p, v),
            5. => (v, p, q),
            _ => unreachable!(),
        };

        for point in cluster {
            image.put_pixel(
                point.x,
                point.y,
                Rgb([(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8]),
            );
        }
    }

    image.save("export/hsv_kmeans.png").unwrap();
}

fn k_means_hsv(
    image: &image::ImageBuffer<Rgb<f32>, Vec<f32>>,
    k: usize,
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
    while !converged {
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

/// returns (centers, clusters)
fn k_means<T: GenericImage>(
    image: &T,
    k: usize,
    distance: impl Fn(&T, Point, Point) -> f32,
) -> (Vec<Point>, Vec<Vec<Point>>) {
    let mut centers = Vec::<Point>::with_capacity(k);
    let mut clusters: Vec<Vec<Point>> = vec![Vec::new(); centers.len()];

    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let x = rng.gen_range(0..image.width());
        let y = rng.gen_range(0..image.height());
        centers.push(Point::new(x, y));
    }

    let mut converged = false;
    while !converged {
        clusters = vec![Vec::new(); centers.len()];

        image.pixels().for_each(|(x, y, _)| {
            let point = Point::new(x, y);
            let nearest_center_index = centers
                .iter()
                .map(|&center| {
                    let distance = distance(image, point, center);
                    distance
                })
                .enumerate()
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(index, _)| index)
                .expect("one center should be the nearest");

            clusters[nearest_center_index].push(point);
        });

        // println!(
        //     "clusters: {:?}",
        //     clusters.iter().map(|c| c.len()).collect::<Vec<_>>()
        // );

        let new_centers = clusters
            .iter()
            .enumerate()
            .map(|(index, cluster)| {
                // if cluster is empty, return the same center
                if cluster.is_empty() {
                    return centers[index];
                }

                let x = cluster.iter().map(|p| p.x).sum::<u32>() / cluster.len() as u32;
                let y = cluster.iter().map(|p| p.y).sum::<u32>() / cluster.len() as u32;
                Point::new(x, y)
            })
            .collect::<Vec<_>>();

        converged = centers
            .iter()
            .zip(new_centers.iter())
            .all(|(c1, c2)| c1 == c2);
        centers = new_centers;

        // if converged {
        //     break;
        // }
    }

    (centers, clusters)
}
