use image::{DynamicImage, GrayImage, ImageResult, RgbImage};
use rayon::iter::ParallelIterator;

use super::contour::{Contour, ConvexHull};

pub trait BinarizeExt {
    fn binarize(&self, threshold: u8) -> BinaryImage;
}

pub struct BinaryImage(pub GrayImage);

impl BinaryImage {
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<std::path::Path>,
    {
        self.0.save(path)?;
        Ok(())
    }

    pub fn draw_contours(
        &self,
        inner_contours: Vec<Contour>,
        outer_contours: Vec<Contour>,
    ) -> RgbImage {
        let mut out_img = DynamicImage::ImageLuma8(self.0.clone()).to_rgb8();

        for contour in inner_contours {
            for (x, y) in contour.points {
                out_img.put_pixel(x, y, image::Rgb([255, 0, 0]));
            }
        }

        for contour in outer_contours {
            for (x, y) in contour.points {
                out_img.put_pixel(x, y, image::Rgb([0, 0, 255]));
            }
        }

        out_img
    }

    pub fn draw_hulls(&self, hulls: Vec<ConvexHull>) -> RgbImage {
        let mut out_img = DynamicImage::ImageLuma8(self.0.clone()).to_rgb8();

        for hull in hulls {
            for i in 0..hull.0.len() {
                let (x1, y1) = hull.0[i];
                let (x2, y2) = hull.0[(i + 1) % hull.0.len()];
                draw_line(&mut out_img, x1, y1, x2, y2);
            }
        }

        out_img
    }
}

pub fn draw_line(img: &mut RgbImage, x1: u32, y1: u32, x2: u32, y2: u32) {
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
        img.put_pixel(x1 as u32, y1 as u32, image::Rgb([0, 255, 0]));

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

impl BinarizeExt for GrayImage {
    /// returns a new binary image
    fn binarize(&self, threshold: u8) -> BinaryImage {
        let mut new_img = self.clone();

        new_img
            .par_enumerate_pixels_mut()
            .for_each(|(x, y, pixel)| {
                pixel.0[0] = if self.get_pixel(x, y).0[0] <= threshold {
                    0
                } else {
                    255
                };
            });

        BinaryImage(new_img)
    }
}
