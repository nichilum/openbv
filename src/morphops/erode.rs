use rayon::iter::ParallelIterator;

use crate::binary_image::BinaryImage;

pub trait ErodeExt {
    fn erode(&self, kernel: &[(i32, i32)], steps: usize) -> BinaryImage;
}

impl ErodeExt for BinaryImage {
    // TODO: inplace
    fn erode(&self, kernel: &[(i32, i32)], steps: usize) -> BinaryImage {
        if steps == 0 {
            return self.0.clone().into();
        }

        let mut eroded_img = erode(self, kernel);
        for _ in 0..steps - 1 {
            eroded_img = erode(&eroded_img, kernel);
        }
        eroded_img
    }
}

fn erode(ref_image: &BinaryImage, kernel: &[(i32, i32)]) -> BinaryImage {
    let mut result_img = ref_image.0.clone();

    result_img
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            let mut value: usize = 0;
            for (x_filter, y_filter) in kernel {
                let index_x = x as i32 + x_filter;
                let index_y = y as i32 + y_filter;

                if index_x >= 0
                    && index_x < ref_image.0.dimensions().0 as i32
                    && index_y >= 0
                    && index_y < ref_image.0.dimensions().1 as i32
                {
                    value += ref_image.0.get_pixel(index_x as u32, index_y as u32).0[0] as usize;
                }
            }

            if value == kernel.len() * 255 {
                pixel.0[0] = 255;
            } else {
                pixel.0[0] = 0;
            }
        });

    result_img.into()
}
