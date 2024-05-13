use super::binarize::BinaryImage;

pub struct Contour;

pub struct Moments;

pub trait ContourExt {
    fn find_contours(&self) -> Vec<Contour>;
}

impl ContourExt for BinaryImage {
    fn find_contours(&self) -> Vec<Contour> {
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
