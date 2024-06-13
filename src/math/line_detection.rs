use image::{DynamicImage, GrayImage, RgbImage};

use crate::binary_image::draw_line;

#[derive(Debug)]
pub struct Line {
    /// Distance from the origin
    pub r: f32,

    /// Angle in radians
    pub theta: u32,
}

pub fn hough(image: &GrayImage, r_step: f32, theta_step: f32, threshold: u32) -> Vec<Line> {
    let (width, height) = image.dimensions();
    let max_r = ((width.pow(2) + height.pow(2)) as f32).sqrt();
    let max_theta = std::f32::consts::PI;

    todo!();
}

pub fn draw_lines(image: &GrayImage, lines: &[Line]) -> RgbImage {
    let mut out_img = DynamicImage::ImageLuma8(image.clone()).to_rgb8();

    for line in lines {
        let r = line.r;
        let theta = line.theta as f32;
        let x1 = 0;
        let y1 = (r / theta.cos()) as u32;
        let x2 = image.width();
        let y2 = ((r - x2 as f32 * theta.cos()) / theta.sin()) as u32;

        draw_line(&mut out_img, x1, y1, x2, y2, image::Rgb([255, 0, 0]));
    }

    out_img
}
