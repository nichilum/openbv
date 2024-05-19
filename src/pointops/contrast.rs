use rayon::iter::ParallelIterator;

pub trait ContrastExt {
    fn contrast_stretch(&mut self, t_0: u8, t_1: u8);
}

impl ContrastExt for image::GrayImage {
    /// Applies contrast stretching with the given thresholds.
    fn contrast_stretch(&mut self, t_0: u8, t_1: u8) {
        assert!(t_0 < t_1);

        self.par_pixels_mut().for_each(|pixel| {
            let value = if pixel[0] <= t_0 {
                0
            } else {
                (pixel[0] - t_0).saturating_mul(255 / (t_1 - t_0))
            };

            *pixel = image::Luma([value]);
        });
    }
}
