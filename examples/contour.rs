use openbv::{
    math::{kernel::PLUS_FILTER, thresholds::ThresholdExt},
    morphops::{dilate::DilateExt, erode::ErodeExt},
    open_gray,
    pointops::binarize::BinarizeExt,
    regionops::contour::{Contour, ContourDeleteExt, ContourExt},
};

fn main() {
    let image = open_gray("./images/test.png").unwrap();

    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);

    let eroded_img = binary_img.erode(PLUS_FILTER, 2);
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 2);

    let (mut inner_contours, mut outer_contours) = dilated_img.find_contours();

    inner_contours.delete_by_area(10);
    outer_contours.delete_by_area(10);
    let combined_contours = Contour::combine(&inner_contours, &outer_contours);

    let contour_img = dilated_img.draw_contours(&combined_contours);
    contour_img.save("contours.png").unwrap();
}
