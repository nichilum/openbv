use image::{ImageBuffer, Rgb, RgbImage};

/// scuffed lol
pub type HsvImage = ImageBuffer<Rgb<f32>, Vec<f32>>;

pub trait HsvExt {
    fn to_hsv(&self) -> HsvImage;
}

impl HsvExt for RgbImage {
    fn to_hsv(&self) -> HsvImage {
        
    }
}
