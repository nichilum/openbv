use openbv::{
    math::{kernel::PLUS_FILTER, thresholds::ThresholdExt},
    morphops::{dilate::DilateExt, erode::ErodeExt},
    open_gray,
    pointops::binarize::BinarizeExt,
    regionops::contour::{Contour, ContourDeleteExt, ContourExt},
};

// hardcode, ._.
const CARD_AREA_THRESH: u32 = 5000;

fn main() {
    let image = open_gray("./images/Set03.jpg").unwrap();

    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);

    let eroded_img = binary_img.erode(PLUS_FILTER, 1);
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 1);

    let (mut inner_contours, mut outer_contours) = dilated_img.find_contours();
    inner_contours.delete_by_area(10);
    outer_contours.delete_by_area(10);

    let cards = outer_contours.filter_by_area(CARD_AREA_THRESH);
    let symbol_contours = Contour::combine(&inner_contours, &outer_contours);

    for contour in &cards {
        let convex_hull = contour.convex_hull();
        for symbol in &symbol_contours {
            let center = symbol.get_center();
            let contains = convex_hull.contains(center);
        }
    }

    let contour_img = dilated_img.draw_contours(symbol_contours);
    contour_img.save("outer_contours.png").unwrap();
}