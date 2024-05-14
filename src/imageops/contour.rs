use image::{GenericImageView, GrayImage};

use super::binarize::BinaryImage;

pub struct Contour {
    pub points: Vec<(u32, u32)>,
}

pub struct Moments;

pub trait ContourExt {
    fn find_contours(&self) -> Vec<Contour>;
    fn trace_contour(&self, x: u32, y: u32) -> Contour;
    fn handle_fg_pixel(
        &self,
        output_img: &mut GrayImage,
        x: u32,
        y: u32,
        label: &mut u8,
        r: &mut u8,
        outer_contours: &mut Vec<Contour>,
    );
    fn handle_bg_pixel(
        &self,
        output_img: &mut GrayImage,
        x: u32,
        y: u32,
        label: &mut u8,
        r: &mut u8,
        inner_contours: &mut Vec<Contour>,
    );
}

impl ContourExt for BinaryImage {
    fn find_contours(&self) -> Vec<Contour> {
        let img = &self.0;
        let mut output_img = self.0.clone();
        let mut outer_contours = vec![];
        let mut inner_contours = vec![];

        let mut r = 1;
        for y in 0..img.dimensions().1 {
            let mut label = 0;
            for x in 0..img.dimensions().0 {
                if img.get_pixel(x, y)[0] == 255 {
                    self.handle_fg_pixel(
                        &mut output_img,
                        x,
                        y,
                        &mut label,
                        &mut r,
                        &mut outer_contours,
                    );
                } else {
                    self.handle_bg_pixel(
                        &mut output_img,
                        x,
                        y,
                        &mut label,
                        &mut r,
                        &mut inner_contours,
                    );
                }
            }
        }

        outer_contours
    }

    fn trace_contour(&self, x: u32, y: u32) -> Contour {
        todo!()
    }

    fn handle_fg_pixel(
        &self,
        output_img: &mut GrayImage,
        x: u32,
        y: u32,
        mut label: &mut u8,
        r: &mut u8,
        outer_contours: &mut Vec<Contour>,
    ) {
        if *label != 0 {
            output_img.put_pixel(x, y, image::Luma([*label]))
        } else {
            label = &mut output_img.get_pixel(x, y)[0];
            if *label == 0 {
                *r += 1;
                label = r;
                let contour = self.trace_contour(x, y);
                outer_contours.push(contour);
                output_img.put_pixel(x, y, image::Luma([*label]))
            }
        }
    }

    fn handle_bg_pixel(
        &self,
        output_img: &mut GrayImage,
        x: u32,
        y: u32,
        label: u8,
        r: u32,
        inner_contours: &mut Vec<Contour>,
    ) {
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
