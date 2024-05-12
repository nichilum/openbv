use image::GrayImage;
use rayon::iter::ParallelIterator;

pub trait BinarizeExt {
    fn binarize(&self, threshold: u8) -> BinaryImage;
}

pub struct BinaryImage(pub GrayImage);

impl Into<GrayImage> for BinaryImage {
    fn into(self) -> GrayImage {
        self.0
    }
}

impl Into<BinaryImage> for GrayImage {
    // TODO: check if self is an actual binary image
    fn into(self) -> BinaryImage {
        BinaryImage(self)
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
