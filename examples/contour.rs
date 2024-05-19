use openbv::{
    math::{kernel::PLUS_FILTER, thresholds::ThresholdExt},
    morphops::{dilate::DilateExt, erode::ErodeExt},
    open_gray,
    pointops::binarize::BinarizeExt,
    regionops::contour::ContourExt,
};

fn main() {
    let image = open_gray("./images/test.png").unwrap();

    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);

    let eroded_img = binary_img.erode(PLUS_FILTER, 2);
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 2);

    let (mut inner_contours, mut outer_contours) = dilated_img.find_contours();
    // only keep contours with area > 10
    // maybe clean up always in find_contours()?
    inner_contours.retain(|c| c.contour_area() > 10);
    outer_contours.retain(|c| c.contour_area() > 10);
    inner_contours.append(&mut outer_contours);

    let contour_img = dilated_img.draw_contours(inner_contours);
    contour_img.save("test.png").unwrap();
}
