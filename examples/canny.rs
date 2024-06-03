use openbv::{open_gray, regionops::canny::CannyExt};

fn main() {
    let image = open_gray("./images/messi.jpeg").unwrap();

    image.save("export/bcanny.png").unwrap();

    let canny_edges = image.canny();
    canny_edges.save("export/canny.png").unwrap();
}
