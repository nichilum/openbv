use rayon::iter::ParallelIterator;

pub trait ContrastExt {
    fn contrast_stretch(&mut self, t_0: u8, t_1: u8);
}

impl ContrastExt for image::GrayImage {
    /// Applies contrast stretching with the given thresholds.
    fn contrast_stretch(&mut self, t_0: u8, t_1: u8) {
        assert!(t_0 < t_1);

        self.par_pixels_mut().for_each(|pixel| {
            let value = lin_con(pixel[0], t_1, t_0);
            *pixel = image::Luma([value]);
        });
    }
}

fn lin_con(a: u8, t_1: u8, t_0: u8) -> u8 {
    if a <= t_0 {
        0
    } else if t_0 < a && a < t_1 {
        (255 / (t_1 - t_0)) * (a - t_0)
    } else {
        255
    }
}
