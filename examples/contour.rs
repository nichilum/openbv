use openbv::{imageops::binarize::BinarizeExt, math::thresholds::ThresholdExt, *};

fn main() {
    let image = open_gray("./images/Set01.jpg").unwrap();
    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);
}
