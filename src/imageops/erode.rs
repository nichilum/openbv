use rayon::iter::ParallelIterator;

use super::binarize::BinaryImage;

pub trait ErodeExt {
    fn erode(&self, kernel: &[(i32, i32)]) -> BinaryImage;
}

impl ErodeExt for BinaryImage {
    // TODO: inplace
    fn erode(&self, kernel: &[(i32, i32)]) -> BinaryImage {
        let mut new_img = self.0.clone();

        new_img
            .par_enumerate_pixels_mut()
            .for_each(|(x, y, pixel)| {
                let mut value: usize = 0;
                for (x_filter, y_filter) in kernel {
                    let index_x = x as i32 + x_filter;
                    let index_y = y as i32 + y_filter;

                    if index_x >= 0
                        && index_x < self.0.dimensions().0 as i32
                        && index_y >= 0
                        && index_y < self.0.dimensions().1 as i32
                    {
                        value += self.0.get_pixel(index_x as u32, index_y as u32).0[0] as usize;
                    }
                }

                if value == kernel.len() * 255 {
                    pixel.0[0] = 255;
                } else {
                    pixel.0[0] = 0;
                }
            });

            new_img.into()
    }
}
