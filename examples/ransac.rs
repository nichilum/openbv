use openbv::{
    math::{point::Point, ransac::ransac_circle_2d},
    open_gray,
};

fn main() {
    let image = open_gray("./images/ransac.png").unwrap();
    let data_thresh = 20;
    let mut data: Vec<Point> = vec![];

    image.enumerate_pixels().for_each(|(x, y, pixel)| {
        if pixel[0] < data_thresh {
            data.push(Point { x, y });
        }
    });

    let ransac_circle = ransac_circle_2d(data.as_slice(), 100);

    println!("{:?}", ransac_circle.radius());
    println!("{:?}", ransac_circle.center());
}
