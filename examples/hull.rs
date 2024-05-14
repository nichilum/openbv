use openbv::{
    imageops::{binarize::BinarizeExt, contour::ContourExt, dilate::DilateExt, erode::ErodeExt},
    math::{hull::{approx_hull, graham_scan}, kernel::PLUS_FILTER, thresholds::ThresholdExt},
    open_gray,
};

fn main() {
    let image = open_gray("./images/Set03.jpg").unwrap();

    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);

    let eroded_img = binary_img.erode(PLUS_FILTER, 2);
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 2);

    let contours = dilated_img.find_contours();
    let hulls = contours.0.iter().map(|c| graham_scan(&c.points)).map(|c| approx_hull(&c, 0.5)).collect::<Vec<_>>();
    let hull_img = dilated_img.draw_hulls(hulls);
    hull_img.save("test.png").unwrap();
}
