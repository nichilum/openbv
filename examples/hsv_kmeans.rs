use image::GenericImage;
use openbv::{colorops::hsv::HsvExt, math::point::Point, open_rgb};

use rand::{self, Rng};

fn main() {
    let mut image = open_rgb("./images/Set03_single.jpg").unwrap();
    let hsv_img = image.to_hsv();

    let (centers, clusters) = k_means(&hsv_img, 2, |image, p1, p2| {
        let hsv1 = image.get_pixel(p1.x, p1.y);
        let hsv2 = image.get_pixel(p2.x, p2.y);

        // calculate distance between hues in a circular way
        let hue_distance = (hsv1[0] - hsv2[0])
            .abs()
            .min(360.0 - (hsv1[0] - hsv2[0]).abs());
        hue_distance
    });

    println!("centers: {:?}", centers);

    for (i, cluster) in clusters.iter().enumerate() {
        let center = centers[i];
        let center_color = image.get_pixel(center.x, center.y).clone();
        for point in cluster {
            image.put_pixel(point.x, point.y, center_color);
        }
    }

    image.save("export/hsv_kmeans.png").unwrap();
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
