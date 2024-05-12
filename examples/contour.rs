use openbv::{math::thresholds::otsu, *};

fn main() {
    let image = open_gray("./images/Set01.jpg");
    let otsu_thresh = image.otsu();
}
