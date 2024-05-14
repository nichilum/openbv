use std::io::Read;

use image::{DynamicImage, GrayImage, ImageResult, RgbImage};
use rayon::iter::ParallelIterator;

use super::contour::Contour;

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

        // for contour in outer_contours {
        //     for (x, y) in contour.points {
        //         out_img.put_pixel(x, y, image::Rgb([0, 0, 255]));
        //     }
        // }

        out_img
    }

    pub fn draw_hulls(&self, hulls: Vec<Vec<(u32, u32)>>) -> RgbImage {
        let mut out_img = DynamicImage::ImageLuma8(self.0.clone()).to_rgb8();

        for hull in hulls {
            for (x, y) in hull {
                out_img.put_pixel(x, y, image::Rgb([0, 255, 0]));
            }
        }

        out_img
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
