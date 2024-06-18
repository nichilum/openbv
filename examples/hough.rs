use std::f32::consts::PI;

use openbv::{
    math::hough::{draw_lines, hough},
    open_gray, regionops::canny::CannyExt,
};

fn main() {
    let image = open_gray("./images/Set03_single.jpg").unwrap();
    let canny = image.canny();
    let hough_lines = hough(&canny, 1., PI / 180., 15);
    println!("{:?}", hough_lines);

    let out_img = draw_lines(&canny, &hough_lines);
    // let out_img = draw_lines(
    //     &image,
    //     &[Line {
    //         r: 100.,
    //         theta: 90f32.to_radians(),
    //     }],
    // );
    out_img.save("export/line_hough.png").unwrap();
}
