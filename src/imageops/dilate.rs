use rayon::iter::ParallelIterator;

use super::binarize::BinaryImage;

pub trait DilateExt {
    fn dilate(&self, kernel: &[(i32, i32)], steps: usize) -> BinaryImage;
}

impl DilateExt for BinaryImage {
    fn dilate(&self, kernel: &[(i32, i32)], steps: usize) -> BinaryImage {
        let mut dilated_img = dilate(self, kernel);
        for _ in 0..steps - 1 {
            dilated_img = dilate(&dilated_img, kernel);
        }
        dilated_img
    }
}

fn dilate(ref_image: &BinaryImage, kernel: &[(i32, i32)]) -> BinaryImage {
    let mut result_img = ref_image.0.clone();

    result_img
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            for (x_filter, y_filter) in kernel {
                let index_x = x as i32 - x_filter;
                let index_y = y as i32 - y_filter;

                if index_x >= 0
                    && index_x < ref_image.0.dimensions().0 as i32
                    && index_y >= 0
                    && index_y < ref_image.0.dimensions().1 as i32
                    && ref_image.0.get_pixel(index_x as u32, index_y as u32).0[0] == 255
                {
                    pixel.0[0] = 255;
                    break;
                }
            }
        });

    result_img.into()
}
