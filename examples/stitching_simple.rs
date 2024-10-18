use image::{
    imageops::{crop, resize},
    GenericImage, GenericImageView, GrayImage, RgbImage,
};
use openbv::{open_gray, regionops::canny::CannyExt};

const STEP_SIZE: i32 = 100;
const SCALE_FAC: u32 = 10;

fn main() {
    let image_one = open_gray("./images/stitch_two.jpg").unwrap();
    let image_two = open_gray("./images/stitch_one.jpg").unwrap();

    let image_one_r = resize(
        &image_one,
        image_one.width() / SCALE_FAC,
        image_one.height() / SCALE_FAC,
        image::imageops::FilterType::Nearest,
    );
    let image_two_r = resize(
        &image_two,
        image_two.width() / SCALE_FAC,
        image_two.height() / SCALE_FAC,
        image::imageops::FilterType::Nearest,
    );

    let edges_one = image_one_r.canny();
    let edges_two = image_two_r.canny();

    edges_one.save("edges_one.png").unwrap();
    edges_two.save("edges_two.png").unwrap();

    let width = edges_one.width() as i32;
    let height = edges_one.height() as i32;

    // println!("width: {width}");
    // println!("height: {height}");

    let mut min_diff = std::i32::MAX;
    let mut min_x = 0;
    let mut min_y = 0;

    let mut i = 1; // debug

    for x in (-width + STEP_SIZE..width - STEP_SIZE).step_by(STEP_SIZE as usize) {
        for y in (-height + STEP_SIZE..height - STEP_SIZE).step_by(STEP_SIZE as usize) {
            // println!("x: {x}");
            // println!("y: {y}");

            let view_one = edges_one.view(
                if x <= 0 { 0 } else { x } as u32,
                if y <= 0 { 0 } else { y } as u32,
                (width - x.abs()) as u32,
                (height - y.abs()) as u32,
            );
            let view_two = edges_two.view(
                if x <= 0 { x.abs() } else { 0 } as u32,
                if y <= 0 { y.abs() } else { 0 } as u32,
                (width - x.abs()) as u32,
                (height - y.abs()) as u32,
            );
            let diff = difference(&view_one, &view_two);

            println!(
                "Diff: {}, {i}/{}",
                diff,
                (edges_one.width() as i32 / STEP_SIZE as i32)
                    * (edges_one.height() as i32 / STEP_SIZE as i32)
            ); // debug
            i += 1;

            if diff < min_diff && diff != 0 {
                min_diff = diff;
                min_x = x;
                min_y = y;
            }
        }
    }

    println!("Min diff: {}", min_diff);
    println!("Min x: {}", min_x);
    println!("Min y: {}", min_y);

    let x_position = min_x * SCALE_FAC as i32;
    let y_position = min_y * SCALE_FAC as i32;

    let mut stitch_img = GrayImage::new(
        image_one.width() + x_position.abs() as u32,
        image_one.height() + y_position.abs() as u32,
    );

    stitch_img
        .copy_from(
            &image_two,
            if x_position <= 0 {
                x_position.abs() as u32
            } else {
                0
            },
            if y_position <= 0 {
                y_position.abs() as u32
            } else {
                0
            },
        )
        .unwrap();
    stitch_img
        .copy_from(
            &image_one,
            if x_position <= 0 {
                0
            } else {
                x_position as u32
            },
            if y_position <= 0 {
                0
            } else {
                y_position as u32
            },
        )
        .unwrap();
    stitch_img.save("export/stitchtest_1.png").unwrap();
}

fn difference(
    image_one: &image::SubImage<&image::GrayImage>,
    image_two: &image::SubImage<&image::GrayImage>,
) -> i32 {
    assert_eq!(image_one.dimensions(), image_two.dimensions());

    let mut acc = 0;

    image_one
        .to_image()
        .enumerate_pixels()
        .for_each(|(x, y, pixel)| {
            let pixel_two = image_two.get_pixel(x, y);
            acc += pixel.0[0] as i32 - pixel_two.0[0] as i32
        });
    acc.abs()
}
