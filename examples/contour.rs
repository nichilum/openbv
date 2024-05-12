use openbv::{
    imageops::{binarize::BinarizeExt, dilate::DilateExt, erode::ErodeExt},
    math::{kernel::PLUS_FILTER, thresholds::ThresholdExt},
    *,
};

fn main() {
    let image = open_gray("./images/Set03.jpg").unwrap();
    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);
    binary_img.save("./fuck_this.png").unwrap();
    let eroded_img = binary_img.erode(PLUS_FILTER, 2);
    eroded_img.save("./shit_im.png").unwrap();
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 2);
    dilated_img.save("./out.png").unwrap();
}
