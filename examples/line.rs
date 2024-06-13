use std::f32::consts::PI;

use openbv::{math::line_detection::{draw_lines, hough}, open_gray};

fn main() {
    let image = open_gray("./images/line.png").unwrap();
    let hough_lines = hough(&image, 1., PI / 180., 150);
    println!("{:?}", hough_lines);

    let out_img = draw_lines(&image, &hough_lines);
    out_img.save("export/line_hough.png").unwrap();
}
