use openbv::{
    imageops::{binarize::BinarizeExt, contour::ContourExt, dilate::DilateExt, erode::ErodeExt},
    math::{kernel::PLUS_FILTER, thresholds::ThresholdExt},
    open_gray,
};

fn main() {
    let image = open_gray("./images/Set03.jpg").unwrap();

    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);

    let eroded_img = binary_img.erode(PLUS_FILTER, 2);
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 2);

    let (mut inner_contours, mut outer_contours) = dilated_img.find_contours();

    // only keep contours with area > 100
    inner_contours.retain(|c| c.contour_area() > 100);
    outer_contours.retain(|c| c.contour_area() > 100);

    let contour_img = dilated_img.draw_contours(inner_contours, outer_contours);
    contour_img.save("test.png").unwrap();
}
