use openbv::{
    imageops::{binarize::BinarizeExt, contour::ContourExt, dilate::DilateExt, erode::ErodeExt},
    math::{kernel::PLUS_FILTER, thresholds::ThresholdExt},
    open_gray,
};

fn main() {
    let image = open_gray("./images/Set03_single.jpg").unwrap();

    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);

    let eroded_img = binary_img.erode(PLUS_FILTER, 2);
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 2);

    let (mut inner_contours, mut outer_contours) = dilated_img.find_contours();
    inner_contours.append(&mut outer_contours);

    let hulls = inner_contours
        .iter()
        .map(|c| c.convex_hull())
        .collect::<Vec<_>>();

    for hull in hulls {
        println!("{:?}", hull.0);
    }

    let hulls = inner_contours
        .iter()
        .map(|c| c.convex_hull().approx_db(0.1))
        .collect::<Vec<_>>();

    for hull in &hulls {
        println!("{:?}", hull.0);
    }

    let hull_img = dilated_img.draw_hulls(hulls);
    hull_img.save("test.png").unwrap();
}
