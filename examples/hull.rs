use std::fmt::Binary;

use openbv::{
    imageops::{
        binarize::{BinarizeExt, BinaryImage},
        contour::ContourExt,
        dilate::DilateExt,
        erode::ErodeExt,
    },
    math::{kernel::PLUS_FILTER, thresholds::ThresholdExt},
    open_gray,
};

fn main() {
    let image = open_gray("./images/test.png").unwrap();

    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);

    let eroded_img = binary_img.erode(PLUS_FILTER, 2);
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 2);

    let (mut inner_contours, mut outer_contours) = dilated_img.find_contours();
    inner_contours.retain(|c| c.contour_area() > 10);
    outer_contours.retain(|c| c.contour_area() > 10);
    // inner_contours.append(&mut outer_contours);

    // let convex_hulls = outer_contours
    //     .iter()
    //     .map(|c| c.convex_hull())
    //     .collect::<Vec<_>>();

    let poly_hulls = outer_contours
        .iter()
        .map(|c| c.poly_hull(10.9))
        .collect::<Vec<_>>();

    // for hull in &poly_hulls {
    //     println!("{:?}", hull.0);
    // }

    let hull_img = dilated_img.draw_hulls(poly_hulls);
    hull_img.save("poly_hulls.png").unwrap();
}
