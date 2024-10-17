use image::Rgb;
use openbv::{open_gray, open_rgb, pointops::binarize::BinarizeExt};

fn main() {
    let mut image = open_rgb("./images/messi.jpeg").unwrap();
    let bin_img = open_gray("./images/laute.jpg").unwrap().binarize(128);

    image.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        *pixel = Rgb([
            pixel.0[0],
            pixel.0[1],
            (pixel.0[2] & 0b11111110) | (bin_img.0.get_pixel(x, y).0[0] >> 7),
        ]);
    });

    // bin_img.save("export/juan_bin.png").unwrap();
    image.save("export/steganography.png").unwrap();
}
