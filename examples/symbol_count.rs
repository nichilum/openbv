use openbv::{
    math::{kernel::PLUS_FILTER, thresholds::ThresholdExt},
    morphops::{dilate::DilateExt, erode::ErodeExt},
    open_gray,
    pointops::binarize::BinarizeExt,
    regionops::contour::{Contour, ContourDeleteExt, ContourExt},
};

// hardcode, ._.
const CARD_AREA_THRESH: u32 = 5000;

struct Card {
    pub contour: Contour,
    pub inner_symbols_contours: Vec<Contour>,
    pub outer_symbols_contours: Vec<Contour>,
}

fn main() {
    let image = open_gray("./images/Set03.jpg").unwrap();

    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);

    let eroded_img = binary_img.erode(PLUS_FILTER, 1);
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 1);

    let (mut inner_contours, mut outer_contours) = dilated_img.find_contours();
    inner_contours.delete_by_area(10);
    outer_contours.delete_by_area(10);

    let card_contours = outer_contours.filter_by_area(CARD_AREA_THRESH);
    let mut cards = Vec::new();

    for contour in &card_contours {
        let convex_hull = contour.convex_hull();
        let mut card = Card {
            contour: contour.clone(),
            inner_symbols_contours: Vec::new(),
            outer_symbols_contours: Vec::new(),
        };

        for inner in &inner_contours {
            let center = inner.get_center();
            if convex_hull.contains(center) {
                card.inner_symbols_contours.push(inner.clone());
            }
        }

        cards.push(card);
    }

    let contour_img = dilated_img.draw_contours(&cards[4].inner_symbols_contours);
    contour_img.save("outer_contours.png").unwrap();
}
