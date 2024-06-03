use image::{GenericImageView, GrayImage, Luma};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub trait CannyExt {
    fn canny(&self) -> GrayImage;
}

impl CannyExt for GrayImage {
    fn canny(&self) -> GrayImage {
        let smoothed_image = canny_gauss_filter(self);
        let (gx, gy) = canny_sobel(&smoothed_image);

        let mut edge_image = GrayImage::new(self.width(), self.height());
        edge_image.par_enumerate_pixels_mut().for_each(
            |(x, y, pixel)| {
                let index = (y * self.width() + x) as usize;
                let gx_val = gx.data[index];
                let gy_val = gy.data[index];

                let val = ((gx_val.pow(2) + gy_val.pow(2)) as f32).sqrt() as u8;
                *pixel = Luma([val]);
            },
        );

        let mut direction_image = GrayImage::new(self.width(), self.height());
        direction_image.par_enumerate_pixels_mut().for_each(
            |(x, y, pixel)| {
                let index = (y * self.width() + x) as usize;
                let gx_val = gx.data[index] as f64;
                let gy_val = gy.data[index] as f64;

                let mut val = gy_val.atan2(gx_val);
                if val < 0.0 {
                    val += std::f64::consts::PI;
                }
                
                match val.to_degrees() {
                    0.0..=22.5 | 157.5..=180.0 => *pixel = Luma([0]),
                    22.5..=67.5 => *pixel = Luma([45]),
                    67.5..=112.5 => *pixel = Luma([90]),
                    112.5..=157.5 => *pixel = Luma([135]),
                    _ => unreachable!(),
                }
            },
        );
        
        direction_image
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

pub struct EdgeImage {
    pub data: Vec<i32>,
    pub width: u32,
    pub height: u32,
}

impl EdgeImage {
    pub fn to_gray_image(&self) -> GrayImage {
        let mut new_image = GrayImage::new(self.width, self.height);

        new_image
            .par_enumerate_pixels_mut()
            .for_each(|(x, y, pixel)| {
                let index = (y * self.width + x) as usize;
                let val = self.data[index].abs() as u8;
                *pixel = Luma([val]);
            });

        new_image
    }
}

/// returns (Gx, Gy)
fn canny_sobel(smooth_image: &GrayImage) -> (EdgeImage, EdgeImage) {
    let gx = canny_convolve(smooth_image, &[&[1, 0, -1], &[2, 0, -2], &[1, 0, -1]], 1.);
    let gy = canny_convolve(smooth_image, &[&[1, 2, 1], &[0, 0, 0], &[-1, -2, -1]], 1.);

    (gx, gy)
}

/// same as "normal" convolve, but can handle negative kernel values and returns a `EdgeImage`
fn canny_convolve(base_image: &GrayImage, kernel: &[&[i32]], norm_fac: f32) -> EdgeImage {
    let (width, height) = base_image.dimensions();
    let mut new_image = EdgeImage {
        data: vec![0; (base_image.width() * base_image.height()) as usize],
        width,
        height,
    };

    let half_filter_width_h = kernel[0].len() as i32 / 2;
    let half_filter_width_v = kernel.len() as i32 / 2;

    new_image
    .data
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| unsafe {
            let x = index as u32 % width;
            let y = index as u32 / width;

            let mut sum = 0;

            for dx in -half_filter_width_h..=half_filter_width_h {
                for dy in -half_filter_width_v..=half_filter_width_v {
                    sum += kernel[(dy + half_filter_width_v) as usize][(dx + half_filter_width_h) as usize]
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
                        )[0] as i32;
                }
            }

            *pixel = (norm_fac * sum as f32 ) as i32;
        });

    new_image
}

fn convolve(base_image: &GrayImage, kernel: &[&[u32]], norm_fac: f32) -> GrayImage {
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
