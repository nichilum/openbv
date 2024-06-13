use image::{DynamicImage, GenericImageView, GrayImage, ImageResult, Rgb, RgbImage};
use rand::Rng;

use crate::{
    math::point::Point,
    regionops::{contour::Contour, hull::Hull},
};

pub struct BinaryImage(pub GrayImage);

impl BinaryImage {
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<std::path::Path>,
    {
        self.0.save(path)?;
        Ok(())
    }

    // draws contour and their respective center points
    pub fn draw_contours(&self, contours: &[Contour]) -> RgbImage {
        let mut out_img = DynamicImage::ImageLuma8(self.0.clone()).to_rgb8();
        let mut rng = rand::thread_rng();

        for contour in contours {
            let r = rng.gen_range(0..255);
            let g = rng.gen_range(0..255);
            let b = rng.gen_range(0..255);
            for Point { x, y } in &contour.points {
                out_img.put_pixel(*x, *y, image::Rgb([r, g, b]));
            }

            let Point { x, y } = contour.get_center();
            out_img.put_pixel(x, y, image::Rgb([r, g, b]));
        }

        out_img
    }

    pub fn draw_hulls(&self, hulls: Vec<impl Hull>) -> RgbImage {
        let mut out_img = DynamicImage::ImageLuma8(self.0.clone()).to_rgb8();

        let mut rng = rand::thread_rng();
        for hull in hulls {
            let r = rng.gen_range(60..255);
            let g = rng.gen_range(60..255);
            let b = rng.gen_range(60..255);

            let points = hull.get_points();
            for i in 0..points.len() {
                let Point { x: x1, y: y1 } = points[i];
                let Point { x: x2, y: y2 } = points[(i + 1) % points.len()];
                draw_line(&mut out_img, x1, y1, x2, y2, image::Rgb([r, g, b]));
            }

            let Point { x, y } = hull.get_center();
            out_img.put_pixel(x, y, image::Rgb([r, g, b]));
        }

        out_img
    }
}

pub fn draw_line(img: &mut RgbImage, x1: u32, y1: u32, x2: u32, y2: u32, pixel: Rgb<u8>) {
    let mut x1 = x1 as i32;
    let mut y1 = y1 as i32;
    let x2 = x2 as i32;
    let y2 = y2 as i32;

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        if img.in_bounds(x1 as u32, y1 as u32) {
            img.put_pixel(x1 as u32, y1 as u32, pixel);
        }

        if x1 == x2 && y1 == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x1 += sx;
        }
        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }
}

impl From<BinaryImage> for GrayImage {
    fn from(img: BinaryImage) -> GrayImage {
        img.0
    }
}

impl From<GrayImage> for BinaryImage {
    // TODO: maybe check if self is an actual binary image
    fn from(img: GrayImage) -> BinaryImage {
        BinaryImage(img)
    }
}
