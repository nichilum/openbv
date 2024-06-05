use image::Luma;
use openbv::{colorops::hsv::HsvExt, open_gray, open_rgb};

use rayon::iter::ParallelIterator;

fn main() {
    let image = open_rgb("./images/Set03_single.jpg").unwrap();
    let hsv_img = image.to_hsv();

    println!("{:?}", hsv_img.get_pixel(93, 127));

    // hsl(347, 57%, 43%)

    let mut binary_img = open_gray("./images/Set03_single.jpg").unwrap();
    binary_img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
        let hsv_pixel = hsv_img.get_pixel(x, y);
        let h = hsv_pixel[0];
        let s = hsv_pixel[1];
        let v = hsv_pixel[2];
        *p = Luma([match (h, s, v) {
            (330f32..=360f32, 0.5f32..=1f32, 0.5f32..=1f32) => 255,
            _ => 0,
        }]);
    });

    binary_img.save("export/hsv_bin.png").unwrap();
}
