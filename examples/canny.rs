use openbv::{open_gray, regionops::canny::CannyExt};

fn main() {
    let image = open_gray("./images/Set03_single.jpg").unwrap();

    image.save("export/bcanny.png").unwrap();

    let canny_edges = image.canny();
    canny_edges.save("export/canny.png").unwrap();
}
