use image::{ImageBuffer, Rgb, RgbImage};

use rayon::iter::ParallelIterator;

/// scuffed lol
pub type HsvImage = ImageBuffer<Rgb<f32>, Vec<f32>>;

pub trait HsvExt {
    fn to_hsv(&self) -> HsvImage;
}

impl HsvExt for RgbImage {
    fn to_hsv(&self) -> HsvImage {
        let mut hsv_image = HsvImage::new(self.width(), self.height());

        hsv_image
            .par_enumerate_pixels_mut()
            .for_each(|(x, y, pixel)| {
                let base_pixel = self.get_pixel(x, y);
                let r = base_pixel[0] as f32 / 255.;
                let g = base_pixel[1] as f32 / 255.;
                let b = base_pixel[2] as f32 / 255.;

                let max = r.max(g).max(b);
                let min = r.min(g).min(b);

                let mut h = if max == min {
                    0.
                } else if max == r {
                    60. * (0. + (g - b) / (max - min))
                } else if max == g {
                    60. * (2. + (b - r) / (max - min))
                } else if max == b {
                    60. * (4. + (r - g) / (max - min))
                } else {
                    unreachable!()
                };

                if h < 0. {
                    h += 360.
                }

                let s = if max == min { 0. } else { (max - min) / (max) };

                let v = max;

                *pixel = Rgb([h, s, v]);
            });

        hsv_image
    }
}
