use image::GrayImage;

pub trait ThresholdExt {
    fn otsu(&self) -> Option<u8>;
}

impl ThresholdExt for GrayImage {
    fn otsu(&self) -> Option<u8> {
        let abs_hist = !todo!();
        let img_size = (self.dimensions().0 * self.dimensions().1) as f32;
        let mut t_star: Option<u8> = None;
        let mut var_max = -1f32;
        let mut c0: f32 = 0f32;
        let mut sum0: f32 = 0f32;
        let weight_sum = weighted_sum(abs_hist);
        for i in 0..256 {
            c0 += abs_hist[i];
            // can c1 be zero??
            let c1 = img_size - c0;
            if c1 == 0. {
                continue;
            }

            sum0 += i as f32 * abs_hist[i];
            let mu0 = sum0 / c0;
            let mu1 = (weight_sum - sum0) / c1;
            let var_between = c0 * c1 * (mu0 - mu1).powi(2);
            if var_max < var_between {
                var_max = var_between;
                t_star = Some(i as u8);
            }
        }

        t_star
    }
}

fn weighted_sum(norm_hist: &Vec<f32>) -> f32 {
    let mut sum = 0f32;
    for i in 0..norm_hist.len() {
        sum += i as f32 * norm_hist[i];
    }
    sum
}
