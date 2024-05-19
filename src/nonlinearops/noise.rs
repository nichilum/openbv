use image::{GrayImage, Luma};
use rand::Rng;
use rayon::iter::ParallelIterator;

pub trait NoiseExt {
    fn salt_and_pepper(&mut self, noise_amount: f64);
}

impl NoiseExt for GrayImage {
    fn salt_and_pepper(&mut self, noise_amount: f64) {
        self.par_pixels_mut().for_each(|p| {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(noise_amount) {
                let lum = if rng.gen_bool(0.5) { 255 } else { 0 };

                *p = Luma([lum]);
            }
        });
    }
}
