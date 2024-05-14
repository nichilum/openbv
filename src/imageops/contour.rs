use image::GenericImageView;

use super::binarize::BinaryImage;

pub struct Contour {
    pub points: Vec<(u32, u32)>,
}

pub struct Moments;

pub trait ContourExt {
    fn find_contours(&self) -> Vec<Contour>;
    fn trace_contour(&self, x: u32, y: u32) -> Contour;
}

impl ContourExt for BinaryImage {
    fn find_contours(&self) -> Vec<Contour> {
        let img = &self.0;
        let mut contours = vec![];

        for y in 0..img.dimensions().1 {
            for x in 0..img.dimensions().0 {
                if img.get_pixel(x, y)[0] == 255 {
                    contours.push(self.trace_contour(x, y));
                    return contours;
                }
            }
        }

        contours
    }

    fn trace_contour(&self, x: u32, y: u32) -> Contour {
        todo!()
    }
}

impl Contour {
    pub fn convex_hull(&self) {
        todo!()
    }
    pub fn approx_poly_db(&self) {
        todo!()
    }
    pub fn contour_area(&self) {
        todo!()
    }
    pub fn arc_length(&self) {
        todo!()
    }
    pub fn moments(&self) -> Moments {
        todo!()
    }
}

impl Moments {
    pub fn hu_moments(&self) {
        todo!()
    }
}
