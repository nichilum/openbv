use std::fmt::Display;

use image::RgbImage;
use openbv::{
    math::{kernel::PLUS_FILTER, point::Point, thresholds::ThresholdExt},
    morphops::{dilate::DilateExt, erode::ErodeExt},
    open_gray,
    pointops::binarize::BinarizeExt,
    regionops::contour::{Contour, ContourDeleteExt, ContourExt},
};

// hardcode, ._.
const CARD_AREA_THRESH: u32 = 5000;
const INNER_SYMBOL_AREA_THRESH: u32 = 300;

#[derive(Debug)]
enum FillStyle {
    Striped,
    Filled,
    Outlined,
}

#[derive(Debug)]
enum SymbolType {
    Wave,
    Pill,
    Rhombus,
}

#[derive(Debug)]
struct Card {
    pub center: Point,
    pub inner_symbols_contours: Vec<Contour>,
    pub outer_symbols_contours: Vec<Contour>,
    pub symbol_amount: Option<usize>,
    pub fill_style: Option<FillStyle>,
    pub symbol_type: Option<SymbolType>,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Symbol Type: {:?}, Symbol Amount: {:?}, Fill Style: {:?}",
            self.symbol_type,
            self.symbol_amount,
            self.fill_style.as_ref()
        )
    }
}

fn main() {
    let image = open_gray("./images/Set03.jpg").unwrap();

    let otsu_thresh = image.otsu().unwrap();
    let binary_img = image.binarize(otsu_thresh);

    let eroded_img = binary_img.erode(PLUS_FILTER, 1);
    let dilated_img = eroded_img.dilate(PLUS_FILTER, 1);

    let (mut inner_contours, mut outer_contours) = dilated_img.find_contours();
    inner_contours.delete_by_area(8);
    outer_contours.delete_by_area(8);
    inner_contours.delete_duplicates();
    outer_contours.delete_duplicates();

    let mut contour_img =
    // dilated_img.draw_contours(&Contour::combine(&inner_contours, &outer_contours));
    dilated_img.draw_contours(&inner_contours);

    let card_contours = outer_contours.filter_by_area(CARD_AREA_THRESH);
    let mut cards = Vec::new();

    // separate symbols in outer and inner per card
    for contour in card_contours {
        let convex_hull = contour.convex_hull();
        let mut card = Card {
            inner_symbols_contours: Vec::new(),
            outer_symbols_contours: Vec::new(),
            fill_style: None,
            symbol_amount: None,
            center: contour.get_center(),
            symbol_type: None,
        };

        for inner in &inner_contours {
            let center = inner.get_center();
            if convex_hull.contains(center) {
                card.inner_symbols_contours.push(inner.clone());
            }
        }

        for outer in &outer_contours {
            let center = outer.get_center();
            if convex_hull.contains(center) {
                card.outer_symbols_contours.push(outer.clone());
            }
        }

        cards.push(card);
    }

    // count outer and inner symbols per card
    for (i, card) in cards.iter_mut().enumerate() {
        card.inner_symbols_contours
            .delete_by_area(INNER_SYMBOL_AREA_THRESH);
        let inner_count = card.inner_symbols_contours.len();
        let outer_count = card.outer_symbols_contours.len();

        assert!(inner_count > 0);

        let hus = card.inner_symbols_contours[0].hu_moments();

        // am siebten Hu Moment machten wir die Wave => e-10
        if hus.hu_moments[6] >= 1.0e-10 {
            card.symbol_type = Some(SymbolType::Wave);
        } else if hus.hu_moments[1] >= 0.021 || hus.eccentricity <= 4.4 {
            card.symbol_type = Some(SymbolType::Rhombus);
        } else {
            card.symbol_type = Some(SymbolType::Pill);
        }

        if outer_count == 0 {
            card.symbol_amount = Some(inner_count);
            card.fill_style = Some(FillStyle::Filled);
        } else if outer_count == inner_count {
            card.symbol_amount = Some(inner_count);
            card.fill_style = Some(FillStyle::Outlined);
        } else if outer_count > inner_count {
            card.symbol_amount = Some(inner_count);
            card.fill_style = Some(FillStyle::Striped);
        }

        draw_text(
            &mut contour_img,
            format!("{}", i).as_str(),
            card.center.x as i32,
            card.center.y as i32,
        );
        draw_text(
            &mut contour_img,
            format!("{:?}", card.fill_style).as_str(),
            card.center.x as i32,
            card.center.y as i32 + 20,
        );
        draw_text(
            &mut contour_img,
            format!("{:?}", card.symbol_amount).as_str(),
            card.center.x as i32,
            card.center.y as i32 + 40,
        );
        draw_text(
            &mut contour_img,
            format!("{:?}", card.symbol_type).as_str(),
            card.center.x as i32,
            card.center.y as i32 + 60,
        );
    }

    contour_img.save("outer_contours.png").unwrap();
}

use ab_glyph::{FontRef, PxScale};
use image::Rgb;
use imageproc::drawing::draw_text_mut;

fn draw_text(image: &mut RgbImage, text: &str, x: i32, y: i32) {
    let font = FontRef::try_from_slice(include_bytes!("../fonts/Cairo-Medium.ttf")).unwrap();

    let height = 20.;
    let scale = PxScale {
        x: height,
        y: height,
    };

    draw_text_mut(image, Rgb([255u8, 0, 0]), x, y, scale, &font, text);
}
