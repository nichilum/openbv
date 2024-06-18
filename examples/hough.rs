use openbv::{
    math::hough::{draw_lines, Line},
    open_gray,
};

fn main() {
    let image = open_gray("./images/line.png").unwrap();
    // let hough_lines = hough(&image, 1., PI / 180., 150);
    // println!("{:?}", hough_lines);

    // let out_img = draw_lines(&image, &hough_lines);
    let out_img = draw_lines(
        &image,
        &[Line {
            r: 100.,
            theta: 90f32.to_radians(),
        }],
    );
    out_img.save("export/line_hough.png").unwrap();
}
