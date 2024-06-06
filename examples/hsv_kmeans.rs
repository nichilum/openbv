use image::Rgb;
use openbv::math::k_means::k_means_hsv;
use openbv::{colorops::hsv::HsvExt, open_rgb};

fn main() {
    let mut image = open_rgb("./images/Set03_single.jpg").unwrap();
    let hsv_img = image.to_hsv();

    let (centers, clusters) = k_means_hsv(&hsv_img, 4, 0);

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
