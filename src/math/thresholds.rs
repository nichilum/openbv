use image::GrayImage;

use super::histogram::HistogramFromImageExt;

pub trait ThresholdExt {
    fn otsu(&self) -> Option<u8>;
}

impl ThresholdExt for GrayImage {
    fn otsu(&self) -> Option<u8> {
        let abs_hist = self.histogram().0;
        let img_size = (self.dimensions().0 * self.dimensions().1) as i32;
        let mut t_star: Option<u8> = None;
        let mut var_max = -1i32;
        let mut c0 = 0i32;
        let mut sum0 = 0i32;
        let weight_sum = weighted_sum(&abs_hist) as i32;
        for i in abs_hist {
            c0 += i as i32;
            // can c1 be zero??
            let c1 = img_size - c0;
            if c1 == 0 {
                continue;
            }

            sum0 += i as i32 * i as i32;
            let mu0 = sum0 / c0;
            let mu1 = (weight_sum - sum0) / c1;
            let var_between = c0 * c1 * (mu0 - mu1).pow(2);
            if var_max < var_between {
                var_max = var_between;
                t_star = Some(i as u8);
            }
        }

        t_star
    }
}

fn weighted_sum(hist: &[usize]) -> usize {
    let mut sum = 0;
    for (i, val) in hist.iter().enumerate() {
        sum += i * val;
    }
    sum
}
