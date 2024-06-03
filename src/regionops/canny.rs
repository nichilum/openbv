use image::{GenericImageView, GrayImage, Luma};
use rayon::iter::ParallelIterator;

pub trait CannyExt {
    fn canny(&self) -> GrayImage;
}

impl CannyExt for GrayImage {
    fn canny(&self) -> GrayImage {
        canny_gauss_filter(self)
    }
}

fn canny_gauss_filter(base_image: &GrayImage) -> GrayImage {
    let norm_fac = 1f32 / 159f32;
    #[rustfmt::skip]
    let kernel = [
        [2, 4, 5, 4, 2].as_slice(), 
        [4, 9, 12, 9, 4].as_slice(), 
        [5, 12, 15, 12, 5].as_slice(), 
        [4, 9, 12, 9, 4].as_slice(), 
        [2, 4, 5, 4, 2].as_slice()
    ];

    convolve(base_image, &kernel[..], norm_fac)
}

/// returns (Gx, Gy)
fn canny_sobel(smooth_image: &GrayImage) -> (GrayImage, GrayImage) {
    let (width, height) = smooth_image.dimensions();

    let mut gx = convolve(smooth_image, &[&[1, 0, -1], &[2, 0, -2], &[1, 0, -1]], 1.);
    let mut gy = smooth_image.clone();

    (gx, gy)
}

fn convolve(base_image: &GrayImage, kernel: &[&[i32]], norm_fac: f32) -> GrayImage {
    let mut new_image = base_image.clone();

    let (width, height) = base_image.dimensions();

    let half_filter_width_h = kernel[0].len() as i32 / 2;
    let half_filter_width_v = kernel.len() as i32 / 2;

    new_image
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| unsafe {
            let mut sum: u32 = 0;

            for dx in -half_filter_width_h..=half_filter_width_h {
                for dy in -half_filter_width_v..=half_filter_width_v {
                    sum += kernel[(dy + 2) as usize][(dx + 2) as usize]
                        * base_image.unsafe_get_pixel(
                            if dx.is_negative() {
                                x.saturating_sub(dx.abs() as u32)
                            } else {
                                if x + dx as u32 >= width {
                                    width - 1
                                } else {
                                    x + dx as u32
                                }
                            },
                            if dy.is_negative() {
                                y.saturating_sub(dy.abs() as u32)
                            } else {
                                if y + dy as u32 >= height {
                                    height - 1
                                } else {
                                    y + dy as u32
                                }
                            },
                        )[0] as u32;
                }
            }

            *pixel = Luma([(norm_fac * sum as f32) as u8]);
        });

    new_image
}
