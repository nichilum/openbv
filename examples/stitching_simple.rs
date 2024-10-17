use image::{imageops::resize, GenericImageView};
use openbv::{open_gray, regionops::canny::CannyExt};

const STEP_SIZE: u32 = 100;

fn main() {
    let image_one = open_gray("./images/stitch_two.jpg").unwrap();
    let image_two = open_gray("./images/stitch_one.jpg").unwrap();

    let image_one = resize(&image_one, image_one.width()/10, image_one.height()/10, image::imageops::FilterType::Nearest);
    let image_two = resize(&image_two, image_two.width()/10, image_two.height()/10, image::imageops::FilterType::Nearest);

    let edges_one = image_one.canny();
    let edges_two = image_two.canny();

    edges_one.save("edges_one.png").unwrap();
    edges_two.save("edges_two.png").unwrap();

    // for x in (-(edges_one.width() as i32)..edges_one.width() as i32) .step_by(10){
    //     for y in (-(edges_one.height() as i32)..edges_one.height() as i32).step_by(10) {
    //         edges_one.view(x, y, width, height)
    //     }
    // }

    let mut min_diff = std::i32::MAX;
    let mut min_x = 0;
    let mut min_y = 0;

    let mut i = 1;

    for x in (0..edges_one.width() as i32 - STEP_SIZE as i32).step_by(STEP_SIZE as usize) {
        for y in (0..edges_one.height() as i32 - STEP_SIZE as i32).step_by(STEP_SIZE as usize) {
            let view_one = edges_one.view(
                x as u32,
                y as u32,
                edges_one.width() - x as u32,
                edges_one.height() - y as u32,
            );
            let view_two = edges_two.view(
                0,
                0,
                edges_two.width() - x as u32,
                edges_two.height() - y as u32,
            );
            let diff = difference(&view_one, &view_two);

            println!("Diff: {}, {i}/{}", diff, (edges_one.width() as i32 / STEP_SIZE as i32) * (edges_one.height() as i32 / STEP_SIZE as i32));
            i += 1;

            if diff < min_diff {
                min_diff = diff;
                min_x = x;
                min_y = y;
            }
        }
    }

    println!("Min diff: {}", min_diff);
    println!("Min x: {}", min_x);
    println!("Min y: {}", min_y);

}


fn difference(
    image_one: &image::SubImage<&image::GrayImage>,
    image_two: &image::SubImage<&image::GrayImage>,
) -> i32 {
    assert_eq!(image_one.dimensions(), image_two.dimensions());

    let mut acc = 0;

    image_one.to_image().enumerate_pixels().for_each(|(x, y, pixel)| {
        let pixel_two = image_two.get_pixel(x, y);
        acc += pixel.0[0] as i32 - pixel_two.0[0] as i32
    });
    acc.abs()
}
