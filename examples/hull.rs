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

    let contours = dilated_img.find_contours();
    println!("Number of contours: {}", contours.len());
    let contour_img = binary_img.draw_contours(contours);
    contour_img.save("test.png").unwrap();
}
