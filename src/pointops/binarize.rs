use image::GrayImage;
use rayon::iter::ParallelIterator;

use crate::binary_image::BinaryImage;

pub trait BinarizeExt {
    fn binarize(&self, threshold: u8) -> BinaryImage;
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
