use image::{GrayImage, ImageResult, RgbImage};
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

    pub fn draw_contours(&self, contours: Vec<Contour>) -> RgbImage {
        todo!()
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
